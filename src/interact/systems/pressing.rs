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

fn is_no_entity_pressed(
    entity_presseds:&HashSet<DeviceType>, //[device_type]
    device_presseds : &HashMap<(Entity,DeviceType),(Entity,bool)>, //[(root_entity,device_type)]=(pressed_entity,pressed)
    root_entity : Entity,
    press_always : bool,
) -> bool {
    entity_presseds.iter().fold(true, |prev,&device_type|{
        prev && !device_presseds.get(&(root_entity,device_type)).unwrap().1 && !press_always
    })
}


pub fn update_press_events(
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    layout_computed_query: Query<&UiLayoutComputed>,
    mut pressable_query: Query<(Entity,&mut UiPressable)>,

    focus_states : Res<UiFocusStates>,
    focuseds : Res<UiFocuseds>,

    // mut press_states:ResMut<UiPressStates>,
    mut device_cursors : Local<HashMap<(Entity,i32),Vec2>>, //[(root_entity,device)]=cursor
    mut device_presseds : Local<HashMap<i32,HashMap<(Entity,DeviceType),(Entity,bool)>>>,     //[button][device_type]=(pressed_entity,is_pressed)
    mut entities_presseds : Local<HashMap<i32,HashMap<Entity,HashSet<DeviceType>>>>, //[button][press_entity][device_type]
    // mut entities_presseds : Local<HashMap<i32,HashMap<(Entity,Entity),HashSet<DeviceType>>>>, //[button][(root_entity,press_entity)][device_type]


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
        button_entities_presseds.retain(|&pressed_entity,_|{ //(root_entity,pressed_entity)
            let root_entity=layout_computed_query.get(pressed_entity).unwrap().root_entity;
            let root_unlocked= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();
            let pressable=pressable_query.get(pressed_entity).map(|(_,pressable)|pressable.enable).unwrap_or_default();
            root_unlocked && pressable
        });

        !button_entities_presseds.is_empty()
    });

    //remove dead roots/pressed entities from device_presseds
    device_presseds.retain(|&button,button_device_presseds|{
        button_device_presseds.retain(|&(root_entity,device_type),&mut (pressed_entity,_pressed)|{
            let button_entities_presseds=entities_presseds.get(&button).unwrap();
            let root_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();
            let is_pressable=pressable_query.get(pressed_entity).map(|(_,pressable)|pressable.enable).unwrap_or_default();
            let q=button_entities_presseds.get(&pressed_entity).map(|x|x.contains(&device_type)).unwrap_or_default();
            root_alive && is_pressable && q
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
    let mut roots_pressable_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    //get root entities with their pressable descendants
    for (entity,pressable) in pressable_query.iter() {
        if !pressable.enable {
            continue;
        }

        // if !parent_query.contains(entity) { //roots can't be pressable
        //     continue;
        // }

        let Ok(computed) = layout_computed_query.get(entity) else {
            continue;
        };

        if !computed.unlocked {
            continue;
        }

        // // let root_entity=parent_query.iter_ancestors(entity).last().unwrap();

        // let Some(root_entity)=[entity].into_iter().chain(parent_query.iter_ancestors(entity)).find(|&ancestor_entity|root_query.contains(ancestor_entity)) else {
        //     continue;
        // };

        roots_pressable_entities.entry(computed.root_entity).or_default().push(entity);
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

    //handle cursor pressed on button, but no longer on the button
    for (&button,button_device_presseds) in device_presseds.iter_mut() {
        for (&(root_entity,device_type),(pressed_entity,is_pressed)) in button_device_presseds.iter_mut() {
            let pressed_entity=*pressed_entity;
            let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap();

            // if !is_cursor {
            //     continue;
            // }

            match device_type {
                DeviceType::Cursor(device)=>{
                    if pressable.always && *is_pressed==false {
                        // ui_output_event_writer.write(UiEvent{entity,event_type:UiEventType::PressBegin});
                        // *pressed=true;
                    } else if !pressable.always {
                        if let Some(&cursor)=device_cursors.get(&(root_entity,device)) {
                            let computed = layout_computed_query.get(pressed_entity).unwrap();
                            let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                            let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor);

                            if cursor_inside && *is_pressed==false {
                                ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressBegin{ device, button }});
                                *is_pressed=true;
                            } else if !cursor_inside && *is_pressed==true {
                                ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                                *is_pressed=false;
                            }
                        } else if *is_pressed==true {
                            ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                            *is_pressed=false;
                        }
                    }
                }
                DeviceType::Focus(device) => {
                    // let focused = focusable_query.get(entity).map(|x|x.focused).unwrap_or_default();
                    let focused = focuseds.0.get(&device).map(|device_focuseds|device_focuseds.contains(&pressed_entity)).unwrap_or_default();

                    if focused && *is_pressed==false {
                        if !pressable.always {
                            ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressBegin{ device, button }});
                        }

                        *is_pressed=true;
                    } else if !focused && *is_pressed==true { //re enable to work like cursor when pressed+unfocus
                        if !pressable.always {
                            ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                        }

                        *is_pressed=false;
                    }
                }
            }
        }
    }


    //
    for ev in input_event_reader.read() {

        // if !root_query.get(ev.get_root_entity()).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
        //     continue;
        // }

        if !ev.get_root_entity()
            .and_then(|root_entity|root_query.get(root_entity).ok())
            .map(|(_,computed)|computed.unlocked)
            .unwrap_or_default()
        {
            continue;
        }

        match ev.clone() {
            UiInteractInputMessage::CursorMoveTo{root_entity,device,cursor} => {
                if let Some(cursor)=cursor {
                    device_cursors.insert((root_entity,device),cursor);
                } else {
                    device_cursors.remove(&(root_entity,device));
                }

                for (&button,button_device_presseds) in device_presseds.iter_mut() {
                    if let Some((pressed_entity,is_pressed)) = button_device_presseds.get_mut(&(root_entity,DeviceType::Cursor(device))) {
                        let pressed_entity=*pressed_entity;
                        // let entity_presseds=entities_presseds.entry((root_entity,entity)).or_default();
                        let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap();

                        if let Some(&cursor)=device_cursors.get(&(root_entity,device)) {
                            let computed = layout_computed_query.get(pressed_entity).unwrap();
                            let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                            let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor);

                            if cursor_inside && *is_pressed==false {
                                *is_pressed=true;

                                if !pressable.always {
                                    // if entity_presseds.len() == 1 {
                                    ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressBegin{ device, button }});
                                    // }
                                }

                            } else if !cursor_inside && *is_pressed==true {
                                *is_pressed=false;

                                if !pressable.always {
                                    ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                                }
                            }
                        } else if *is_pressed==true {
                            *is_pressed=false;

                            if !pressable.always {
                                ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                            }

                        }
                    }
                }

            }

            //if same device used for cursor and button press, then a depress of one will depress the other
            UiInteractInputMessage::CursorPressBegin{root_entity,device, button } => {
                let Some(entities)=roots_pressable_entities.get(&root_entity) else {
                    continue;
                };

                let cursor = device_cursors.get(&(root_entity,device)).cloned();

                //device can't press if it is alway pressed
                let button_device_presseds=device_presseds.entry(button).or_default();
                let device_type=DeviceType::Cursor(device);

                if button_device_presseds.get(&(root_entity,device_type)).is_none() && cursor.is_some() {
                    for &entity in entities.iter() {
                        let computed = layout_computed_query.get(entity).unwrap();
                        let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                        let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor.unwrap());

                        if cursor_inside { //begin press
                            let button_entities_presseds=entities_presseds.entry(button).or_default();
                            let entity_presseds=button_entities_presseds.entry(entity).or_default(); //(root_entity,entity)
                            let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
                            let no_entity_pressed=is_no_entity_pressed(entity_presseds,&button_device_presseds,root_entity,pressable.always);

                            if no_entity_pressed {
                                ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::PressBegin{ device, button }});
                            }

                            button_device_presseds.insert((root_entity,device_type),(entity,true));
                            entity_presseds.insert(device_type);
                            break;
                        }
                    }
                }
            }

            UiInteractInputMessage::FocusPressBegin{root_entity,group,device, button } => {
                let device_type=DeviceType::Focus(device);

                let Some(entity)=focus_states.cur_focuses.get(&device)
                    .and_then(|device_focuses|device_focuses.get(&root_entity))
                    .and_then(|root_focuses|root_focuses.get(&group))
                    .and_then(|(focused_entity,_,_)|focused_entity.clone())
                else {
                    continue;
                };

                // let Some(&entity)=press_states.focuseds.get(&(root_entity,group)) else {
                //     continue;
                // };

                if !pressable_query.contains(entity) {
                    continue;
                }

                //device can't press if it is alway pressed

                let button_device_presseds=device_presseds.entry(button).or_default();

                if button_device_presseds.get(&(root_entity,device_type)).is_none() {
                    let button_entities_presseds=entities_presseds.entry(button).or_default();

                    let entity_presseds=button_entities_presseds.entry(entity).or_default(); //(root_entity,entity)
                    let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
                    let no_entity_pressed=is_no_entity_pressed(entity_presseds,&button_device_presseds,root_entity,pressable.always);

                    if no_entity_pressed {
                        ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::PressBegin{ device, button }});
                    }

                    button_device_presseds.insert((root_entity,device_type),(entity,true));
                    entity_presseds.insert(device_type);
                }
            }
            UiInteractInputMessage::CursorPressEnd{root_entity,device, button }
                |UiInteractInputMessage::FocusPressEnd{root_entity,device, button }
            => {
                let device_type=if let UiInteractInputMessage::CursorPressEnd{..}=&ev {
                    DeviceType::Cursor(device)
                } else {
                    DeviceType::Focus(device)
                };

                if let (
                    Some(button_device_presseds),
                    Some(button_entities_presseds),
                )=(
                    device_presseds.get_mut(&button),
                    entities_presseds.get_mut(&button),
                ) {
                    let mut to_unpress=None;

                    if let Some((entity,pressed))=button_device_presseds.remove(&(root_entity,device_type)) {
                        if let Some(entity_presseds)=button_entities_presseds.get_mut(&entity) { //(root_entity,entity)
                            entity_presseds.remove(&device_type);

                            let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
                            let no_entity_pressed=is_no_entity_pressed(entity_presseds,&button_device_presseds,root_entity,pressable.always);

                            if !pressable.physical || no_entity_pressed {
                                let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();

                                if pressable.always || pressed { //always means it will always be pressed (when cursor/focus is no longer on entity)
                                    ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                                }

                                if pressed {
                                    ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::Click{ device, button }});
                                }

                                if pressed || no_entity_pressed {
                                    to_unpress=Some(entity);
                                }
                            }
                        }
                    }

                    if let Some(pressed_entity)=to_unpress { //removes all devices from button_entities_presseds? why? its only when no other device pressed on it, via no_entity_pressed
                        button_entities_presseds.remove(&pressed_entity).unwrap(); //(root_entity,pressed_entity)
                    }
                }
            }

            UiInteractInputMessage::CursorPressCancel{root_entity,device, button }
                |UiInteractInputMessage::FocusPressCancel{root_entity,device, button }
            => {
                let device_type=if let UiInteractInputMessage::CursorPressEnd{..}=&ev {
                    DeviceType::Cursor(device)
                } else {
                    DeviceType::Focus(device)
                };

                if let (
                    Some(button_device_presseds),
                    Some(button_entities_presseds),
                )=(
                    device_presseds.get_mut(&button),
                    entities_presseds.get_mut(&button),
                ) {
                    if let Some((pressed_entity,is_pressed)) = button_device_presseds.remove(&(root_entity,device_type)) {
                        if let Some(entity_presseds)=button_entities_presseds.get_mut(&pressed_entity) { //(root_entity,pressed_entity)
                            let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap();
                            entity_presseds.remove(&device_type); //may or not contain

                            if pressable.always || is_pressed { //always means it will always be pressed (when cursor/focus is no longer on entity)
                                ui_output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::PressEnd{ device, button }});
                            }
                        }

                        if button_entities_presseds.get(&pressed_entity) //(root_entity,pressed_entity)
                            .map(|entity_presseds|entity_presseds.is_empty()).unwrap() //can just unwrap?
                        {
                            button_entities_presseds.remove(&pressed_entity).unwrap(); //(root_entity,pressed_entity)
                        }
                    }
                }
            }

            _=>{}
        } //match
    } //for
}