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

* when something is focusable, but disabled, then don't allow children to be focused? is that implemented or not?
** can't go past ancestor that has focusable
*** what if want to leave cur nested focus, and move out?
**** eg going past end eg have column of horizontal options, where moving left/right goes between them, and up/down exits?
*/

use std::cmp::Ordering;
// use std::collections::HashMap;

use bevy::ecs::prelude::*;
// use bevy::hierarchy::prelude::*;


use crate::interact::vals::FocusMove;
use crate::FocusDevicePresseds;
use crate::FocusMoveHist2;
// use crate::FocusMoveHists;
use crate::FocusStates;

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

pub fn focus_move_cleanup(

    computed_query: Query<&UiLayoutComputed,With<UiLayoutComputed>>,
    // float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    // parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    // children_query: Query<&Children,(With<UiLayoutComputed>,)>,
    // focus_query : Query<Entity, (With<UiFocusable>,)>,
    mut focusable_query : Query<&mut UiFocusable>,

    // mut input_event_reader: MessageReader<UiInteractInputMessage>,
    mut ui_event_writer: MessageWriter<UiInteractEvent>,
    mut focus_states:ResMut<FocusStates>, //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk)
    // mut move_hists:ResMut<FocusMoveHists>, //[device][entity]=(left,top,right,bottom)
    mut move_hists:ResMut<FocusMoveHist2>,

    //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk,from_dirs)

    // mut hist_incr : Local<u64>,
) {
    //
    move_hists.0.retain(|&(_device,entity),_|computed_query.contains(entity));
    // //
    // move_hists.0.retain(|&(root_entity,_device),device_move_hists|{
    //     device_move_hists.retain(|&entity|{
    //         let layout_computed=computed_query.get(entity).ok();
    //         let focusable=focusable_query.get(entity).ok();

    //         focusable.is_some() && layout_computed.is_some() && layout_computed.unwrap().root_entity==root_entity
    //     });
    // //     device_move_hists.retain(|&entity,dirs|{
    // //         for entity2 in dirs {
    // //             if *entity2!=Entity::PLACEHOLDER && !computed_query.contains(*entity2) {
    // //                 *entity2=Entity::PLACEHOLDER;
    // //             }
    // //         }

    // //         computed_query.contains(entity)
    // //     });

    //     !device_move_hists.is_empty()
    // });

    //in cur_focus_entity, focus_entity_stk, unfocus removed entities, disabled/invisible entities, disabled focusables, focusables changed groups
    for (&device,device_focus_states) in focus_states.0.iter_mut() {
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
    focus_states.0.retain(|&_device,device_focus_states|{
        device_focus_states.retain(|&root_entity,_root_focus_states|{
            let root_entity_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();
            root_entity_alive
        });

        !device_focus_states.is_empty()
    });
}


pub fn focus_press_cleanup(
    root_query: Query<&UiLayoutComputed, With<UiRoot>>,
    layout_computed_query: Query<&UiLayoutComputed>,
    focusable_query: Query<(Entity,& UiFocusable)>,
    mut device_presseds : ResMut<FocusDevicePresseds>,
    mut ui_output_event_writer: MessageWriter<UiInteractEvent>,
) {
    //remove dead roots/pressed entities from device_presseds
    device_presseds.0.retain(|&button,button_device_presseds|{
        button_device_presseds.retain(|&(root_entity,device),&mut (pressed_entity,is_pressed)|{
            let root_alive= root_query.get(root_entity).map(|computed|computed.unlocked).unwrap_or_default();
            let (computed_root_entity,unlocked)=layout_computed_query.get(pressed_entity).map(|c|(c.root_entity,c.unlocked)).unwrap_or((Entity::PLACEHOLDER,false));
            let pressable_enabled=focusable_query.get(pressed_entity).map(|(_,c)|c.enable && c.pressable && (c.press_onlys.is_empty() || c.press_onlys.contains(&button))).unwrap_or_default();

            let b=root_alive && unlocked && pressable_enabled && computed_root_entity==root_entity; //&& entities_presseds_contains

            if !b && is_pressed {
                ui_output_event_writer.write(UiInteractEvent{entity:pressed_entity,event_type:UiInteractMessageType::FocusPressEnd{ device, button }});
            }

            b
        });

        !button_device_presseds.is_empty()
    });
}
fn do_press_down(
    root_entity: Entity,group: i32,device: i32, button: i32,
    focusable_query : Query<& UiFocusable>,
    focus_states:&FocusStates, //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk)
    device_presseds : &mut FocusDevicePresseds,
    ui_output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    //already pressed on an entity
    if device_presseds.0.get(&button).map(|button_device_presseds|{
        button_device_presseds.contains_key(&(root_entity,device))
    }).unwrap_or_default() {
        return;
    }

    let focused_entity=focus_states.0.get(&device)
        .and_then(|device_focus_states|device_focus_states.get(&root_entity))
        .and_then(|root_focus_states|root_focus_states.get(&group))
        .and_then(|(cur_focus_entity,_)|cur_focus_entity.clone());

    //
    if let Some(entity)=focused_entity {
        let pressable=focusable_query.get(entity).map(|c| c.enable && c.pressable && (c.press_onlys.is_empty() || c.press_onlys.contains(&button))).unwrap_or_default();//c.enable?
        if !pressable {return;}

        ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusPressBegin{ device, button }});

        device_presseds.0.entry(button).or_default().insert((root_entity,device),(entity,true));

    }
}

fn do_press_up(
    root_entity: Entity,
    // group: i32,
    device: i32, button: i32,
    // focusable_query : Query<& UiFocusable>,
    // focus_states:&FocusStates, //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk)
    device_presseds : &mut FocusDevicePresseds,
    ui_output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    let Some((pressed_entity,_is_pressed))=device_presseds.0.get_mut(&button)
        .and_then(|button_device_presseds|button_device_presseds.remove(&(root_entity,device)))
    else {
        return;
    };

    //
    ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::FocusPressEnd{ device, button }});
    ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::FocusClick{ device, button }});
}

fn do_press_cancel(
    root_entity: Entity,
    // group: i32,
    device: i32, button: i32,
    // focusable_query : Query<& UiFocusable>,
    // focus_states:&FocusStates, //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk)
    device_presseds : &mut FocusDevicePresseds,
    ui_output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    let Some((pressed_entity,_))=device_presseds.0.get_mut(&button)
        .and_then(|button_device_presseds|button_device_presseds.remove(&(root_entity,device)))
    else {
        return;
    };

    //
    ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::FocusPressEnd{ device, button }});
}

// fn get_cur_focus {
//             let (cur_focus_entity,focus_entity_stk)=focus_states.0
//             .entry(device).or_default()
//             .entry(root_entity).or_default()
//             .entry(group).or_default();
// }

fn do_focus_enter(
    root_entity: Entity,group: i32,device: i32,
    focusable_query : Query<& UiFocusable>,

    layout_computed_query: Query<&UiLayoutComputed>,
    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    children_query: Query<&Children,(With<UiLayoutComputed>,)>,

    // cur_focus_entity:&mut Option<Entity>,
    // focus_entity_stk: &mut Vec<Entity>,
    focus_states:&mut FocusStates,
    move_hists:&mut FocusMoveHist2,

    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    let (cur_focus_entity,focus_entity_stk)=focus_states.0
        .entry(device).or_default()
        .entry(root_entity).or_default()
        .entry(group).or_default();

    //
    if let Some(entity)=*cur_focus_entity {
        focus_entity_stk.push(entity);
        *cur_focus_entity=None;
    }


    /////here
    do_focus_move2(
        FocusMove::Next,
        root_entity,group,device,
        focusable_query ,

        layout_computed_query,
        parent_query,
        float_query,
        children_query,

        cur_focus_entity,
        focus_entity_stk,
        // &mut focus_states,
        move_hists,
        output_event_writer,
    );

    //
    //focus enter, on no focusable found (undo push above)

    if cur_focus_entity.is_none() {
        *cur_focus_entity=focus_entity_stk.pop();
    }
}
fn do_focus_exit(
    root_entity: Entity,
    group: i32,
    device: i32,

    // cur_focus_entity:&mut Option<Entity>,
    // focus_entity_stk: &mut Vec<Entity>,
    focus_states:&mut FocusStates,

    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    let (cur_focus_entity,focus_entity_stk)=focus_states.0
        .entry(device).or_default()
        .entry(root_entity).or_default()
        .entry(group).or_default();

    if let Some(entity)=*cur_focus_entity {
        output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group, device }});
        *cur_focus_entity=focus_entity_stk.pop();
    }

    // //should also exit all the way if only single focusable all the way?
    // continue;
}

fn do_focus_init(
    root_entity: Entity,group: i32,device: i32,
    focusable_query : Query<& UiFocusable>,

    layout_computed_query: Query<&UiLayoutComputed>,
    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    children_query: Query<&Children,(With<UiLayoutComputed>,)>,

    // cur_focus_entity:&mut Option<Entity>,
    // focus_entity_stk: &mut Vec<Entity>,
    focus_states:&mut FocusStates,
    move_hists:&mut FocusMoveHist2,

    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    let (cur_focus_entity,focus_entity_stk)=focus_states.0
        .entry(device).or_default()
        .entry(root_entity).or_default()
        .entry(group).or_default();

    if cur_focus_entity.is_some() {
        return;
    }

    do_focus_move2(
        FocusMove::Next,
        root_entity,group,device,
        focusable_query ,

        layout_computed_query,
        parent_query,
        float_query,
        children_query,

        cur_focus_entity,
        focus_entity_stk,
        // &mut focus_states,
        move_hists,
        output_event_writer,
    );
}


fn do_focus_move(
    move_dir:FocusMove,
    root_entity: Entity,group: i32,device: i32,
    focusable_query : Query<& UiFocusable>,

    layout_computed_query: Query<&UiLayoutComputed>,
    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    children_query: Query<&Children,(With<UiLayoutComputed>,)>,

    // cur_focus_entity:&mut Option<Entity>,
    // focus_entity_stk: &mut Vec<Entity>,
    focus_states:&mut FocusStates,
    move_hists:&mut FocusMoveHist2,

    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    let (cur_focus_entity,focus_entity_stk)=focus_states.0
        .entry(device).or_default()
        .entry(root_entity).or_default()
        .entry(group).or_default();

    do_focus_move2(
        move_dir,
        root_entity,group,device,
        focusable_query ,

        layout_computed_query,
        parent_query,
        float_query,
        children_query,

        cur_focus_entity,
        focus_entity_stk,
        // &mut focus_states,
        move_hists,
        output_event_writer,
    );
}

fn do_focus_move2(
    move_dir:FocusMove,
    root_entity: Entity,group: i32,device: i32,
    focusable_query : Query<& UiFocusable>,

    layout_computed_query: Query<&UiLayoutComputed>,
    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    children_query: Query<&Children,(With<UiLayoutComputed>,)>,

    cur_focus_entity:&mut Option<Entity>,
    focus_entity_stk: &mut Vec<Entity>,
    // focus_states:&mut FocusStates,
    move_hists:&mut FocusMoveHist2,

    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {


    //init
    let mut neg_init=false;

    if move_dir.negative() && !move_dir.tab() && cur_focus_entity.is_none() {
        if let Some((entity,_))=calc_focus_move( //don't need to worry about popping focus_entity_stk, since using FocusMove::Next won't try to exit
            FocusMove::Next,device,group,
            root_entity,None,
            focus_entity_stk,
            // device_move_hists,
            // &mut
            move_hists,
            parent_query,
            layout_computed_query,
            // &mut focus_computed_query,
            children_query,
            float_query,
            focusable_query,
        ) {
            *cur_focus_entity = Some(entity);
            // println!("init {entity}");
            neg_init=true;
        }
    }

    let move_result=calc_focus_move(
        move_dir,device,group,
        root_entity,cur_focus_entity.clone(),
        focus_entity_stk,
        // device_move_hists,
        // &mut
        move_hists,
        parent_query,layout_computed_query,
        // &mut focus_computed_query,
        children_query,float_query,focusable_query,
    );

    if let Some((entity,focus_depth))=move_result {
        //unfocus some ancestors?
        for _ in 0 .. focus_depth {
            let entity=focus_entity_stk.pop().unwrap();
            output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group, device }});
        }

        //should be after ancestors focus end?
        if let Some(cur_focus_entity) = *cur_focus_entity {
            if !neg_init {
                output_event_writer.write(UiInteractEvent{entity:cur_focus_entity,event_type:UiInteractMessageType::FocusEnd{group, device }});
            }
        }

        *cur_focus_entity = Some(entity);
        output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group, device }});
        // println!("hmm1! {neg_init}");
    // } else {
    //     println!("hmm2! {neg_init}");
    }

}

pub fn update_focus_events(


    // mut focus_computed_query: Query<&mut UiFocusableComputed>,
    layout_computed_query: Query<&UiLayoutComputed>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    children_query: Query<&Children,(With<UiLayoutComputed>,)>,
    // focus_query : Query<Entity, (With<UiFocusable>,)>,
    focusable_query : Query<& UiFocusable>,

    mut input_event_reader: MessageReader<UiInteractInputMessage>,
    mut output_event_writer: MessageWriter<UiInteractEvent>,

    // mut focus_states:Local<HashMap<i32,HashMap<Entity,HashMap<i32,(Option<Entity>,Vec<Entity>)>>>>, //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk)

    // mut move_hists:Local<HashMap<i32,HashMap<Entity,[Entity;4]>>>, //[device][entity]=(left,top,right,bottom)

    mut focus_states:ResMut<FocusStates>, //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk)
    // mut move_hists:ResMut<FocusMoveHists>, //[device][entity]=(left,top,right,bottom)
    mut move_hists:ResMut<FocusMoveHist2>,

    mut device_presseds : ResMut<FocusDevicePresseds>,
    //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk,from_dirs)

    // mut hist_incr : Local<u64>,
) {

    for &ev in input_event_reader.read() {
        if ev.root_entity()
            .and_then(|root_entity|root_query.get(root_entity).map(|(_,computed)|!computed.unlocked).ok())
            .unwrap_or_default()
        {
            continue;
        }

        match ev {
            UiInteractInputMessage::FocusInit{ root_entity, group, device } => {
                do_focus_init(
                    root_entity, group, device,
                    focusable_query, layout_computed_query, parent_query, float_query, children_query,
                    &mut focus_states, &mut move_hists, &mut output_event_writer,
                );
            }
            UiInteractInputMessage::FocusLeft{ root_entity, group, device }
                |UiInteractInputMessage::FocusRight{ root_entity, group, device }
                |UiInteractInputMessage::FocusUp{ root_entity, group, device }
                |UiInteractInputMessage::FocusDown{ root_entity, group, device }
                |UiInteractInputMessage::FocusPrev{ root_entity, group, device }
                |UiInteractInputMessage::FocusNext{ root_entity, group, device }
            => {
                let move_dir= match ev {
                    UiInteractInputMessage::FocusPrev{..} => FocusMove::Prev,
                    UiInteractInputMessage::FocusNext{..} => FocusMove::Next,
                    UiInteractInputMessage::FocusLeft{..} => FocusMove::Left,
                    UiInteractInputMessage::FocusRight{..} => FocusMove::Right,
                    UiInteractInputMessage::FocusUp{..} => FocusMove::Up,
                    UiInteractInputMessage::FocusDown{..} => FocusMove::Down,
                    _ => {continue;},
                };

                do_focus_move(
                    move_dir,
                    root_entity, group, device,
                    focusable_query, layout_computed_query, parent_query, float_query, children_query,
                    &mut focus_states, &mut move_hists, &mut output_event_writer,
                );
            }

            UiInteractInputMessage::FocusEnter { root_entity, group, device } => {
                do_focus_enter(root_entity, group, device,
                    focusable_query, layout_computed_query, parent_query, float_query, children_query,
                    &mut focus_states, &mut move_hists, &mut output_event_writer,
                );

            }
            UiInteractInputMessage::FocusExit { root_entity, group, device } => {
                do_focus_exit(root_entity, group, device, &mut focus_states, &mut output_event_writer);
            }
            UiInteractInputMessage::FocusPressBegin{root_entity,group,device, button } => {
                do_press_down(
                    root_entity,group,device, button,
                    focusable_query,
                    &mut focus_states,
                    &mut device_presseds,
                    &mut output_event_writer,
                );
            }
            UiInteractInputMessage::FocusPressEnd{root_entity,device, button } => {
                // let group=0;
                do_press_up(
                    root_entity,
                    // group,
                    device, button,
                    // focusable_query,
                    // &mut focus_states,
                    &mut device_presseds,
                    &mut output_event_writer,
                );
            }

            UiInteractInputMessage::FocusPressCancel{root_entity,device, button } => {
                // let group=0;
                do_press_cancel(
                    root_entity,
                    // group,
                    device, button,
                    // focusable_query,
                    // &mut focus_states,
                    &mut device_presseds,
                    &mut output_event_writer,
                );
            }

            // UiInteractInputMessage::FocusOn { entity, device } => {

            // }

            _ => {}
        }
    }

}

fn calc_focus_move(
    move_dir: FocusMove,
    device:i32,
    cur_group:i32,
    top_root_entity:Entity,
    cur_focus_entity:Option<Entity>,
    focus_entity_stk:& Vec<Entity>,
    // device_move_hists : &mut HashMap<Entity,[Entity;4]>, //[device][entiti]=(left,top,right,bottom)
    // move_hists:&mut FocusMoveHists,
    move_hists:&mut FocusMoveHist2,
    parent_query:Query<&ChildOf, With<UiLayoutComputed>>,
    layout_computed_query: Query<&UiLayoutComputed,>,
    // focus_computed_query: &mut Query<&mut UiFocusableComputed>,
    children_query: Query<&Children,(With<UiLayoutComputed>,)>,
    float_query: Query<&UiFloat,With<UiLayoutComputed>>,
    focusable_query : Query<& UiFocusable>,
) -> Option<(Entity,usize)> {
    // println!("go");
    //on nofocus(init) then up/back, don't send focus_begin/end for the init dif from up/back focus entity

    //with move_hists, if moving right, and reaching end, want it to wrap to first item in cur row's move_hist.left
    //  could look through list of the candidates, and search for the first candidate.move_hist.left

    //

    // let device_move_hists=move_hists.0.entry((top_root_entity,device)).or_default();
    // let device_move_hists_map:HashMap<Entity,usize>=device_move_hists.iter().enumerate().map(|(i,&e)|(e,i)).collect();

    //
    let move_hori = move_dir.horizontal();
    let move_vert = move_dir.vertical();
    let move_tab = move_dir.tab();
    let move_pos = move_dir.positive();

    //
    let (wrap,exit)=cur_focus_entity.and_then(|e|focusable_query.get(e).ok()).and_then(|c|{
        if move_hori {
            Some((c.hwrap,c.hexit))
        } else if move_vert {
            Some((c.vwrap,c.vexit))
        } else if move_tab {
            Some((true,false))
        } else {
            None
        }
    }).unwrap_or_default();

    //

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

    if let Some(cur_focus_entity)=cur_focus_entity {
        //calculated from curfocus+focus_stk, used on "to" nodes,
        //[depth_len-depth-1]=(focus_nodes[depth].col,.focus_nodes[depth].parent.cols)
        //    eg [(cur_focus.col,parent.cols),(parent.col,gparent.cols),(gparent.col,ggparent.cols) ]
        let mut from_bounds = Vec::new();

        //
        //befores are ones that come before, used for wrap
        //past are the ones that go past a focusable ancestor, used for exit


        let mut stk_befores: Vec<FocusMoveWork> = Vec::new();
        let mut stk_past_befores: Vec<FocusMoveWork> = Vec::new();
        let mut stk_past_afters: Vec<FocusMoveWork> = Vec::new();

        //
        let mut focus_depth = 0;

        //loop thru cur focused entity and its ancestors, adding children to stk
        for cur_entity in [cur_focus_entity].into_iter().chain(parent_query.iter_ancestors(cur_focus_entity)) {
            // if cur_entity!=cur_focus_entity && focusable_query.get(cur_entity).map(|focusable|!focusable.enable).unwrap_or_default() {
            //     continue;
            // }

            //can't focus outside ancestor's focusable (if there is one)
            // if cur_entity!=cur_focus_entity && focusable_query.contains(cur_entity) {
            //     continue;
            // }

            //
            let Ok(parent_entity) = parent_query.get(cur_entity).map(|p|p.parent()) else {
                break; //only loop entities with parent
            };

            let Ok(parent_children) = children_query.get(parent_entity) else {
                continue;
            };

            //
            // let cur_computed = layout_computed_query.get(cur_entity).unwrap();
            // let parent_computed = layout_computed_query.get(parent_entity).unwrap();

            //

            let Ok(cur_computed) = layout_computed_query.get(cur_entity) else {continue;};
            let Ok(parent_computed) = layout_computed_query.get(parent_entity) else {continue;};

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
                if focus_depth==0 {
                    if is_after {
                        stk.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound, focus_depth: 0, valid: true })
                    } else if wrap {
                        stk_befores.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound, focus_depth: 0, valid: true });
                    }
                } else if exit {
                    if is_after {
                        stk_past_afters.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound, focus_depth, valid: true });
                    } else if wrap {
                        stk_past_befores.push(FocusMoveWork { entity: child_entity, from_bounds: from_bounds.clone(), to_bound, focus_depth, valid: true });
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
                let q= if move_pos { q } else { q.reverse() };
                q
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

            //
            if focus_entity_stk.len() >0 {
                if focus_entity_stk.len() >= (1+focus_depth) {
                    if let Some(&focus_entity)=focus_entity_stk.get(focus_entity_stk.len()-1-focus_depth) {
                        if parent_entity==focus_entity {
                            focus_depth+=1;

                            //stop at first focusable ancestor
                            // if !exit {
                            //     break;
                            // }
                        }
                    }
                }
            }
        } //end for

        //everything was added to stk backwards

        //
        stk.extend(stk_past_afters);
        stk.extend(stk_past_befores.into_iter().rev());
        stk.extend(stk_befores.into_iter().rev());

        stk.reverse();
    // } else { //if move_tab
    } else if let Some(&focus_stk_last_entity)=focus_entity_stk.last() { //just init if no focus
        if let Ok(children)=children_query.get(focus_stk_last_entity) {
            stk.extend(children.iter().filter_map(|child_entity|{
                layout_computed_query.contains(child_entity).then_some(FocusMoveWork {
                    entity: child_entity,
                    from_bounds: vec![],
                    to_bound: (0,0),
                    focus_depth: 0,
                    valid: true,
                })
            }));
        }

        //
        stk.sort_by(|x,y|{
            let x_computed=layout_computed_query.get(x.entity).unwrap();
            let y_computed=layout_computed_query.get(y.entity).unwrap();
            let q= x_computed.order.cmp(&y_computed.order);
            let q= if move_pos { q.reverse() } else { q };
            q
        });
    } else { //just init if no focus
        stk.push(FocusMoveWork { entity: top_root_entity, from_bounds: vec![], to_bound: (0,0), focus_depth: 0, valid: true });
    }

    //
    // println!("\n\nstk init {stk:?}");

    //
    // let mut _found=false;

    //
    // let hist_last = cur_focus_entity.and_then(|cur_focus_entity|{
    //     move_dir.ind().and_then(|ind|device_move_hists.get(&cur_focus_entity).map(|x|x[ind]))
    // });


    // if let Some(hist)=hist_last {
    //     println!("h {hist:?}");

    // }
    // if let Some(cur_focus_entity)=cur_focus_entity {
    //     if let Some(ind)=move_dir.ind() { //get opposite dir
    //         if let Some(x)=device_move_hists.get(&cur_focus_entity).map(|x|x[ind]) {

    //         }

    // }

    //ancestors

    // let mut ancestors:HashMap<Entity,Vec<Entity>> = Default::default();



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

            // // //unfocus some ancestors?
            // // if focus_depth>0 {
            // //     for _ in 0 .. focus_depth {
            // //         let entity=focus_entity_stk.pop().unwrap();
            // //         ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device: cur_device }});
            // //     }
            // // }

            // // //should be after ancestors focus end?
            // // if let Some(cur_focus_entity) = cur_focus_entity {
            // //     ui_event_writer.write(UiInteractEvent{entity:cur_focus_entity,event_type:UiInteractMessageType::FocusEnd{group:cur_group, device: cur_device }});
            // // }

            // // //
            // // if let Some(cur_focus_entity)=cur_focus_entity {
            // //     if let Some(rev_ind)=move_dir.rev().ind() { //get opposite dir
            // //         // let x=device_move_hists.entry(entity).or_insert_with(||[Entity::PLACEHOLDER;4]).get_mut(rev_ind).unwrap();
            // //         // // let y=*x;


            // //         // *x=cur_focus_entity;
            // //         // // let x=device_move_hists.entry(y).or_insert_with(||[Entity::PLACEHOLDER;4]).get_mut(ind).unwrap();
            // //         // // *x=Entity::PLACEHOLDER;

            // //         // device_move_hists.entry(entity).or_insert_with(||[Entity::PLACEHOLDER;4])[rev_ind]=cur_focus_entity;


            // //         // let ind=move_dir.ind().unwrap();

            // //         // for i in (0..stk.len()).rev() {
            // //         //     let w=&stk[i];

            // //         //     if !w.valid {
            // //         //         break;
            // //         //     }

            // //         //     if i!=0 {
            // //         //         let w2=&stk[i-1];

            // //         //         if w2.valid {

            // //         //             device_move_hists.entry(w.entity).or_insert_with(||[Entity::PLACEHOLDER;4])[ind]=cur_focus_entity;
            // //         //         }

            // //         //     }


            // //         //     device_move_hists.entry(w.entity).or_insert_with(||[Entity::PLACEHOLDER;4])[rev_ind]=cur_focus_entity;
            // //         // }

            // //         // for rest in stk.iter().rev().filter(|x|x.valid) {
            // //         //     //only do valid ones
            // //         //     // if !rest.valid {
            // //         //     //     // break;
            // //         //     // }

            // //         //     //


            // //         //     device_move_hists.entry(rest.entity).or_insert_with(||[Entity::PLACEHOLDER;4])[rev_ind]=cur_focus_entity;


            // //         // }
            // //     }
            // // }

            // //
            // if let Some(old_pos)=device_move_hists.iter().rev().position(|&x|entity==x) {
            //     device_move_hists.remove(old_pos);
            // }
            // device_move_hists.push(entity);
            // //
            // // *cur_focus_entity = Some(entity);
            // // ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::FocusBegin{group:cur_group, device:cur_device }});

            // // //println!("focus found {entity:?}, valid={valid}");

            // // // *hist_incr+=1; //the hist!
            // // // hist.insert(entity, *hist_incr); //the hist!

            // // // println!("\tfound focusable {entity:?},");

            // // //
            // // // _found = true;
            // // // break;

            // if move_hori || move_vert
            {
                let mut last_entity=entity;
                for ancestor_entity in parent_query.iter_ancestors(entity) {
                    let last_layout_computed=layout_computed_query.get(last_entity).unwrap();
                    // let mut ancestor_focus_computed=focus_computed_query.get_mut(ancestor_entity).unwrap();

                    move_hists.0.insert((device,ancestor_entity), (last_layout_computed.row,last_layout_computed.col));
                    // println!("=={:?}",((device,ancestor_entity), (last_layout_computed.row,last_layout_computed.col)));
                    // // if move_hori {
                    //     ancestor_focus_computed.hist_row=Some(last_layout_computed.row);
                    // // } else {// if move_vert
                    //     ancestor_focus_computed.hist_col=Some(last_layout_computed.col);
                    // // }

                    last_entity=ancestor_entity;
                }
                // if let Ok(parent_entity)=parent_query.get(entity).map(|p|p.parent()) {
                //     let layout_computed=layout_computed_query.get(entity).unwrap();
                //     let mut parent_focus_computed=focus_computed_query.get_mut(parent_entity).unwrap();


                //     if move_hori {
                //         parent_focus_computed.hist_row=Some(layout_computed.row);
                //     } else // if move_vert
                //     {
                //         parent_focus_computed.hist_col=Some(layout_computed.col);
                //     }
                // }
            }

            return Some((entity,focus_depth))
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
                    // let child_computed = layout_computed_query.get(child_entity).unwrap();

                    let Ok(child_computed) = layout_computed_query.get(child_entity)else{continue;};

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

            //what is from/to about? it's the current row/col? and the target row/col?
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
                    stk.push(FocusMoveWork { entity, from_bounds, to_bound:new_to_bound, focus_depth, valid: true });
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
                                    stk.push(FocusMoveWork { entity: child_entity, from_bounds, to_bound:new_to_bound, focus_depth, valid: true });

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
        // let hist_last= (!move_dir.tab()).then(||device_move_hists.get(entity).and_then(|hist|))
        // let hist_last= move_dir.rev().ind().and_then(|ind|device_move_hists.get(&entity).map(|hist|hist[ind]));

        // if let Some(x)=device_move_hists.get(entity) {
        //     if let Some(ind)=move_dir.rev().ind() { //get opposite dir
        //     }


        //     // device_move_hists.get(entity).or_insert_with(||[Entity::PLACEHOLDER;4])[ind]=cur_focus_entity;
        // }

        //
        let move_hist=move_hists.0.get(&(device,entity))
        .and_then(|x|if move_hori{Some(x.0)} else if move_vert {Some(x.1)}else{None});

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

            //added
            if !v.is_eq() {
                return v;
            }

            //
            let x_computed=layout_computed_query.get(x.entity).unwrap();
            let y_computed=layout_computed_query.get(y.entity).unwrap();

            let x_order = if move_tab {
                x_computed.order
            } else if move_vert {
                x_computed.row
            } else { //move_hori
                x_computed.col
            };

            let y_order = if move_tab {
                y_computed.order
            } else if move_vert {
                y_computed.row
            } else { //move_hori
                y_computed.col
            };

            let q=x_order.cmp(&y_order);

            // //
            // let q = if move_tab {
            //     x_computed.order.cmp(&y_computed.order)
            // } else if move_vert {
            //     x_computed.row.cmp(&y_computed.row)
            // } else { //move_hori
            //     x_computed.col.cmp(&y_computed.col)
            // };

            let q = if move_pos { q.reverse() } else { q };

            if move_tab {
                return q;
            }

            //

            let x_order_alt = if move_hori {
                x_computed.row
            } else { //move_vert
                x_computed.col
            };

            let y_order_alt = if move_hori {
                y_computed.row
            } else { //move_vert
                y_computed.col
            };
            //added
            if !q.is_eq() {
                return q;
            }

            // let x_parent=parent_query.get(x.entity).map(|p|p.parent()).ok();
            // let y_parent=parent_query.get(y.entity).map(|p|p.parent()).ok();

            // let x_hist=x_parent.and_then(|p|move_hists.0.get(&(device,p)));
            // let y_hist=y_parent.and_then(|p|move_hists.0.get(&(device,p)));

            // let x_hist=x_hist.map(|v|if move_hori {v.0}else{v.1});
            // let y_hist=y_hist.map(|v|if move_hori {v.0}else{v.1});

            // println!("h {x_hist:?} {y_hist:?}, {x_order} {y_order}");
            // //
            // let xhist=parent_query.get(x.entity).map(|p|p.parent()).ok()
            //     .map(|parent_entity|focus_computed_query.get(parent_entity).unwrap())
            //     .and_then(|parent_focus_computed|
            // {
            //     if move_hori {
            //         parent_focus_computed.hist_row
            //     } else { //move_vert
            //         parent_focus_computed.hist_col
            //     }
            // });

            // let yhist=parent_query.get(x.entity).map(|p|p.parent()).ok()
            //     .map(|parent_entity|focus_computed_query.get(parent_entity).unwrap())
            //     .and_then(|parent_focus_computed|
            // {
            //     if move_hori {
            //         parent_focus_computed.hist_row
            //     } else { //move_vert
            //         parent_focus_computed.hist_col
            //     }
            // });

            if let Some(move_hist)=move_hist {

                let x_dif=x_order_alt.max(move_hist)-x_order_alt.min(move_hist);
                let y_dif=y_order_alt.max(move_hist)-y_order_alt.min(move_hist);
                let c=x_dif.cmp(&y_dif).reverse();


                if !c.is_eq() {
                    return c;
                }

                // if x_order_alt==move_hist {
                //     return Ordering::Greater;
                // } else if y_order_alt==move_hist {
                //     return Ordering::Less;
                // }
                // let x_dif=x_order_alt.max(move_hist)-x_order_alt.min(move_hist);
                // let y_dif=y_order_alt.max(move_hist)-y_order_alt.min(move_hist);
                // let c=x_dif.cmp(&y_dif).reverse();

                // if !c.is_eq() {
                //     return c;
                // }
            }

            // //
            // match (x_hist,y_hist) {
            //     (Some(x_hist), Some(y_hist)) => {
            //         println!("==a");
            //         let x_dif=x_order.max(x_hist)-x_order.min(x_hist);
            //         let y_dif=y_order.max(y_hist)-y_order.min(y_hist);
            //         let c=x_dif.cmp(&y_dif).reverse();

            //         if !c.is_eq() {
            //             return c;
            //         }
            //     }
            //     (Some(_), None) => {
            //         println!("==b");
            //         return Ordering::Greater;
            //     }
            //     (None, Some(_)) => {
            //         println!("==c");
            //         return Ordering::Less;
            //     }
            //     (None, None) => {
            //     }
            // }

            // if let (Some(xhist),Some(yhist))=(xhist,yhist) {

            // }

            //
            // let x_ancestors = parent_query.iter_ancestors(x.entity).collect::<Vec<_>>();
            // let y_ancestors = parent_query.iter_ancestors(y.entity).collect::<Vec<_>>();
            // //

            // for i in 0..x_ancestors.len().max(y_ancestors.len()) {
            //     let x_ancestor=x_ancestors.get(i).cloned();
            //     let y_ancestor=y_ancestors.get(i).cloned();

            // }



            //
            // if let Some(cur_focus_entity)=cur_focus_entity {
            //     for a in parent_query.iter_ancestors(cur_focus_entity) {

            //     }
            // }

            // here
            // let x_hist=device_move_hists_map.get(&x.entity).cloned();
            // let y_hist=device_move_hists_map.get(&y.entity).cloned();

            // if x_hist.is_some() && y_hist.is_some(){
            //     return x_hist.unwrap().cmp(&y_hist.unwrap());
            // } else if x_hist.is_some() && y_hist.is_none() {
            //     return Ordering::Greater;
            // } else if x_hist.is_none() && y_hist.is_some() {
            //     return Ordering::Less;
            // }


            // if let Some(hist)=hist_last {
            //     if hist==x.entity {
            //         println!("x {} => {hist}",cur_focus_entity.map(|x|format!("{x:?}")).unwrap_or("_".into()));
            //     } else if hist==y.entity {
            //         println!("y {} => {hist}",cur_focus_entity.map(|x|format!("{x:?}")).unwrap_or("_".into()));

            //     }
            // }

            // if hist_last==Some(x.entity) {
            //     return Ordering::Greater;
            // } else if hist_last==Some(y.entity) {
            //     return Ordering::Less;
            // }

            // let  s=if hist_last==Some(x.entity) {
            //     Some(Ordering::Less)
            // } else if hist_last==Some(y.entity) {
            //     Some(Ordering::Greater)
            // } else {
            //     None
            // };

            //
            let r=x_order_alt.cmp(&y_order_alt).reverse();
            //
            // let r= if move_vert {
            //     x_computed.col.cmp(&y_computed.col).reverse()
            // } else { //move_hori
            //     x_computed.row.cmp(&y_computed.row).reverse()
            // };

            // //added
            // if !r.is_eq() {
            //     return r;
            // }


            //
            // if let Some(x)=device_move_hists.get(entity) {
            //     if let Some(ind)=move_dir.rev().ind() { //get opposite dir
            //     }


            //     // device_move_hists.get(entity).or_insert_with(||[Entity::PLACEHOLDER;4])[ind]=cur_focus_entity;
            // }

            //
            r
            // //
            // let mut c1=0;
            // let mut c2=0;

            // //
            // //if invalid then h more important than q
            // //if valid then h more important than r

            // // match h { //the hist!
            // //     Ordering::Equal=> {
            // //     }
            // //     Ordering::Greater=> {
            // //         c1+=2;
            // //     }
            // //     Ordering::Less=>{
            // //         c2+=2;
            // //     }
            // // }

            // //
            // match v {
            //     Ordering::Equal=> {
            //     }
            //     Ordering::Greater=> {
            //         c1+=4;
            //     }
            //     Ordering::Less=>{
            //         c2+=4;
            //     }
            // }

            // //
            // match q {
            //     Ordering::Equal=> {
            //     }
            //     Ordering::Greater=> {
            //         c1+=3;
            //     }
            //     Ordering::Less=>{
            //         c2+=3;
            //     }
            // }

            // //
            // match r {
            //     Ordering::Equal=> {
            //     }
            //     Ordering::Greater=> {
            //         c1+=1;
            //     }
            //     Ordering::Less=>{
            //         c2+=1;
            //     }
            // }

            // //
            // c1.cmp(&c2)

            // //
            // // let q=if q==Ordering::Equal { h } else { q };
            // // let q=if q==Ordering::Equal { v } else { q };
            // // let q=if q==Ordering::Equal { r } else { q };
            // // q
        });

        // println!("\tstk2={stk:?}");
    } //end while

    None
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