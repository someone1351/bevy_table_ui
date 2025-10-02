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
*/

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use bevy::ecs::prelude::*;
// use bevy::hierarchy::prelude::*;


use super::super::components::*;
use super::super::resources::*;
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

#[derive(PartialEq,Debug)]
enum FocusMove {Left,Right,Up,Down,Prev,Next}


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
    mut input_event_reader: MessageReader<UiInteractInputMessage>,

    // computed_query: Query<&UiComputed>,
    computed_query: Query<&UiLayoutComputed,With<UiLayoutComputed>>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,

    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    mut focusable_query : Query<&mut UiFocusable>,

    focus_query : Query<Entity, (With<UiFocusable>,)>,
    // root_query: Query<Entity,(Without<Parent>,With<UiLayoutComputed>)>,

    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,

    children_query: Query<&Children,(With<UiLayoutComputed>,)>,

    mut ui_event_writer: MessageWriter<UiInteractEvent>,

    //todo replace cur_focus_entity with bool, and move cur_focus_entity to focus_entity_stk
    // mut focus_state.cur_focuses  : Local<HashMap<
    //     Entity, //root_entity
    //     HashMap<
    //         i32, //group
    //         (
    //             Option<Entity>, //cur_focus_entity
    //             Vec<Entity>, //focus_entity_stk
    //             // Vec<(Option<FocusMoveType>,HashSet<Entity>)>, //prev_focused_stk, len is focus_entity_stk.len()+1
    //             // Vec<Entity>, //hist
    //             HashMap<Entity,u64>,
    //         )
    //     >
    // >>, //[root_entity][group]=(cur_focus_entity,focus_entity_stk)
    mut focus_states : ResMut<UiFocusStates>,
    // mut cur_focuses:Local<HashMap<
    //     Entity, //root_entity
    //     HashMap<
    //         i32, //group
    //         (
    //             Option<Entity>, //cur_focus_entity
    //             Vec<Entity>, //focus_entity_stk
    //             HashMap<Entity,u64>, //hist
    //         )
    //     >
    // >>,

    mut hist_incr : Local<u64>,
) {

    //in cur_focus_entity, focus_entity_stk, unfocus removed entities, disabled/invisible entities, disabled focusables, focusables changed groups
    for (&root_entity,groups) in focus_states.cur_focuses.iter_mut() {
        // let root_entity_alive=computed_query.get(root_entity).is_ok();
        let root_entity_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();

        for (&cur_group,(cur_focus_entity,focus_entity_stk,hist)) in groups.iter_mut() {
            // //init prev_focused_stk
            // if prev_focused_stk.len()==0 {
            //     prev_focused_stk.push(Default::default());
            // }

            //remove removed entities from prev_focused_stk
            // for (_,prev_focused) in prev_focused_stk.iter_mut() {
            //     prev_focused.retain(|&entity|computed_query.contains(entity));
            // }
            hist.retain(|&entity,_|computed_query.contains(entity));

            //remove from cur_focus/focus_stk any invisible, changed group, no longer focusable or focusable.enabled
            for i in 0..focus_entity_stk.len() {
                let unlocked = computed_query.get(focus_entity_stk[i]).map(|x|x.unlocked).unwrap_or_default();
                let focusable=focusable_query.get(focus_entity_stk[i]).ok();
                let focusable_enabled = focusable.map(|x|x.enable && x.focused).unwrap_or_default();
                let node_focus_group = focusable.map(|x|x.group).unwrap_or_default();

                if !root_entity_alive || !focusable_enabled || !unlocked || node_focus_group!=cur_group {
                    //should not unfocus if invisible under some situations?

                    //also end this
                    if let Some(entity)=*cur_focus_entity {
                        ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group}});
                        *cur_focus_entity=None;

                        //comment out?
                        if let Ok(mut focusable)=focusable_query.get_mut(entity) {
                            focusable.focused=false;
                        }
                    }

                    //
                    for j in (i..focus_entity_stk.len()).rev() {
                        let entity=focus_entity_stk[j];
                        ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group}});

                        //comment out?
                        if let Ok(mut focusable)=focusable_query.get_mut(entity) {
                            focusable.focused=false;
                        }
                    }

                    //
                    focus_entity_stk.truncate(i);
                    // prev_focused_stk.truncate(i+1);
                    break;
                }
            }

            //remove from cur_focus disabled/invisible, changed group, no longer focusable
            if let Some(entity)=*cur_focus_entity {
                let unlocked = computed_query.get(entity).map(|x|x.unlocked).unwrap_or_default();

                let focusable = focusable_query.get_mut(entity).ok();
                let focusable_enabled=focusable.as_ref().map(|x|x.enable && x.focused).unwrap_or_default();
                let node_focus_group = focusable.as_ref().map(|x|x.group).unwrap_or_default();

                if !root_entity_alive || !focusable_enabled || !unlocked || node_focus_group != cur_group {
                    ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group}});
                    *cur_focus_entity=None;

                    //comment out?
                    if let Some(mut focusable)=focusable {
                        focusable.focused=false;
                    }
                }
            }
        }

    }

    //remove dead roots
    for root_entity in focus_states.cur_focuses.keys().cloned().collect::<Vec<_>>() {
        // let root_entity_alive=computed_query.get(root_entity).is_ok();
        let root_entity_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();

        if !root_entity_alive {
            focus_states.cur_focuses.remove(&root_entity);
        }
    }

    //handle focused==true, but not in focus_entity_stk/cur_focus_entity
    {
        for entity in focus_query.iter() {

            let Ok(computed) = computed_query.get(entity) else { continue; };
            let focusable=focusable_query.get(entity).unwrap();

            //don't need to check if focusable entity has ancestor with ui_root, as its computed.unlocked will be false if it doesn't
            if !computed.unlocked || !focusable.enable ||!focusable.focused {
                continue;
            }

            //
            let cur_group=focusable.group;
            // let root_entity=parent_query.iter_ancestors(entity).last().unwrap_or(entity);

            // let Some(root_entity)=[entity].into_iter().chain(parent_query.iter_ancestors(entity)).find(|&ancestor_entity|root_query.contains(ancestor_entity)) else {
            //     continue;
            // };

            //
            let (cur_focus_entity,focus_entity_stk,hist)=focus_states.cur_focuses
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
                ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group}});
                focusable_query.get_mut(entity).unwrap().focused=false;
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
                    ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group}});
                    focusable_query.get_mut(entity).unwrap().focused=false;
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
                ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:cur_group}});

                if let Ok(mut focusable2)=focusable_query.get_mut(entity) {
                    focusable2.focused=true;

                    //is in right order? ie root => parent
                    *hist_incr+=1;
                    hist.insert(entity, *hist_incr);
                }
            }

            //
            *cur_focus_entity=Some(entity);
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:cur_group}});
            focusable_query.get_mut(entity).unwrap().focused=true;


            *hist_incr+=1;
            hist.insert(entity, *hist_incr);
        }
    }

    //
    // let mut focusable_entity_visiteds = HashSet::new();
    let mut focusable_entity_roots = HashSet::new();
    let mut root_focus_group_focuses=HashMap::<(Entity,i32),(Entity,u32)>::new(); //[(root_entity,focus_group)]=(entity,order)

    //create set of focusable entities?
    for entity in focus_query.iter() {
        let computed=computed_query.get(entity).unwrap();
        let focusable=focusable_query.get(entity).unwrap();

        if !computed.unlocked || !focusable.enable //|| focusable_entity_visiteds.contains(&entity)
        {
            continue;
        }

        // let root_entity = parent_query.iter_ancestors(entity).last().unwrap_or(entity);
        // let Some(root_entity)=[entity].into_iter().chain(parent_query.iter_ancestors(entity)).find(|&ancestor_entity|root_query.contains(ancestor_entity)) else {
        //     continue;
        // };

        let (_cur_focus_entity,_focus_entity_stk,hist)=focus_states.cur_focuses
            .entry(computed.root_entity).or_default()
            .entry(focusable.group).or_default()
            ;


        focusable_entity_roots.insert(entity);

        let prev=root_focus_group_focuses.entry((computed.root_entity,focusable.group)).or_insert((entity,computed.order));


        let cur_hist=hist.get(&entity).cloned().unwrap_or_default();
        let prev_hist=hist.get(&prev.0).cloned().unwrap_or_default();

        if cur_hist>prev_hist || ((cur_hist==prev_hist)&&computed.order < prev.1 )
        {
            prev.0=entity;
            prev.1=computed.order;
        }
    }

    for (&(root_entity,focus_group),&(entity,_order)) in root_focus_group_focuses.iter() {
        let (cur_focus_entity,focus_entity_stk,_hist)=focus_states.cur_focuses
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
                ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:focus_group}});

                if let Ok(mut focusable2)=focusable_query.get_mut(entity) {
                    focusable2.focused=true;

                    // *hist_incr+=1;
                    // hist.insert(entity, *hist_incr);
                }
            }

            let mut focusable=focusable_query.get_mut(entity).unwrap();
            focusable.focused=true;

            //not necessary, since it was selected because it had the highest last visit hist
            // *hist_incr+=1;
            // hist.insert(entity, *hist_incr);


            *cur_focus_entity=Some(entity);
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:focus_group}});
        }
    }


    //

    let mut ev_stk= input_event_reader.read().cloned().collect::<Vec<_>>();
    ev_stk.reverse();



    let mut was_resent=false; //not sure if need? to stop infinite loop if no focusable is there

    //
    while let Some(ev)=ev_stk.pop() {

        if !root_query.get(ev.get_root_entity()).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
            continue;
        }


        match ev.clone() {
            UiInteractInputMessage::FocusEnter{root_entity,group} => {
                if let Some(groups)=focus_states.cur_focuses.get_mut(&root_entity) {
                    if let Some((cur_focus_entity,focus_entity_stk, _hist))=groups.get_mut(&group) {
                        if let Some(entity)=*cur_focus_entity {
                            focus_entity_stk.push(entity);
                            // prev_focused_stk.push(Default::default());
                            *cur_focus_entity=None;
                        }
                    }
                }
            }
            UiInteractInputMessage::FocusExit{root_entity,group} => {
                if let Some(groups)=focus_states.cur_focuses.get_mut(&root_entity) {
                    if let Some((cur_focus_entity,focus_entity_stk, _hist))=groups.get_mut(&group) {
                        //already checked above for enabled/unlocked
                        if let Some(entity)=*cur_focus_entity {
                            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group}});

                            *cur_focus_entity=focus_entity_stk.pop();

                            // if cur_focus_entity.is_some() {
                            //     prev_focused_stk.pop().unwrap();
                            // }

                            //
                            if let Ok(mut focusable)=focusable_query.get_mut(entity) {
                                focusable.focused=false;
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
        let (top_root_entity, cur_group,(cur_focus_entity,focus_entity_stk, hist))=match ev {
            UiInteractInputMessage::FocusEnter{root_entity,group}
                |UiInteractInputMessage::FocusPrev{root_entity,group}|UiInteractInputMessage::FocusNext{root_entity,group}
                |UiInteractInputMessage::FocusLeft{root_entity,group}|UiInteractInputMessage::FocusRight {root_entity,group}
                |UiInteractInputMessage::FocusUp{root_entity,group}|UiInteractInputMessage::FocusDown{root_entity,group}
                |UiInteractInputMessage::FocusInit{root_entity,group}
                // |UiInputEvent::FocusPressBegin(root_entity,group,..)
            => {
                (root_entity,group,focus_states.cur_focuses.entry(root_entity).or_default().entry(group).or_default())
            }
            _=>{continue;}
        };

        let move_dir=match ev.clone() {
            UiInteractInputMessage::FocusEnter{..}|UiInteractInputMessage::FocusNext{..}|UiInteractInputMessage::FocusRight{..}|UiInteractInputMessage::FocusDown{..}
            |UiInteractInputMessage::FocusInit{..}
                // |UiInputEvent::FocusPressBegin(..)
                if cur_focus_entity.is_none() => FocusMove::Next,
            UiInteractInputMessage::FocusPrev{..}|UiInteractInputMessage::FocusLeft{..}|UiInteractInputMessage::FocusUp{..} if cur_focus_entity.is_none()  =>  {
                if !was_resent {
                    ev_stk.push(ev.clone());
                    was_resent=true;
                } else {
                    was_resent=false;
                }

                FocusMove::Next
            },
            UiInteractInputMessage::FocusPrev{..} => FocusMove::Prev,
            UiInteractInputMessage::FocusNext{..} => FocusMove::Next,
            UiInteractInputMessage::FocusLeft{..} => FocusMove::Left,
            UiInteractInputMessage::FocusRight{..} => FocusMove::Right,
            UiInteractInputMessage::FocusUp{..} => FocusMove::Up,
            UiInteractInputMessage::FocusDown{..} => FocusMove::Down,
            _ => {continue;}
        };

        //
        {
            let move_hori = move_dir==FocusMove::Left || move_dir==FocusMove::Right;
            let move_vert = move_dir==FocusMove::Down || move_dir==FocusMove::Up;
            let move_tab = move_dir==FocusMove::Prev || move_dir==FocusMove::Next;
            let move_pos = move_dir==FocusMove::Down || move_dir==FocusMove::Right || move_dir==FocusMove::Next;

            let mut stk: Vec<(Entity,Vec<(u32,u32)>,(u32,u32),usize,bool)>=Vec::new(); //[]=(entity,from_bounds,(to_start,to_len),focus_depth,valid)

            //init stk
            //get all uncles, great uncles, great great uncles etc
            //  that are in same row/col (for hori/vert) or all if using order (for prev/enxt)

            if let Some(cur_focus_entity)=*cur_focus_entity {
                // let mut prev_entity = cur_focus_entity;

                //calculated from curfocus+focus_stk, used on "to" nodes,
                let mut from_bounds = Vec::new(); //[depth_len-depth-1]=(focus_nodes[depth].col,.focus_nodes[depth].parent.cols)
                //[(cur_focus.col,parent.cols),(parent.col,gparent.cols),(gparent.col,ggparent.cols) ]

                //past is for going past the edge and wrapping, forget why its split into befores/afters
                //befores is for ones behind ie in the opposite move dir, think also used in wrapping?


                //
                let mut stk_befores: Vec<(Entity, Vec<(u32, u32)>,(u32,u32), usize,bool)> = Vec::new(); //[]=(entity,from_bounds,to_range,focus_depth,valid)
                let mut stk_past_befores: Vec<(Entity, Vec<(u32, u32)>,(u32,u32), usize,bool)> = Vec::new();
                let mut stk_past_afters: Vec<(Entity, Vec<(u32, u32)>,(u32,u32), usize,bool)> = Vec::new();

                //
                let mut focus_depth = 0;

                //
                for cur_entity in [cur_focus_entity].into_iter().chain(parent_query.iter_ancestors(cur_focus_entity)) {
                    let Ok(parent_entity) = parent_query.get(cur_entity).map(|p|p.parent()) else {
                        break; //only loop entities with parent
                    };

                    let cur_computed = computed_query.get(cur_entity).unwrap();
                    let parent_computed = computed_query.get(parent_entity).unwrap();

                    let stk_len = stk.len();
                    let stk_befores_len = stk_befores.len();
                    let stk_past_befores_len = stk_past_befores.len();
                    let stk_past_afters_len = stk_past_afters.len();

                    let Ok(parent_children) = children_query.get(parent_entity) else {
                        continue;
                    };

                    //
                    for child_entity in parent_children.iter() {
                        // if !focusable_entity_visiteds.contains(&child_entity) { //not really necessary?
                        //     continue;
                        // }

                        let child_computed=computed_query.get(child_entity).unwrap();

                        // let float=float_query.get(child_entity).map(|x|x.float).unwrap_or_default();

                        // if float {
                        //     continue;
                        // }

                        if child_entity == cur_entity {
                            continue;
                        }

                        if (move_vert && child_computed.col != cur_computed.col) || (move_hori && child_computed.row != cur_computed.row) {
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

                        // let to_len=match move_dir {
                        //     FocusMove::Up|FocusMove::Down => parent_computed.rows,
                        //     FocusMove::Left|FocusMove::Right => parent_computed.cols,
                        //     FocusMove::Prev|FocusMove::Next => 0,
                        // };

                        let to_len=match move_dir {
                            FocusMove::Up|FocusMove::Down => child_computed.cols,
                            FocusMove::Left|FocusMove::Right => child_computed.rows,
                            FocusMove::Prev|FocusMove::Next => 0,
                        };

                        let to_bound=(0,to_len);

                        if is_after {
                            if focus_depth>0 {
                                stk_past_afters.push((child_entity,from_bounds.clone(),to_bound,focus_depth,true));
                            } else {
                                stk.push((child_entity,from_bounds.clone(),to_bound,0,true));
                            }
                        } else {
                            if focus_depth>0 {
                                stk_past_befores.push((child_entity,from_bounds.clone(),to_bound,focus_depth,true));
                            } else {
                                stk_befores.push((child_entity,from_bounds.clone(),to_bound,0,true));
                            }
                        }
                    }

                    //sort backwards
                    stk[stk_len..].sort_by(|x,y|{
                        let x_computed=computed_query.get(x.0).unwrap();
                        let y_computed=computed_query.get(y.0).unwrap();


                        let q = if move_tab {
                            x_computed.order.cmp(&y_computed.order)
                        } else if move_vert {
                            x_computed.row.cmp(&y_computed.row)
                        } else { //move_hori
                            x_computed.col.cmp(&y_computed.col)
                        };

                        let q=if q == Ordering::Equal {
                            let x_float=float_query.get(x.0).map(|f|f.float).unwrap_or_default();
                            let y_float=float_query.get(y.0).map(|f|f.float).unwrap_or_default();

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

                        if move_pos { q } else { q.reverse() }
                    });

                    stk_befores[stk_befores_len..].sort_by(|x,y|{
                        let x_computed=computed_query.get(x.0).unwrap();
                        let y_computed=computed_query.get(y.0).unwrap();

                        let q = if move_tab {
                            x_computed.order.cmp(&y_computed.order)
                        } else if move_vert {
                            x_computed.row.cmp(&y_computed.row)
                        } else { //move_hori
                            x_computed.col.cmp(&y_computed.col)
                        };

                        let q=if q == Ordering::Equal {
                            let x_float=float_query.get(x.0).map(|f|f.float).unwrap_or_default();
                            let y_float=float_query.get(y.0).map(|f|f.float).unwrap_or_default();

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

                        if move_pos { q.reverse() } else { q }
                    });

                    //
                    stk_past_afters[stk_past_afters_len..].sort_by(|x,y|{
                        let x_computed=computed_query.get(x.0).unwrap();
                        let y_computed=computed_query.get(y.0).unwrap();

                        let q = if move_tab {
                            x_computed.order.cmp(&y_computed.order)
                        } else if move_vert {
                            x_computed.row.cmp(&y_computed.row)
                        } else { //move_hori
                            x_computed.col.cmp(&y_computed.col)
                        };

                        let q=if q == Ordering::Equal {
                            let x_float=float_query.get(x.0).map(|f|f.float).unwrap_or_default();
                            let y_float=float_query.get(y.0).map(|f|f.float).unwrap_or_default();

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

                        if move_pos { q } else { q.reverse() }
                    });

                    stk_past_befores[stk_past_befores_len..].sort_by(|x,y|{
                        let x_computed=computed_query.get(x.0).unwrap();
                        let y_computed=computed_query.get(y.0).unwrap();

                        let q = if move_tab {
                            x_computed.order.cmp(&y_computed.order)
                        } else if move_vert {
                            x_computed.row.cmp(&y_computed.row)
                        } else { //move_hori
                            x_computed.col.cmp(&y_computed.col)
                        };

                        let q=if q == Ordering::Equal {
                            let x_float=float_query.get(x.0).map(|f|f.float).unwrap_or_default();
                            let y_float=float_query.get(y.0).map(|f|f.float).unwrap_or_default();

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

                        if move_pos { q.reverse() } else { q }
                    });

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
                stk_befores.reverse();
                stk_past_befores.reverse();
                stk.extend(stk_past_afters);
                stk.extend(stk_past_befores);
                stk.extend(stk_befores);
                stk.reverse();

                // println!("stk is {stk:?}");

            } else if move_tab {
                if let Some(&focus_stk_last_entity)=focus_entity_stk.last() {
                    if let Ok(children)=children_query.get(focus_stk_last_entity) {
                        stk.extend(children.iter().map(|child_entity|(
                            child_entity,
                            // computed_query.get(child_entity).unwrap(),
                            Vec::new(),
                            (0,0),
                            0,
                            true,
                        )));
                    }
                } else {
                    stk.push((
                        top_root_entity,
                        // computed_query.get(top_root_entity).unwrap(),
                        Vec::new(),
                        (0,0),
                        0,
                        true,
                    ));
                }

                stk.sort_by(|x,y|{
                    let x_computed=computed_query.get(x.0).unwrap();
                    let y_computed=computed_query.get(y.0).unwrap();

                    let q= x_computed.order.cmp(&y_computed.order);
                    if move_dir==FocusMove::Next { q.reverse() } else { q } //
                });
            }

            //
            // println!("\n\n");
            // println!("stk init {stk:?}");

            let mut _found=false;

            //eval stk
            while let Some((entity, from_bounds, to_bound, focus_depth,_valid))=stk.pop() {
                // println!("while stk: entity={entity:?}, from_bounds={from_bounds:?}, to_bound={to_bound:?}, focus_depth={focus_depth}, stk_len={}",stk.len());

                let Ok(computed) = computed_query.get(entity) else {continue;};

                if !computed.unlocked {
                    continue;
                }

                //when coming across a focusable, focus on it
                if let Some(focusable) = focusable_query.get(entity).ok() {
                    if cur_group==focusable.group && focusable.enable {
                        if let Some(cur_focus_entity) = *cur_focus_entity {
                            ui_event_writer.write(UiInteractEvent{entity:cur_focus_entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group}});

                            if let Ok(mut focusable)=focusable_query.get_mut(cur_focus_entity) {
                                focusable.focused=false;
                            }
                        }

                        if focus_depth>0 {
                            for _ in 0 .. focus_depth {
                                let entity=focus_entity_stk.pop().unwrap();
                                // prev_focused_stk.pop().unwrap();

                                ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group}});

                                if let Ok(mut focusable)=focusable_query.get_mut(entity) {
                                    focusable.focused=false;
                                }
                            }
                        }


                        //

                        *cur_focus_entity = Some(entity);
                        ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:cur_group}});


                        // if let Ok(mut focusable)=focusable_query.get_mut(entity) {
                        //     focusable.focused=true;
                        // }

                        focusable_query.get_mut(entity).unwrap().focused=true;

                        //println!("focus found {entity:?}, valid={valid}");

                                *hist_incr+=1;
                                hist.insert(entity, *hist_incr);
                        // println!("\tfound focusable {entity:?},");

                        _found = true;
                        break;
                    }
                }

                //else if non focusable, ancestor to a focusable
                let stk_len = stk.len();

                //non focusable's children
                //only add children from correct cols

                //

                if (move_vert && computed.cols <= 1)
                    || (move_hori && computed.rows <= 1)
                    || from_bounds.len()==0 //or move_tab
                { //no splits
                    // println!("\tnosplit");

                    if let Ok(children)=children_query.get(entity) {
                        for child_entity in children.iter() {
                            let child_computed = computed_query.get(child_entity).unwrap();
                            // let float=float_query.get(child_entity).map(|x|x.float).unwrap_or_default();

                            // if float {
                            //     continue;
                            // }

                            let new_to_len=if move_vert {
                                child_computed.cols
                            } else {
                                child_computed.rows
                            };

                            let new_to_bound = (0,new_to_len);

                            stk.push((
                                child_entity,
                                from_bounds.clone(),
                                new_to_bound,
                                focus_depth,
                                true,
                            ));
                        }
                    }
                } else { //splits, not move_tab
                    let mut from_bounds = from_bounds.clone();
                    let (from_bound_start,from_bound_end)=from_bounds.pop().unwrap();
                    let (to_bound_start,to_bound_end)=to_bound;


                    let absolute_to_len = if move_vert { computed.cols } else { computed.rows };

                    // let to_bound_len = to_bound_end-to_bound_start;

                    //use from_bound_end as min, as any from_val min'd against it will always be smaller
                    //use 0 as max, as any from_val max'd against it will always be larger

                    let mut to_from_map=(0..absolute_to_len).map(|_|(from_bound_end,0)).collect::<Vec<_>>(); //[to_ind]=(from_min,from_max)

                    //
                    let mut from_to_stk= vec![((0,from_bound_end),(to_bound_start,to_bound_end))]; //[]=((from_start,from_len),(to_start,to_len))

                    // println!("\tto_from_map0={:?}", to_from_map.iter().enumerate() .map(|(to,(from_start,from_end))|format!("{from_start}..{from_end} => {to}")) .collect::<Vec<_>>() );


                    //calculates to_from_map
                    while let Some(((from_start,from_end),(to_start,to_end)))=from_to_stk.pop() {
                        let from_len=from_end-from_start;
                        let to_len=to_end-to_start;

                        // println!("\twhile from_to_stk: (from_start,from_end)={:?},(to_start,to_end)={:?}, from_to_stk_len={}", (from_start,from_end),(to_start,to_end),from_to_stk.len(), );

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

                            from_to_stk.push((
                                (from_start,from_start+from_len_half2),
                                (to_start,to_start+to_len_half2),
                            ));

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

                        for (to,&(from_start,from_end)) in to_from_map.iter().enumerate() {
                            let to = to as u32;
                            // let from_len = from_end-from_start;

                            for from in from_start .. from_end {
                                // let from=from_start+i;
                                let to_range=&mut from_to_map[from as usize];
                                to_range.0=to_range.0.min(to);
                                to_range.1=to_range.1.max(to+1);
                            }
                        }

                        // println!("\t\tfrom_to_map={:?}",from_to_map.iter().enumerate().map(|(from,(to_start,to_end))|format!("{from} => {to_start}..{to_end}")) .collect::<Vec<_>>());

                        let (to_start,to_end)=from_to_map[from_bound_start as usize];
                        let to_len=to_end-to_start;

                        if to_len > 1 && from_bounds.len()>0 {
                            let new_to_bound=(to_start,to_start+to_len);
                            // let mut from_bounds =from_bounds.clone();
                            // // from_bounds.push((from_bound_ind,from_bound_len));

                            // println!("\t\tto_len > 1");
                            // println!("\t\t\tstk={stk:?}");
                            // println!("\t\t\tstk_push {:?}",(entity, &from_bounds,new_to_bound,focus_depth));

                            stk.push((entity, from_bounds,new_to_bound,focus_depth,true));
                        } else
                        {
                            // println!("\t\tto_len <= 1");

                            // let q=children_query.get(entity).map(|x|x.iter()).unwrap_or_default();

                            if let Ok(children)=children_query.get(entity) {
                                // let mut child_stk=Vec::new();
                                for child_entity in children.iter() {
                                    let child_computed = computed_query.get(child_entity).unwrap();

                                    // let float=float_query.get(child_entity).map(|x|x.float).unwrap_or_default();

                                    // if float {
                                    //     continue;
                                    // }

                                    let to=if move_vert {
                                        child_computed.col
                                    } else {
                                        child_computed.row
                                    };

                                    let new_to_len=if move_vert {
                                        child_computed.cols
                                    } else {
                                        child_computed.rows
                                    };
                                    let new_to_bound = (0,new_to_len);

                                    // println!("\t\t\tchild={child_entity:?}");
                                    // println!("\t\t\t\tto={to:?}, to_start={to_start:?}, to_end={to_end:?}, to_len={to_len:?}");

                                    if to>=to_start && to<to_end //to_start+to_len
                                    {
                                        let (from_start,from_end)=to_from_map[to as usize];
                                        // println!("\t\t\t\tfrom_start={from_start:?}, from_end={from_end:?}");

                                        if from_bound_start >= from_start && from_bound_start < from_end {
                                            let mut from_bounds=from_bounds.clone();
                                            let from_len=from_end-from_start;

                                            // println!("\t\t\t\tok");

                                            //add new from split
                                            if from_len>1 {
                                                let new_split_ind=from_bound_start-from_start;
                                                from_bounds.push((new_split_ind,from_len));

                                                // println!("\t\t\t\t\tfrom_len>1");
                                                // println!("\t\t\t\t\t\tfrom_bounds_push ({new_split_ind},{from_len})");
                                            }

                                            //



                                            //

                                            // println!("\t\t\t\tnew_to_len={new_to_len:?}");
                                            // println!("\t\t\t\t\tfrom_bounds={from_bounds:?}");
                                            // println!("\t\t\t\t\tstk={stk:?}");
                                            // println!("\t\t\t\t\tstkpush {:?}",(child_entity, &from_bounds,new_to_bound,focus_depth));

                                            //
                                            stk.push((child_entity, from_bounds,new_to_bound,focus_depth,true));
                                            continue;
                                        } else {
                                            // println!("\t\t\t\tnot_ok from");
                                        }
                                    } else {
                                        // println!("\t\t\t\tnot_ok to");
                                    }

                                    //adds invalid moves, which are sorted to the end of the stk, so they are only chosen
                                    // if there are no other options
                                    stk.push((child_entity, from_bounds.clone(),new_to_bound,focus_depth,false));

                                } //for
                                // println!("\t\t\tstk={stk:?}");
                            }
                        }
                    }
                }

                //
                stk[stk_len..].sort_by(|x,y|{
                    //does float ordering need to be done here as well?

                    let h = {
                        let x_hist=hist.get(&x.0).cloned().unwrap_or_default();
                        let y_hist=hist.get(&y.0).cloned().unwrap_or_default();
                        x_hist.cmp(&y_hist)
                    };

                    //sort invalid ones to start

                    let v= if x.4 && !y.4 {
                        Ordering::Greater
                    } else if !x.4 && y.4 {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    };

                    //
                    let x_computed=computed_query.get(x.0).unwrap();
                    let y_computed=computed_query.get(y.0).unwrap();

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

                    let r= if move_vert {
                        x_computed.col.cmp(&y_computed.col).reverse()
                    } else { //move_hori
                        x_computed.row.cmp(&y_computed.row).reverse()
                    };

                    let mut c1=0;
                    let mut c2=0;

                    //if invalid then h more important than q
                    //if valid then h more important than r
                    match h {
                        Ordering::Equal=> {
                        }
                        Ordering::Greater=> {
                            c1+=2;
                        }
                        Ordering::Less=>{
                            c2+=2;
                        }
                    }
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
                    // let q=if q==Ordering::Equal { h } else { q };
                    // let q=if q==Ordering::Equal { v } else { q };
                    // let q=if q==Ordering::Equal { r } else { q };

                    // q
                    c1.cmp(&c2)
                });
                // println!("\tstk2={stk:?}");
            } //while

            //
            // if let Some(entity)=cur_focus_entity.clone() { //the if let some, is maybe to skip when its doing focus_exit/enter
            //     //if found is true, means it moved, since it won't find anything if it hasn't moved

            //     match ev {
            //         UiInputEvent::FocusPrev(..) => {
            //             ui_event_writer.write(UiEvent{entity,event_type:UiEventType::FocusPrev{group:cur_group,moved:found}});
            //         }
            //         UiInputEvent::FocusNext(..) => {
            //             ui_event_writer.write(UiEvent{entity,event_type:UiEventType::FocusNext{group:cur_group,moved:found}});
            //         }
            //         UiInputEvent::FocusLeft(..) => {
            //             ui_event_writer.write(UiEvent{entity,event_type:UiEventType::FocusLeft{group:cur_group,moved:found}});
            //         }
            //         UiInputEvent::FocusRight(..) => {
            //             ui_event_writer.write(UiEvent{entity,event_type:UiEventType::FocusRight{group:cur_group,moved:found}});
            //         }
            //         UiInputEvent::FocusUp(..) => {
            //             ui_event_writer.write(UiEvent{entity,event_type:UiEventType::FocusUp{group:cur_group,moved:found}});
            //         }
            //         UiInputEvent::FocusDown(..) => {
            //             ui_event_writer.write(UiEvent{entity,event_type:UiEventType::FocusDown{group:cur_group,moved:found}});
            //         }
            //         _ => {
            //         }
            //     }
            // }
        }

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