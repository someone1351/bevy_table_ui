/*
TODO
* add DeviceRemoved(device,is_cursor) event
* instead of just using pressable.pressed, only use it for press begin, and use pressable.released for press end?
* should pressable.pressed work when node is locked?
* handle pressable.released, so it clicks

* does focus press/release need a "device"?
* * replace with enum Device {CursorDevice(i32),FocusGroup(i32)} ??


* handle on rect moved/resized so old cursor no longer inside => unpress
* could just add cursormove to end of input events with last cursor

* fix on root/entity removed section, some overlap, and pressed entities not being removed from all local resources?

TODO
* allow additional buttons for press? or just provide it an alternate "device" for dif buttons?

* separate press begin/end/click by device, don't have "physical" type buttons
*/

use std::collections::{HashMap, HashSet};

use bevy::ecs::prelude::*;
// use bevy::hierarchy::prelude::*;
use bevy::math::Vec2;

use super::super::components::*;
use super::super::resources::*;
use super::super::messages::*;
// use super::super::utils::*;
// use super::super::values::*;

use super::super::super::layout::components::{UiLayoutComputed,UiRoot};

// fn is_no_entity_pressed(
//     entity_presseds:&HashSet<DeviceType>, //[device_type]
//     device_presseds : &HashMap<(Entity,DeviceType),(Entity,bool)>, //[(root_entity,device_type)]=(pressed_entity,pressed)
//     root_entity : Entity,
//     press_always : bool,
// ) -> bool {
//     entity_presseds.iter().fold(true, |prev,&device_type|{
//         prev && !device_presseds.get(&(root_entity,device_type)).unwrap().1 && !press_always
//     })
// }


pub fn update_press_events(
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    layout_computed_query: Query<&UiLayoutComputed>,
    mut pressable_query: Query<(Entity,&mut UiPressable)>,

    focus_states : Res<UiFocusStates>,
    focuseds : Res<UiFocuseds>,

    mut device_cursors : Local<HashMap<(Entity,i32),Vec2>>, //[(root_entity,device)]=cursor

    mut device_presseds :   Local<HashMap<i32,HashMap<(Entity,DeviceType),(Entity,bool)>>>, //[button][(root_entity,device_type)]=(pressed_entity,is_pressed)
    mut entities_presseds : Local<HashMap<i32,HashMap<Entity,HashSet<DeviceType>>>>,      //[button][press_entity][device_type]

    // mut device_presseds : Local<HashMap<i32,HashMap<(Entity,DeviceType),Entity>>>,     //[button][(root_entity,device_type)]=pressed_entity
    // mut entities_presseds : Local<HashMap<i32,HashMap<Entity,HashMap<DeviceType,bool>>>>, //[button][press_entity][device_type]=cur_pressed

    mut input_event_reader: MessageReader<UiInteractInputMessage>,
    mut ui_output_event_writer: MessageWriter<UiInteractEvent>,
) {
    //device_cursors: remove dead roots/pressed entities from device_cursors
    device_cursors.retain(|&(root_entity,_device),_|{
        let root_unlocked= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();
        root_unlocked
    });

    //entities_presseds: remove dead roots or disabled/locked pressable entities
    entities_presseds.retain(|&_button,button_entities_presseds|{
        button_entities_presseds.retain(|&pressed_entity,_device_types|{ //(root_entity,pressed_entity)
            let root_entity=layout_computed_query.get(pressed_entity).map(|c|c.root_entity);
            let root_unlocked= root_entity.and_then(|root_entity|root_query.get(root_entity)).map(|(_root_entity,c)|c.unlocked).unwrap_or_default();
            let unlocked=layout_computed_query.get(pressed_entity).map(|c|c.unlocked).unwrap_or_default();
            let pressable=pressable_query.get(pressed_entity).map(|(_,pressable)|pressable.enable).unwrap_or_default();
            root_unlocked && pressable && unlocked
        });

        !button_entities_presseds.is_empty()
    });

    //remove dead roots/pressed entities from device_presseds
    device_presseds.retain(|&button,button_device_presseds|{
        button_device_presseds.retain(|&(root_entity,device_type),&mut (pressed_entity,_pressed)|{
            let root_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();
            let button_entities_presseds=entities_presseds.get(&button);
            let entities_presseds_contains=button_entities_presseds.and_then(|button_entities_presseds|button_entities_presseds.get(&pressed_entity))
                .map(|device_types|device_types.contains(&device_type)).unwrap_or_default();

            // let is_pressable=pressable_query.get(pressed_entity).map(|(_,pressable)|pressable.enable).unwrap_or_default();
            // let q=button_entities_presseds.get(&pressed_entity).map(|x|x.contains(&device_type)).unwrap_or_default();
            // && is_pressable

            root_alive && entities_presseds_contains
        });

        !button_device_presseds.is_empty()
    });

    //entities_presseds

    //unpress
    for (&button,button_entities_presseds) in entities_presseds.iter_mut() {
    //     button_entities_presseds.retain(|&(root_entity,pressed_entity),device_types|{

    //         !device_types.is_empty()
    //     });
        for pressed_entity in button_entities_presseds.keys().cloned().collect::<Vec<_>>() //(root_entity,pressed_entity)
        {
            let pressable=pressable_query.get_mut(pressed_entity).map(|x|x.1);
            // focuseds.
            //focus pressed but unfocused
            //  ... removed

            // let root_entity=layout_computed_query.get(pressed_entity).unwrap().root_entity;
            //layout_computed couldve been removed from entity in meantime
            let (root_entity,unlocked)=layout_computed_query.get(pressed_entity).map(|x|(Some(x.root_entity),x.unlocked)).unwrap_or_default();

            //inactive/disabled/invisible/no_devices/
            let root_entity_alive=root_entity.and_then(|root_entity|root_query.get(root_entity).ok()).map(|(_,computed)|computed.unlocked).unwrap_or_default();
            // let unlocked=layout_computed_query.get(pressed_entity).map(|x|x.unlocked).unwrap_or_default();
            let pressable_enable=pressable.as_ref().map(|x|x.enable).unwrap_or_default();

            if !root_entity_alive || !unlocked || !pressable_enable //|| no_devices_pressed
            {
                let entity_presseds= button_entities_presseds.remove(&pressed_entity).unwrap(); //(root_entity,pressed_entity)

                if !entity_presseds.is_empty() {
                    ui_output_event_writer.write(UiInteractEvent{entity:pressed_entity,event_type:UiInteractMessageType::PressEnd{ device:99, button }});
                }

                // if let Ok(mut pressable)=pressable {
                //     pressable.pressed=false;
                // }

                //does device_presseds need to be cleared too?
            }
        }
    }


    // //when pressable.pressed=true
    // for (entity,pressable) in pressable_query.iter_mut() {
    //     //
    //     if !pressable.pressed {
    //         continue;
    //     }

    //     //
    //     let unlocked=layout_computed_query.get(entity).map(|x|x.unlocked).unwrap_or_default();

    //     if !unlocked {
    //         continue;
    //     }

    //     //
    //     let root_entity=parent_query.iter_ancestors(entity).last().unwrap();
    //     let entity_presseds=entities_presseds.entry((root_entity,entity)).or_default();

    //     if entity_presseds.contains(&None) {
    //         continue;
    //     }

    //     //
    //     if is_no_entity_pressed(entity_presseds,&device_presseds,root_entity,pressable.always) {
    //         ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractEventType::PressBegin});
    //     }

    //     //
    //     entity_presseds.insert(None); //represents a device for pressable.pressed
    // }

    //
    let mut roots_pressable_entities: HashMap<Entity, Vec<Entity>> = HashMap::new(); //[root_entity]=pressable_entities

    //get root entities with their pressable descendants
    for (pressable_entity,pressable) in pressable_query.iter() {
        if !pressable.enable {
            continue;
        }

        let Ok(computed) = layout_computed_query.get(pressable_entity) else { continue; };

        if !computed.unlocked {
            continue;
        }

        roots_pressable_entities.entry(computed.root_entity).or_default().push(pressable_entity);
    }

    //sort press_root_entities by computed.order
    for (&_root_entity, root_pressable_entities) in roots_pressable_entities.iter_mut() {
        root_pressable_entities.sort_by(|&a,&b|{
            let computed_a = layout_computed_query.get(a).unwrap();
            let computed_b = layout_computed_query.get(b).unwrap();
            computed_a.order.cmp(&computed_b.order).reverse()
        });
    }

    //

    // //handle cursor pressed on button, but no longer on the button
    // for (&button,button_device_presseds) in device_presseds.iter_mut() {
    //     for (&(root_entity,device_type),(pressed_entity,is_pressed)) in button_device_presseds.iter_mut() {
    //         let pressed_entity=*pressed_entity;
    //         let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap();

    //         // if !is_cursor {
    //         //     continue;
    //         // }

    //         match device_type {
    //             DeviceType::Cursor(device)=>{
    //                 if pressable.always && *is_pressed==false {
    //                     // ui_output_event_writer.write(UiEvent{entity,event_type:UiEventType::PressBegin});
    //                     // *pressed=true;
    //                 } else if !pressable.always {
    //                     if let Some(&cursor)=device_cursors.get(&(root_entity,device)) {
    //                         let computed = layout_computed_query.get(pressed_entity).unwrap();
    //                         let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
    //                         let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor);

    //                         if cursor_inside && *is_pressed==false {
    //                             ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressBegin{ device, button }});
    //                             *is_pressed=true;
    //                         } else if !cursor_inside && *is_pressed==true {
    //                             ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
    //                             *is_pressed=false;
    //                         }
    //                     } else if *is_pressed==true {
    //                         ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
    //                         *is_pressed=false;
    //                     }
    //                 }
    //             }
    //             DeviceType::Focus(device) => {
    //                 // let focused = focusable_query.get(entity).map(|x|x.focused).unwrap_or_default();
    //                 let focused = focuseds.0.get(&device).map(|device_focuseds|device_focuseds.contains(&pressed_entity)).unwrap_or_default();

    //                 if focused && *is_pressed==false {
    //                     if !pressable.always {
    //                         ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressBegin{ device, button }});
    //                     }

    //                     *is_pressed=true;
    //                 } else if !focused && *is_pressed==true { //re enable to work like cursor when pressed+unfocus
    //                     if !pressable.always {
    //                         ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
    //                     }

    //                     *is_pressed=false;
    //                 }
    //             }
    //         }
    //     }
    // }


    //
    for ev in input_event_reader.read() {
        //
        if !ev.get_root_entity()
            .and_then(|root_entity|root_query.get(root_entity).ok())
            .map(|(_,computed)|computed.unlocked)
            .unwrap_or_default()
        {
            continue;
        }

        //
        match ev.clone() {
            UiInteractInputMessage::CursorMoveTo{root_entity,device,cursor} => {
                if let Some(cursor)=cursor {
                    device_cursors.insert((root_entity,device),cursor);
                } else {
                    device_cursors.remove(&(root_entity,device));
                }

                //
                for (button,pressed_entity,is_pressed) in device_presseds.iter_mut()
                    .filter_map(|(&button,button_device_presseds)|button_device_presseds.get_mut(&(root_entity,DeviceType::Cursor(device))).map(|x|(button,x)))
                    .map(|(button,x)|(button,x.0,&mut x.1))
                {
                    //
                    let pressable_always=pressable_query.get(pressed_entity).map(|x|x.1.always).unwrap(); //can use unwrap otherwise won't be in device_presseds

                    //
                    let cursor_inside= cursor.map(|cursor|{
                        let layout_computed = layout_computed_query.get(pressed_entity).unwrap(); //can use unwrap otherwise won't be in device_presseds
                        let outer_rect=layout_computed.clamped_border_rect();//.clamped_padding_rect();
                        let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor);
                        cursor_inside
                    }).unwrap_or_default();

                    //handle mouse pressed and button prev pressed and moving on/off button
                    if cursor_inside && !(*is_pressed) {
                        *is_pressed=true;

                        if !pressable_always {
                            ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressBegin{ device, button }});
                        }
                    } else if !cursor_inside && *is_pressed {
                        *is_pressed=false;

                        if !pressable_always {
                            ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                        }
                    }
                }
            }

            //if same device used for cursor and button press, then a depress of one will depress the other
            UiInteractInputMessage::CursorPressBegin{root_entity,device, button } => {
                let cursor = device_cursors.get(&(root_entity,device)).cloned();
                let device_type=DeviceType::Cursor(device);

                //already pressed on an entity
                if device_presseds.get(&button).map(|button_device_presseds|{
                    button_device_presseds.contains_key(&(root_entity,device_type))
                }).unwrap_or_default() {
                    continue;
                }

                //get entity cursor is on
                let pressable_entity=cursor.and_then(|cursor|{
                    roots_pressable_entities.get(&root_entity).and_then(|pressable_entities|{
                        pressable_entities.iter().find(|&&entity|{
                            let computed = layout_computed_query.get(entity).unwrap();
                            let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                            let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor);
                            cursor_inside
                        })
                    })
                }).cloned();

                //
                if let Some(entity)=pressable_entity {
                    ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::PressBegin{ device, button }});

                    device_presseds.entry(button).or_default()
                        .insert((root_entity,device_type),(entity,true));

                    entities_presseds.entry(button).or_default()
                        .entry(entity).or_default()
                        .insert(device_type);
                }
            }

            UiInteractInputMessage::FocusPressBegin{root_entity,group,device, button } => {
                let device_type=DeviceType::Focus(device);

                //already pressed on an entity
                if device_presseds.get(&button).map(|button_device_presseds|{
                    button_device_presseds.contains_key(&(root_entity,device_type))
                }).unwrap_or_default() {
                    continue;
                }

                let focused_entity=focus_states.cur_focuses.get(&device)
                    .and_then(|device_focus_states|device_focus_states.get(&root_entity))
                    .and_then(|root_focus_states|root_focus_states.get(&group))
                    .and_then(|(cur_focus_entity,_,_)|cur_focus_entity.clone());

                //
                if let Some(entity)=focused_entity {
                    ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::PressBegin{ device, button }});

                    device_presseds.entry(button).or_default()
                        .insert((root_entity,device_type),(entity,true));

                    entities_presseds.entry(button).or_default()
                        .entry(entity).or_default()
                        .insert(device_type);
                }
            }
            UiInteractInputMessage::CursorPressEnd{root_entity,device, button }
                |UiInteractInputMessage::FocusPressEnd{root_entity,device, button }
            => {
                let device_type=ev.device_type();

                if let Some((pressed_entity,is_pressed))=device_presseds.get_mut(&button)
                    .and_then(|button_device_presseds|button_device_presseds.remove(&(root_entity,device_type)))
                {
                    entities_presseds.get_mut(&button)
                        .and_then(|button_entities_presseds|button_entities_presseds.get_mut(&pressed_entity))
                        .and_then(|entity_presseds|entity_presseds.remove(&device_type).then_some(()))
                        .unwrap();

                    //
                    let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap(); //can use unwrap, wouldn't be in device_presseds otherwise

                    //
                    if pressable.always || is_pressed { //always means it will always be pressed (when cursor/focus is no longer on entity)
                        ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                    }

                    if is_pressed {
                        ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::Click{ device, button }});
                    }
                }

            }

            UiInteractInputMessage::CursorPressCancel{root_entity,device, button }
                |UiInteractInputMessage::FocusPressCancel{root_entity,device, button }
            => {
                let device_type=ev.device_type();

                if let Some((pressed_entity,is_pressed))=device_presseds.get_mut(&button).and_then(|button_device_presseds|button_device_presseds.remove(&(root_entity,device_type))) {
                    entities_presseds.get_mut(&button)
                        .and_then(|button_entities_presseds|button_entities_presseds.get_mut(&pressed_entity))
                        .and_then(|entity_presseds|entity_presseds.remove(&device_type).then_some(()))
                        .unwrap();

                    //
                    let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap(); //can use unwrap, wouldn't be in device_presseds otherwise

                    if pressable.always || is_pressed { //always means it will always be pressed (when cursor/focus is no longer on entity)
                        ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                    }
                }
            }

            _=>{}
        } //match
    } //for
}