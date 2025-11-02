/*
TODO:
* add devices, so a focusable entity can be focused on by multiple devices ?
- but will have to remove focusable.focused ? or replace with hashset<device>
- also should have flag on focusable, to default focus on it (regardless of device)?


* when node is focused on, have it scroll to fit it in if it is partially/not on screen
- when focusing left, scroll enough to fit left side of node at 0, when right, make right side of node fit within, etc

* able to have sub focus where parent focus isn't an ancestor, eg focus on button, press it, sub focus on panel next to it

* when scrolling
- if focus goes off screen, could move focus
- or if no focus, init a focus from the back side

* when entity (top or nested) gets moved and is in nested focus,
** should refocus new focusable ancestors?
** or just unfocus moved entity? and set prev focusable ancestor as top focus?
*/

use std::cmp::Ordering;
use std::collections::HashMap;

use bevy::ecs::prelude::*;
// use bevy::hierarchy::prelude::*;


use crate::interact::vals::FocusMove;

use super::super::components::*;
// use super::super::resources::*;
use super::super::messages::*;
// use super::super::utils::*;
// use super::super::values::*;

use super::super::super::layout::components::{UiLayoutComputed, UiFloat,UiRoot};
// use crate::table_ui::{UiComputed, UiFloat};


/*
for focus, visiteds, could add how many moves ago it was visited, smallest ago is prioritised
also switching between h/v doesn't erase the history
instead of hashset<entity>, hashmap<entity,last_visited>
will need to remove dead entities from it though
also keep a count to use for adding visit time


*/


// #[derive(PartialEq,Debug)] //,Default
// pub enum FocusMoveType {
//     Horizontal,Vertical,
//     // #[default]
//     Tab
// }

// fn float_order_rev(x_entity:Entity,y_entity:Entity,x_computed:&UiComputed,y_computed:&UiComputed,) {

//     let q=if q == Ordering::Equal {
//         let x_float=float_query.get(x.0).map(|f|f.float).unwrap_or_default();
//         let y_float=float_query.get(y.0).map(|f|f.float).unwrap_or_default();

//         if !x_float && y_float {
//             Ordering::Greater
//         } else if x_float && !y_float {
//             Ordering::Less
//         } else if x_float && y_float {
//             x_computed.order.cmp(&y_computed.order)
//         } else {
//             q
//         }
//     } else {
//         q
//     };
// }
pub fn update_focus_events(

    computed_query: Query<&UiLayoutComputed,With<UiLayoutComputed>>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    children_query: Query<&Children,(With<UiLayoutComputed>,)>,
    // focus_query : Query<Entity, (With<UiFocusable>,)>,
    mut focusable_query : Query<&mut UiFocusable>,

    mut input_event_reader: MessageReader<UiInteractInputMessage>,
    mut ui_event_writer: MessageWriter<UiInteractEvent>,

    mut focus_states:Local<HashMap<i32,HashMap<Entity,HashMap<i32,(Option<Entity>,Vec<Entity>,

        // HashMap<Entity,u64>, //replace with left/right/top/bottom eg [Option<Entity>;4] or [(Option<Entity>,u64);4]
        // [(Option<Entity>,u64);4]
        // i32 //the hist!
        // [Option<Entity>;4] //left,top,right,bottom

    )>>>>, //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk,hist)

    mut move_hists:Local<HashMap<i32,HashMap<Entity,[Entity;4]>>>, //[device][entity]=(left,top,right,bottom)
    //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk,from_dirs)

    // mut hist_incr : Local<u64>,
) {
    //
    move_hists.retain(|_device,device_move_hists|{
        device_move_hists.retain(|&entity,dirs|{
            for entity2 in dirs {
                if *entity2!=Entity::PLACEHOLDER && !computed_query.contains(*entity2) {
                    *entity2=Entity::PLACEHOLDER;
                }
            }

            computed_query.contains(entity)
        });

        !device_move_hists.is_empty()
    });

    //in cur_focus_entity, focus_entity_stk, unfocus removed entities, disabled/invisible entities, disabled focusables, focusables changed groups
    for (&device,device_focus_states) in focus_states.iter_mut() {
        for (&root_entity,groups) in device_focus_states.iter_mut() {
            let root_entity_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();

            for (&cur_group,(cur_focus_entity,focus_entity_stk)) in groups.iter_mut() {
                 //necessary? better to check when using it?
                // hist.retain(|&entity,_|{  computed_query.contains(entity) }); //the hist!

                // for h in hist {
                //     if h.map(|entity|!computed_query.contains(entity)).unwrap_or_default() {
                //         *h=None;
                //     }
                // }

                //remove from cur_focus/focus_stk any invisible, changed group, no longer focusable or focusable.enabled
                //  should compare against ancestors, so can check if cur entity or ancestor entities have been moved
                for i in 0..focus_entity_stk.len() {
                    let entity=focus_entity_stk[i];
                    let unlocked = computed_query.get(entity).map(|x|x.unlocked).unwrap_or_default();
                    let focusable=focusable_query.get(entity).ok();
                    // let is_focused=device_focuseds.as_ref().map(|device_focuseds|device_focuseds.contains(&entity)).unwrap_or_default(); //hmph
                    let focusable_enabled = focusable.map(|x|x.enable).unwrap_or_default();// && is_focused;
                    let node_focus_group = focusable.map(|x|x.group).unwrap_or_default();

                    if !root_entity_alive || !focusable_enabled || !unlocked || node_focus_group!=cur_group {
                        //should not unfocus if invisible under some situations?

                        //also end this
                        if let Some(entity)=*cur_focus_entity {
                            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device }});
                            *cur_focus_entity=None;

                            // focusable.focused=false; //focus bool removed
                            // device_focuseds.and_then(|x|x.remove(&entity).then_some(())).unwrap(); //hmph
                        }

                        //
                        for j in (i..focus_entity_stk.len()).rev() {
                            let entity=focus_entity_stk[j];
                            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device }});

                            // focusable.focused=false; //focus bool removed
                            // device_focuseds.and_then(|x|x.remove(&entity).then_some(())).unwrap(); //hmph
                        }

                        //
                        focus_entity_stk.truncate(i);
                        break;
                    }
                }

                //remove from cur_focus disabled/invisible, changed group, no longer focusable
                if let Some(entity)=*cur_focus_entity {
                    let unlocked = computed_query.get(entity).map(|x|x.unlocked).unwrap_or_default();
                    let focusable = focusable_query.get_mut(entity).ok();
                    // let is_focused=device_focuseds.map(|device_focuseds|device_focuseds.contains(&entity)).unwrap_or_default(); //hmph
                    let focusable_enabled=focusable.as_ref().map(|x|x.enable).unwrap_or_default();// && is_focused;
                    let node_focus_group = focusable.as_ref().map(|x|x.group).unwrap_or_default();

                    if !root_entity_alive || !focusable_enabled || !unlocked || node_focus_group != cur_group {
                        ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device }});
                        *cur_focus_entity=None;

                        // focusable.focused=false; //focus bool removed
                        // device_focuseds.and_then(|x|x.remove(&entity).then_some(())).unwrap(); //hmph
                    }
                }
            }
        }
    }

    //remove dead devices, roots
    focus_states.retain(|&_device,device_focus_states|{
        device_focus_states.retain(|&root_entity,_root_focus_states|{
            let root_entity_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();
            root_entity_alive
        });

        !device_focus_states.is_empty()
    });


    // //get list of focusable entities
    // let mut device_focus_group_focuses:HashMap<i32,HashMap<(Entity,i32),(Entity,u32)>>=HashMap::new(); //[device][(root_entity,focus_group)]=(entity,order)
    // //root_focus_group_focuses
    // //create set of focusable entities?
    // for entity in focus_query.iter() {
    //     let computed=computed_query.get(entity).unwrap();
    //     let focusable=focusable_query.get(entity).unwrap();

    //     if !computed.unlocked || !focusable.enable //|| focusable_entity_visiteds.contains(&entity)
    //     {
    //         continue;
    //     }

    //     // let root_entity = parent_query.iter_ancestors(entity).last().unwrap_or(entity);
    //     // let Some(root_entity)=[entity].into_iter().chain(parent_query.iter_ancestors(entity)).find(|&ancestor_entity|root_query.contains(ancestor_entity)) else {
    //     //     continue;
    //     // };

    //     for (&device,device_focus_states) in focus_states.iter_mut() {
    //         let ( _cur_focus_entity, _focus_entity_stk, hist)=device_focus_states
    //             .entry(computed.root_entity).or_default()
    //             .entry(focusable.group).or_default()
    //             ;


    //         // focusable_entity_roots.insert(entity);
    //         let root_focus_group_focuses=device_focus_group_focuses.entry(device).or_default();

    //         let prev=root_focus_group_focuses.entry((computed.root_entity,focusable.group)).or_insert((entity,computed.order));
    //         let cur_hist=hist.get(&entity).cloned().unwrap_or_default();
    //         let prev_hist=hist.get(&prev.0).cloned().unwrap_or_default();

    //         if cur_hist>prev_hist || ((cur_hist==prev_hist)&&computed.order < prev.1 )
    //         {
    //             prev.0=entity;
    //             prev.1=computed.order;
    //         }
    //     }


    // }




    //

    let mut ev_stk= input_event_reader.read().cloned().collect::<Vec<_>>();
    ev_stk.reverse();



    let mut was_resent=false; //not sure if need? to stop infinite loop if no focusable is there

    //
    while let Some(ev)=ev_stk.pop() {

        // if !root_query.get(ev.get_root_entity()).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
        //     continue;
        // }

        if let UiInteractInputMessage::FocusOn { entity, device }=ev {
            //todo
        } else if !ev.root_entity()
            .and_then(|root_entity|root_query.get(root_entity).ok())
            .map(|(_,computed)|computed.unlocked)
            .unwrap_or_default()
        {
            continue;
        }

        match ev.clone() {
            UiInteractInputMessage::FocusEnter{root_entity,group, device } => {
                let device_focus_states=focus_states.entry(device).or_default();

                if let Some(groups)=device_focus_states.get_mut(&root_entity) {
                    if let Some((cur_focus_entity,focus_entity_stk, ))=groups.get_mut(&group) {
                        if let Some(entity)=*cur_focus_entity {
                            focus_entity_stk.push(entity);
                            // prev_focused_stk.push(Default::default());
                            *cur_focus_entity=None;
                        }
                    }
                }
            }
            UiInteractInputMessage::FocusExit{root_entity,group, device } => {
                let device_focus_states=focus_states.entry(device).or_default();

                if let Some(groups)=device_focus_states.get_mut(&root_entity) {
                    if let Some((cur_focus_entity,focus_entity_stk, ))=groups.get_mut(&group) {
                        //already checked above for enabled/unlocked
                        if let Some(entity)=*cur_focus_entity {
                            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group, device }});

                            *cur_focus_entity=focus_entity_stk.pop();

                            // if cur_focus_entity.is_some() {
                            //     prev_focused_stk.pop().unwrap();
                            // }

                            //
                            // if let Ok(mut focusable)=focusable_query.get_mut(entity)
                            if focusable_query.contains(entity)
                            {
                                // focusable.focused=false;

                                // let device_focuseds=focuseds.0.get_mut(&device).unwrap(); //should exist? //hmph
                                // device_focuseds.remove(&entity).then_some(()).unwrap(); //hmph
                            }
                        }
                    }
                }

                //should also exit all the way if only single focusable all the way?
                continue;
            }
            _ => {}
        }

        //
        let Some(top_root_entity)= ev.root_entity() else {continue;};
        let Some(cur_group)= ev.focus_group() else {continue;};
        let cur_device=ev.device();

        //,
        let (cur_focus_entity,focus_entity_stk, )=focus_states.entry(cur_device).or_default()
            // device_focus_states
            .entry(top_root_entity).or_default().entry(cur_group).or_default();

        //
        if let UiInteractInputMessage::FocusPrev{..}|UiInteractInputMessage::FocusLeft{..}|UiInteractInputMessage::FocusUp{..}=ev {
            if cur_focus_entity.is_none() {
                if !was_resent {
                    ev_stk.push(ev.clone());
                    was_resent=true;
                } else {
                    was_resent=false;
                }
            }
        }

        let Some(move_dir)=get_focus_move_dir(&ev,cur_focus_entity.is_none()) else {
            continue;
        };

        let device_move_hists=move_hists.entry(cur_device).or_default();

        // let x: Query<&ChildOf, With<UiLayoutComputed>>=parent_query;
        move_focus(
            move_dir,
            top_root_entity,
            cur_group,
            cur_device,
            cur_focus_entity,
            focus_entity_stk,
            // hist, //the hist!
            // &mut hist_incr, //the hist!
            parent_query,
            computed_query,
            children_query,
            float_query,
            &focusable_query,
            // &mut focuseds,
            &mut ui_event_writer,
            device_move_hists,
        );

        //
        if let UiInteractInputMessage::FocusEnter{..}=ev { //if no focus found, undo focus_entity_push above
            if cur_focus_entity.is_none() {//&& focus_entity_stk.len()>1
                *cur_focus_entity=focus_entity_stk.pop();

                // if cur_focus_entity.is_some() {
                //     prev_focused_stk.pop().unwrap();
                // }
            }
        }
    }
}

fn get_focus_move_dir(ev : &UiInteractInputMessage, b:bool) -> Option<FocusMove> {
    match ev {
        UiInteractInputMessage::FocusInit{..}
        |UiInteractInputMessage::FocusEnter{..}

        |UiInteractInputMessage::FocusPrev{..}
        |UiInteractInputMessage::FocusNext{..}

        |UiInteractInputMessage::FocusLeft{..}
        |UiInteractInputMessage::FocusRight{..}

        |UiInteractInputMessage::FocusUp{..}
        |UiInteractInputMessage::FocusDown{..}
            if b => Some(FocusMove::Next),


        UiInteractInputMessage::FocusPrev{..} => Some(FocusMove::Prev),
        UiInteractInputMessage::FocusNext{..} => Some(FocusMove::Next),

        UiInteractInputMessage::FocusLeft{..} => Some(FocusMove::Left),
        UiInteractInputMessage::FocusRight{..} => Some(FocusMove::Right),

        UiInteractInputMessage::FocusUp{..} => Some(FocusMove::Up),
        UiInteractInputMessage::FocusDown{..} => Some(FocusMove::Down),

        _ => None,
    }
}

fn move_focus(
    move_dir: FocusMove,
    top_root_entity:Entity,
    cur_group:i32,
    cur_device:i32,
    cur_focus_entity:&mut Option<Entity>,
    focus_entity_stk:&mut Vec<Entity>,
    // hist:  &mut HashMap<Entity, u64>, //the hist!
    // hist_incr:&mut u64, //the hist!
    parent_query:Query<&ChildOf, With<UiLayoutComputed>>,

    layout_computed_query: Query<&UiLayoutComputed,With<UiLayoutComputed>>,
    children_query: Query<&Children,(With<UiLayoutComputed>,)>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    focusable_query : &Query<&mut UiFocusable>,
    // focuseds : &mut UiFocuseds, //hmph
    ui_event_writer: &mut MessageWriter<UiInteractEvent>,
    device_move_hists : &mut HashMap<Entity,[Entity;4]>, //[device][entiti]=(left,top,right,bottom)
) {

    //on nofocus(init) then up/back, don't send focus_begin/end for the init dif from up/back focus entity

    //with move_hists, if moving right, and reaching end, want it to wrap to first item in cur row's move_hist.left
    //  could look through list of the candidates, and search for the first candidate.move_hist.left

    let move_hori = move_dir.horizontal();
    let move_vert = move_dir.vertical();
    let move_tab = move_dir.tab();
    let move_pos = move_dir.positive();

    struct FocusMoveWork {
        entity:Entity,
        from_bounds:Vec<(u32,u32)>,
        to_bound:(u32,u32),
        // to_start:u32,
        // to_len:u32,
        focus_depth:usize,
        valid:bool,
    }
    //
    // let mut stk: Vec<(Entity,Vec<(u32,u32)>,(u32,u32),usize,bool)>=Vec::new(); //[]=(entity,from_bounds,(to_start,to_len),focus_depth,valid)
    let mut stk: Vec<FocusMoveWork>=Vec::new();

    //init stk
    //get all uncles, great uncles, great great uncles etc
    //  that are in same row/col (for hori/vert) or all if using order (for prev/enxt)

    if let Some(cur_focus_entity)=*cur_focus_entity {
        //calculated from curfocus+focus_stk, used on "to" nodes,
        //[depth_len-depth-1]=(focus_nodes[depth].col,.focus_nodes[depth].parent.cols)
        //    eg [(cur_focus.col,parent.cols),(parent.col,gparent.cols),(gparent.col,ggparent.cols) ]
        let mut from_bounds = Vec::new();

        //
        //past is for going past the edge and wrapping, forget why its split into befores/afters
        //befores is for ones behind ie in the opposite move dir, think also used in wrapping?

        //
        // let mut stk_befores: Vec<(Entity, Vec<(u32, u32)>,(u32,u32), usize,bool)> = Vec::new(); //[]=(entity,from_bounds,to_range,focus_depth,valid)
        // let mut stk_past_befores: Vec<(Entity, Vec<(u32, u32)>,(u32,u32), usize,bool)> = Vec::new();
        // let mut stk_past_afters: Vec<(Entity, Vec<(u32, u32)>,(u32,u32), usize,bool)> = Vec::new();

        let mut stk_befores: Vec<FocusMoveWork> = Vec::new();
        let mut stk_past_befores: Vec<FocusMoveWork> = Vec::new();
        let mut stk_past_afters: Vec<FocusMoveWork> = Vec::new();

        //
        let mut focus_depth = 0;

        //loop thru cur focused entity and its ancestors
        for cur_entity in [cur_focus_entity].into_iter().chain(parent_query.iter_ancestors(cur_focus_entity)) {
            let Ok(parent_entity) = parent_query.get(cur_entity).map(|p|p.parent()) else {
                break; //only loop entities with parent
            };

            let Ok(parent_children) = children_query.get(parent_entity) else {
                continue;
            };

            //
            let cur_computed = layout_computed_query.get(cur_entity).unwrap();
            let parent_computed = layout_computed_query.get(parent_entity).unwrap();

            //
            let stk_len = stk.len();
            let stk_befores_len = stk_befores.len();
            let stk_past_befores_len = stk_past_befores.len();
            let stk_past_afters_len = stk_past_afters.len();

            //
            for child_entity in parent_children.iter() {
                let Ok(child_computed)=layout_computed_query.get(child_entity) else {
                    continue;
                };

                if child_entity == cur_entity || (move_vert && child_computed.col != cur_computed.col) || (move_hori && child_computed.row != cur_computed.row) {
                    continue;
                }

                let is_after=match move_dir {
                    FocusMove::Down => child_computed.row > cur_computed.row,
                    FocusMove::Right => child_computed.col > cur_computed.col,
                    FocusMove::Up => child_computed.row < cur_computed.row,
                    FocusMove::Left => child_computed.col < cur_computed.col,
                    FocusMove::Next => child_computed.order > cur_computed.order,
                    FocusMove::Prev => child_computed.order < cur_computed.order,
                };

                let to_len=match move_dir {
                    FocusMove::Up|FocusMove::Down => child_computed.cols,
                    FocusMove::Left|FocusMove::Right => child_computed.rows,
                    FocusMove::Prev|FocusMove::Next => 0,
                };

                let to_bound=(0,to_len);

                //
                if is_after {
                    if focus_depth>0 {
                        // stk_past_afters.push((child_entity,from_bounds.clone(),to_bound,focus_depth,true));
                        stk_past_afters.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound, focus_depth, valid: true });
                    } else {
                        // stk.push((child_entity,from_bounds.clone(),to_bound,0,true));
                        stk.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound, focus_depth: 0, valid: true })
                    }
                } else {
                    if focus_depth>0 {
                        // stk_past_befores.push((child_entity,from_bounds.clone(),to_bound,focus_depth,true));
                        stk_past_befores.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound, focus_depth, valid: true });
                    } else {
                        // stk_befores.push((child_entity,from_bounds.clone(),to_bound,0,true));
                        stk_befores.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound, focus_depth: 0, valid: true });
                    }
                }
            }

            //sort backwards?
            let sort_func= &Box::new(|x:&FocusMoveWork,y:&FocusMoveWork|{
                let x_computed=layout_computed_query.get(x.entity).unwrap();
                let y_computed=layout_computed_query.get(y.entity).unwrap();

                //
                let q = if move_tab {
                    x_computed.order.cmp(&y_computed.order)
                } else if move_vert {
                    x_computed.row.cmp(&y_computed.row)
                } else { //move_hori
                    x_computed.col.cmp(&y_computed.col)
                };

                //
                let q=if q == Ordering::Equal {
                    let x_float=float_query.get(x.entity).map(|f|f.float).unwrap_or_default();
                    let y_float=float_query.get(y.entity).map(|f|f.float).unwrap_or_default();

                    if !x_float && !y_float {
                        x_computed.order.cmp(&y_computed.order)
                    } else if !x_float && y_float {
                        Ordering::Greater
                    } else if x_float && !y_float {
                        Ordering::Less
                    } else { //both floating
                        x_computed.order.cmp(&y_computed.order)
                    }
                } else {
                    q
                };

                //
                if move_pos { q } else { q.reverse() }
            });

            //
            let sort_func_rev = &Box::new(|x:&FocusMoveWork,y:&FocusMoveWork|sort_func(x,y).reverse());

            //
            stk[stk_len..].sort_by(sort_func);
            stk_befores[stk_befores_len..].sort_by(sort_func_rev);
            stk_past_befores[stk_past_befores_len..].sort_by(sort_func_rev);
            stk_past_afters[stk_past_afters_len..].sort_by(sort_func);

            //
            if move_vert && parent_computed.cols > 1 { //because doesn't need to split when is 1
                from_bounds.push((cur_computed.col,parent_computed.cols,));
            } else if move_hori && parent_computed.rows > 1 {
                from_bounds.push((cur_computed.row,parent_computed.rows,));
            }

            //stop at first focusable ancestor
            if focus_entity_stk.len() >0 {
                if focus_entity_stk.len() >= (1+focus_depth) {
                    if let Some(&focus_entity)=focus_entity_stk.get(focus_entity_stk.len()-1-focus_depth) {
                        if parent_entity==focus_entity {
                            focus_depth+=1;
                            // break;
                        }
                    }
                }
            }
        }

        //everything was added to stk backwards

        //
        stk.extend(stk_past_afters);
        stk.extend(stk_past_befores.into_iter().rev());
        stk.extend(stk_befores.into_iter().rev());

        stk.reverse();
    } else if move_tab {
        //
        if let Some(&focus_stk_last_entity)=focus_entity_stk.last() {
            if let Ok(children)=children_query.get(focus_stk_last_entity) {
                // stk.extend(children.iter().map(|child_entity|( child_entity, vec![], (0,0), 0, true, )));
                stk.extend(children.iter().map(|child_entity|FocusMoveWork {
                    entity: child_entity, from_bounds: vec![], to_bound: (0,0), focus_depth: 0, valid: true,
                }));
            }
        } else {
            // stk.push(( top_root_entity, vec![], (0,0), 0, true, ));
            stk.push(FocusMoveWork { entity: top_root_entity, from_bounds: vec![], to_bound: (0,0), focus_depth: 0, valid: true });
        }

        //
        stk.sort_by(|x,y|{
            let x_computed=layout_computed_query.get(x.entity).unwrap();
            let y_computed=layout_computed_query.get(y.entity).unwrap();
            let q= x_computed.order.cmp(&y_computed.order);
            if move_dir==FocusMove::Next { q.reverse() } else { q } //
        });
    }

    //
    // println!("\n\nstk init {stk:?}");

    //
    let mut _found=false;

    //eval stk
    // while let Some((entity, from_bounds, to_bound, focus_depth,_valid))=stk.pop()
    while let Some(FocusMoveWork { entity, from_bounds, to_bound, focus_depth, valid:_valid })=stk.pop()
    {
        // println!("while stk: entity={entity:?}, from_bounds={from_bounds:?}, to_bound={to_bound:?}, focus_depth={focus_depth}, stk_len={}",stk.len());

        //
        let Ok(layout_computed) = layout_computed_query.get(entity) else {continue;};

        //
        if !layout_computed.unlocked {
            continue;
        }

        //when coming across a focusable, focus on it
        if focusable_query.get(entity).map(|focusable|cur_group==focusable.group && focusable.enable).unwrap_or_default() {
            //
            if let Some(cur_focus_entity) = *cur_focus_entity {
                ui_event_writer.write(UiInteractEvent{entity:cur_focus_entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device: cur_device }});
            }

            //
            if focus_depth>0 {
                for _ in 0 .. focus_depth {
                    let entity=focus_entity_stk.pop().unwrap();
                    ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device: cur_device }});
                }
            }

            //
            if let Some(ind)=move_dir.rev().ind() {
                device_move_hists.entry(entity).or_insert_with(||[Entity::PLACEHOLDER;4])[ind]=cur_focus_entity.unwrap();
            }

            //
            *cur_focus_entity = Some(entity);
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:cur_group, device:cur_device }});

            //println!("focus found {entity:?}, valid={valid}");

            // *hist_incr+=1; //the hist!
            // hist.insert(entity, *hist_incr); //the hist!

            // println!("\tfound focusable {entity:?},");

            //
            _found = true;
            break;
        }

        //else if non focusable, ancestor to a focusable
        let stk_len = stk.len();

        //non focusable's children
        //only add children from correct cols

        //
        if (move_vert && layout_computed.cols <= 1)
            || (move_hori && layout_computed.rows <= 1)
            || from_bounds.len()==0 //or move_tab
        { //no splits
            // println!("\tnosplit");

            if let Ok(children)=children_query.get(entity) {
                for child_entity in children.iter() {
                    let child_computed = layout_computed_query.get(child_entity).unwrap();

                    //
                    let new_to_len=if move_vert {
                        child_computed.cols
                    } else {
                        child_computed.rows
                    };

                    //
                    let new_to_bound = (0,new_to_len);
                    // stk.push(( child_entity, from_bounds.clone(), new_to_bound, focus_depth, true, ));
                    stk.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound: new_to_bound, focus_depth, valid: true });
                }
            }
        } else { //splits, not move_tab
            let mut from_bounds = from_bounds.clone();
            let (from_bound_start,from_bound_end)=from_bounds.pop().unwrap();
            let (to_bound_start,to_bound_end)=to_bound;
            let absolute_to_len = if move_vert { layout_computed.cols } else { layout_computed.rows };

            // let to_bound_len = to_bound_end-to_bound_start;

            //use from_bound_end as min, as any from_val min'd against it will always be smaller
            //use 0 as max, as any from_val max'd against it will always be larger

            //
            let mut to_from_map=(0..absolute_to_len).map(|_|(from_bound_end,0)).collect::<Vec<_>>(); //[to_ind]=(from_min,from_max)
            let mut from_to_stk= vec![((0,from_bound_end),(to_bound_start,to_bound_end))]; //[]=((from_start,from_len),(to_start,to_len))

            //
            // println!("\tto_from_map0={:?}", to_from_map.iter().enumerate() .map(|(to,(from_start,from_end))|format!("{from_start}..{from_end} => {to}")) .collect::<Vec<_>>() );

            //calculates to_from_map
            while let Some(((from_start,from_end),(to_start,to_end)))=from_to_stk.pop() {
                let from_len=from_end-from_start;
                let to_len=to_end-to_start;

                // println!("\twhile from_to_stk: (from_start,from_end)={:?},(to_start,to_end)={:?}, from_to_stk_len={}", (from_start,from_end),(to_start,to_end),from_to_stk.len(), );

                //
                if from_len == 1 {
                    // println!("\t\tfrom_len == 1");

                    for to in to_start .. to_end {
                        let from_range=&mut to_from_map[to as usize];

                        // // println!("\t\t\tfromrange1 {from_range:?} => {to}");

                        from_range.0=from_range.0.min(from_start);
                        from_range.1=from_range.1.max(from_start+1);

                        // println!("\t\t\tfromrange {from_range:?} => {to}");
                    }
                } else if to_len == 1 {
                    // println!("\t\tto_len == 1");

                    let from_range=&mut to_from_map[to_start as usize];

                    // println!("\t\t\tfromrange1 {from_range:?} => {to_start}");

                    from_range.0=from_range.0.min(from_start);
                    from_range.1=from_range.1.max(from_end); //from_start+from_len

                    // println!("\t\t\tfromrange2 {from_range:?} => {to_start}");

                } else if to_len!=0 && from_len > to_len && from_len%to_len == 0 {
                    // let div=from_bound_len/cur_to_len;
                    let div=from_len/to_len;

                    // println!("\t\tto_len!=0 && from_len > to_len && from_len%to_len == 0");
                    // println!("\t\t\tdiv={div}");

                    //
                    for to in to_start .. to_end {
                        let i = to-to_start;
                        let from_range=&mut to_from_map[to as usize];

                        // // println!("\t\t\tfromrange1 {from_range:?} => {to}");

                        from_range.0=from_range.0.min(from_start+ i*div);
                        from_range.1=from_range.1.max(from_start+ (i+1)*div);

                        // println!("\t\t\tfromrange {from_range:?} => {to}");
                    }
                } else if from_len!=0 && from_len < to_len && to_len%from_len == 0 {
                    // let div=cur_to_len/from_bound_len;
                    let div=to_len/from_len;

                    // println!("\t\tfrom_len!=0 && from_len < to_len && to_len%from_len == 0");
                    // println!("\t\t\tdiv={div}");

                    //
                    for to in to_start .. to_end {
                        let i = to-to_start;
                        let from_range=&mut to_from_map[to as usize];

                        // // println!("\t\t\tfromrange1 {from_range:?} => {to}");

                        from_range.0=from_range.0.min(from_start+ i/div);
                        from_range.1=from_range.1.max(from_start+ (i+div)/div);

                        // println!("\t\t\tfromrange {from_range:?} => {to}");
                    }
                } else if from_len == to_len {
                    // println!("\t\tfrom_len == to_len");
                    // println!("\t\t\tto_start {to_start}, to_end {to_end}");

                    //
                    for to in to_start .. to_end {
                        let i = to-to_start;
                        let from=i+from_start;
                        let from_range=&mut to_from_map[to as usize];

                        // // println!("\t\t\tfromrange1 {from_range:?} => {to}");

                        from_range.0=from_range.0.min(from);
                        from_range.1=from_range.1.max(from+1);

                        // println!("\t\t\tfromrange {from_range:?} => {to}");
                    }
                } else {
                    // println!("\t\telse");

                    let from_len_half=from_len/2;
                    let to_len_half=to_len/2;
                    let from_len_half2=from_len_half + if from_len%2 == 0 {0} else {1};
                    let to_len_half2=to_len_half + if to_len%2 == 0 {0} else {1};

                    // //// println!("= from_start={from_start}, from_end={from_end}, from_len={from_len2}, from_len_half={from_len_half}, from_len_half2={from_len_half2}");
                    // //// println!("= to_start={to_start}, to_end={to_end}, to_len={to_len2}, to_len_half={to_len_half}, to_len_half2={to_len_half2}");

                    // //// println!("= from_to_stk_push {:?}",( (from_start,from_start+from_len_half2), (to_start,to_start+to_len_half2), ));
                    // //// println!("= from_to_stk_push {:?}",( (from_start+from_len_half,from_start+from_len_half+from_len_half2), (to_start+to_len_half,to_start+to_len_half+to_len_half2), ));

                    //
                    from_to_stk.push((
                        (from_start,from_start+from_len_half2),
                        (to_start,to_start+to_len_half2),
                    ));

                    //
                    from_to_stk.push((
                        (from_start+from_len_half,from_start+from_len_half+from_len_half2),
                        (to_start+to_len_half,to_start+to_len_half+to_len_half2),
                    ));
                }

                // println!("\t\t\tto_from_map={:?}", to_from_map.iter().enumerate() .map(|(to,(from_start,from_end))|format!("{from_start}..{from_end} => {to}")) .collect::<Vec<_>>() );
            }

            //
            // if from_bounds.len()>0 {
            //     stk.push((entity, from_bounds,focus_depth));
            // } else
            {
                // let from_bound_len2=from_bound_end-from_bound_start;
                // // println!("hm {to_from_map:?}");

                let from_bound_len2=to_from_map.iter().max_by(|x,y|{
                    // // println!("{x:?}<{y:?}={:?}",x.1.cmp(&y.1));
                    x.1.cmp(&y.1)
                }).unwrap().1;

                let mut from_to_map =  (0 .. from_bound_len2).map(|_|(to_bound_end,0)).collect::<Vec<_>>(); //[from_ind]=(to_min,to_max)

                //
                for (to,&(from_start,from_end)) in to_from_map.iter().enumerate() {
                    let to = to as u32;
                    // let from_len = from_end-from_start;

                    //
                    for from in from_start .. from_end {
                        // let from=from_start+i;
                        let to_range=&mut from_to_map[from as usize];
                        to_range.0=to_range.0.min(to);
                        to_range.1=to_range.1.max(to+1);
                    }
                }

                // println!("\t\tfrom_to_map={:?}",from_to_map.iter().enumerate().map(|(from,(to_start,to_end))|format!("{from} => {to_start}..{to_end}")) .collect::<Vec<_>>());

                //
                let (to_start,to_end)=from_to_map[from_bound_start as usize];
                let to_len=to_end-to_start;

                //
                if to_len > 1 && from_bounds.len()>0 {
                    let new_to_bound=(to_start,to_start+to_len);
                    // let mut from_bounds =from_bounds.clone();
                    // // from_bounds.push((from_bound_ind,from_bound_len));

                    // println!("\t\tto_len > 1");
                    // println!("\t\t\tstk={stk:?}");
                    // println!("\t\t\tstk_push {:?}",(entity, &from_bounds,new_to_bound,focus_depth));

                    // stk.push((entity, from_bounds,new_to_bound,focus_depth,true));
                    stk.push(FocusMoveWork { entity, from_bounds, to_bound, focus_depth, valid: true });
                } else {
                    // println!("\t\tto_len <= 1");

                    if let Ok(children)=children_query.get(entity) {
                        //
                        for child_entity in children.iter() {
                            let Ok(child_computed) = layout_computed_query.get(child_entity) else {
                                continue;
                            };

                            let (to,new_to_len)=if move_vert {
                                (child_computed.col, child_computed.cols)
                            } else {
                                (child_computed.row, child_computed.rows)
                            };

                            //
                            let new_to_bound = (0,new_to_len);

                            //
                            // println!("\t\t\tchild={child_entity:?}");
                            // println!("\t\t\t\tto={to:?}, to_start={to_start:?}, to_end={to_end:?}, to_len={to_len:?}");

                            //
                            if to>=to_start && to<to_end //to_start+to_len
                            {
                                //
                                let (from_start,from_end)=to_from_map[to as usize];

                                //
                                // println!("\t\t\t\tfrom_start={from_start:?}, from_end={from_end:?}");

                                //
                                if from_bound_start >= from_start && from_bound_start < from_end {
                                    let mut from_bounds=from_bounds.clone();
                                    let from_len=from_end-from_start;

                                    //
                                    // println!("\t\t\t\tok");

                                    //add new from split
                                    if from_len>1 {
                                        let new_split_ind=from_bound_start-from_start;
                                        from_bounds.push((new_split_ind,from_len));

                                        //
                                        // println!("\t\t\t\t\tfrom_len>1");
                                        // println!("\t\t\t\t\t\tfrom_bounds_push ({new_split_ind},{from_len})");
                                    }

                                    //
                                    // println!("\t\t\t\tnew_to_len={new_to_len:?}");
                                    // println!("\t\t\t\t\tfrom_bounds={from_bounds:?}");
                                    // println!("\t\t\t\t\tstk={stk:?}");
                                    // println!("\t\t\t\t\tstkpush {:?}",(child_entity, &from_bounds,new_to_bound,focus_depth));

                                    //
                                    // stk.push((child_entity, from_bounds,new_to_bound,focus_depth,true));
                                    stk.push(FocusMoveWork { entity: child_entity, from_bounds, to_bound, focus_depth, valid: true });

                                    //
                                    continue;
                                } else {
                                    // println!("\t\t\t\tnot_ok from");
                                }
                            } else {
                                // println!("\t\t\t\tnot_ok to");
                            }

                            //adds invalid moves, which are sorted to the end of the stk, so they are only chosen
                            // if there are no other options
                            // stk.push((child_entity, from_bounds.clone(),new_to_bound,focus_depth,false));
                            stk.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound: new_to_bound, focus_depth, valid: false });

                        } //end for
                        // println!("\t\t\tstk={stk:?}");
                    }
                }
            }
        }

        //
        stk[stk_len..].sort_by(|x,y|{
            //does float ordering need to be done here as well?

            // let h = { //the hist!
            //     let x_hist=hist.get(&x.0).cloned().unwrap_or_default();
            //     let y_hist=hist.get(&y.0).cloned().unwrap_or_default();
            //     x_hist.cmp(&y_hist)
            // };

            //sort invalid ones to start

            let v= if x.valid && !y.valid {
                Ordering::Greater
            } else if !x.valid && y.valid {
                Ordering::Less
            } else {
                Ordering::Equal
            };

            //
            let x_computed=layout_computed_query.get(x.entity).unwrap();
            let y_computed=layout_computed_query.get(y.entity).unwrap();

            //
            if move_tab {
                let q=x_computed.order.cmp(&y_computed.order);
                let q = if move_pos { q.reverse() }else{ q };
                return q;
            }

            //
            let q = {
                let q=if move_vert {
                    x_computed.row.cmp(&y_computed.row)
                } else { //move_hori
                    x_computed.col.cmp(&y_computed.col)
                };

                if move_pos { q.reverse() } else { q }
            };

            //
            let r= if move_vert {
                x_computed.col.cmp(&y_computed.col).reverse()
            } else { //move_hori
                x_computed.row.cmp(&y_computed.row).reverse()
            };

            //
            let mut c1=0;
            let mut c2=0;

            //
            //if invalid then h more important than q
            //if valid then h more important than r

            // match h { //the hist!
            //     Ordering::Equal=> {
            //     }
            //     Ordering::Greater=> {
            //         c1+=2;
            //     }
            //     Ordering::Less=>{
            //         c2+=2;
            //     }
            // }

            //
            match v {
                Ordering::Equal=> {
                }
                Ordering::Greater=> {
                    c1+=4;
                }
                Ordering::Less=>{
                    c2+=4;
                }
            }

            //
            match q {
                Ordering::Equal=> {
                }
                Ordering::Greater=> {
                    c1+=3;
                }
                Ordering::Less=>{
                    c2+=3;
                }
            }

            //
            match r {
                Ordering::Equal=> {
                }
                Ordering::Greater=> {
                    c1+=1;
                }
                Ordering::Less=>{
                    c2+=1;
                }
            }

            //
            c1.cmp(&c2)

            //
            // let q=if q==Ordering::Equal { h } else { q };
            // let q=if q==Ordering::Equal { v } else { q };
            // let q=if q==Ordering::Equal { r } else { q };
            // q
        });

        // println!("\tstk2={stk:?}");
    } //end while
}


/*

//handle focused==true, but not in focus_entity_stk/cur_focus_entity
{
    for entity in focus_query.iter() {

        let Ok(computed) = computed_query.get(entity) else { continue; };
        let focusable=focusable_query.get(entity).unwrap();
        let focused=device_focuseds.0.contains(&entity);

        //don't need to check if focusable entity has ancestor with ui_root, as its computed.unlocked will be false if it doesn't
        if !computed.unlocked || !focusable.enable || !focused // !focusable.focused
        {
            continue;
        }

        //
        let cur_group=focusable.group;
        // let root_entity=parent_query.iter_ancestors(entity).last().unwrap_or(entity);

        // let Some(root_entity)=[entity].into_iter().chain(parent_query.iter_ancestors(entity)).find(|&ancestor_entity|root_query.contains(ancestor_entity)) else {
        //     continue;
        // };

        //
        let (cur_focus_entity,focus_entity_stk,hist)=device_focus_states
            .entry(computed.root_entity).or_default()
            .entry(cur_group).or_default();

        //
        if Some(entity)==*cur_focus_entity { //&& focus_ancestor_entities == focus_entity_stk
            continue;
        }

        //
        // let mut focus_ancestor_entities: Vec<Entity> = parent_query.iter_ancestors(entity).filter_map(|ancestor_entity|{
        //     focusable_query.get(ancestor_entity).ok().and_then(|ancestor_focusable|{
        //         (ancestor_focusable.enable&&ancestor_focusable.group==cur_group).then_some(ancestor_entity)
        //     })
        // }).collect::<Vec<_>>();

        let mut focus_ancestor_entities: Vec<Entity> = Vec::new();

        for ancestor_entity in parent_query.iter_ancestors(entity) {
            let is_focusable = focusable_query.get(ancestor_entity).ok().map(|ancestor_focusable|{
                ancestor_focusable.enable&&ancestor_focusable.group==cur_group
            }).unwrap_or_default();

            if is_focusable {
                focus_ancestor_entities.push(ancestor_entity);
            }

            if root_query.contains(entity)
            // if computed.root_entity==ancestor_entity
            {
                break;
            }
        }

        //

        focus_ancestor_entities.reverse();

        //
        if Some(entity) == focus_entity_stk.get(focus_ancestor_entities.len()).cloned() {
            continue;
        }

        //
        if focus_ancestor_entities.len()>focus_entity_stk.len() &&
            focus_ancestor_entities.get(focus_entity_stk.len()).cloned() == *cur_focus_entity
        {
            //move cur_focus_entity to focus_entity_stk
            focus_entity_stk.push(cur_focus_entity.unwrap());
            // prev_focused_stk.push(Default::default());
            *cur_focus_entity=None;
        } else if let Some(entity)=*cur_focus_entity {
            //unfocus cur_focus_entity
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device: 77 }});
            // focusable_query.get_mut(entity).unwrap().focused=false;
            device_focuseds.0.remove(&entity).then_some(()).unwrap();
            *cur_focus_entity=None;
        }

        //unfocus some/all of focus_entity_stk
        for i in 0..focus_entity_stk.len() {
            if focus_ancestor_entities.get(i)==focus_entity_stk.get(i) {
                continue;
            }

            //send focus ends
            for j in (i .. focus_entity_stk.len()).rev() {
                let entity=focus_entity_stk[j];
                ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device: 77 }});
                // focusable_query.get_mut(entity).unwrap().focused=false;
                device_focuseds.0.remove(&entity).then_some(()).unwrap();
            }

            //remove from focus_entity_stk
            focus_entity_stk.truncate(i);
            // prev_focused_stk.truncate(i+1);

            //
            break;
        }

        //fill focus_entity_stk with focus_ancestor_entities
        for i in focus_entity_stk.len() .. focus_ancestor_entities.len() {
            let entity=focus_ancestor_entities[i];
            focus_entity_stk.push(entity);
            // prev_focused_stk.push(Default::default());
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:cur_group, device: 77 }});

            // if let Ok(mut focusable2)=focusable_query.get_mut(entity)
            if focusable_query.contains(entity)
            {
                // focusable2.focused=true;
                device_focuseds.0.insert(entity);

                //is in right order? ie root => parent
                *hist_incr+=1;
                hist.insert(entity, *hist_incr);
            }
        }

        //
        *cur_focus_entity=Some(entity);
        ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:cur_group, device: 77 }});

        // focusable_query.get_mut(entity).unwrap().focused=true;
        device_focuseds.0.insert(entity);


        *hist_incr+=1;
        hist.insert(entity, *hist_incr);
    }
}




//for when focusable.focused is set to true, when prev unfocused?
for (&device,root_focus_group_focuses) in device_focus_group_focuses.iter() {
    let device_focus_states=focus_states.entry(device).or_default();

    // let device_focuseds = focuseds.0.entry(device).or_default(); //does exist?? //hmph

    for (&(root_entity,focus_group),&(entity,_order)) in root_focus_group_focuses.iter() {
        let (cur_focus_entity,focus_entity_stk,_hist)=device_focus_states
            .entry(root_entity).or_default()
            .entry(focus_group).or_default()
            ;

        if cur_focus_entity.is_none() && focus_entity_stk.is_empty() {

            //
            // let mut focus_ancestor_entities = parent_query.iter_ancestors(entity).filter_map(|ancestor_entity|{
            //     focusable_query.get(ancestor_entity).ok().and_then(|ancestor_focusable|{
            //         (ancestor_focusable.enable&&ancestor_focusable.group==focus_group).then_some(ancestor_entity)
            //     })
            // }).collect::<Vec<_>>();

            let mut focus_ancestor_entities: Vec<Entity> = Vec::new();

            for ancestor_entity in parent_query.iter_ancestors(entity) {
                let is_focusable = focusable_query.get(ancestor_entity).ok().map(|ancestor_focusable|{
                    ancestor_focusable.enable&&ancestor_focusable.group==focus_group
                }).unwrap_or_default();

                if is_focusable {
                    focus_ancestor_entities.push(ancestor_entity);
                }

                if root_query.contains(entity)
                // if computed.root_entity==ancestor_entity
                {
                    break;
                }
            }

            focus_ancestor_entities.reverse();

            //fill focus_entity_stk with focus_ancestor_entities
            for i in focus_entity_stk.len() .. focus_ancestor_entities.len() {
                let entity=focus_ancestor_entities[i];
                focus_entity_stk.push(entity);
                ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:focus_group, device }});

                // if let Ok(mut focusable2)=focusable_query.get_mut(entity)
                if focusable_query.contains(entity)
                {
                    // focusable2.focused=true;
                    // device_focuseds.insert(entity); //hmph

                    // *hist_incr+=1;
                    // hist.insert(entity, *hist_incr);
                }
            }

            // let mut focusable=focusable_query.get_mut(entity).unwrap();
            // focusable.focused=true;
            // device_focuseds.insert(entity); //hmph

            //not necessary, since it was selected because it had the highest last visit hist
            // *hist_incr+=1;
            // hist.insert(entity, *hist_incr);


            *cur_focus_entity=Some(entity);
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:focus_group, device }});
        }
    }
}
*/