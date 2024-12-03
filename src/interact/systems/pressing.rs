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
*/

use std::collections::{HashMap, HashSet};

use bevy::ecs::prelude::*;
use bevy::hierarchy::prelude::*;
use bevy::math::Vec2;

use super::super::components::*;
use super::super::resources::*;
use super::super::events::*;
// use super::super::utils::*;
// use super::super::values::*;

use super::super::super::layout::components::UiLayoutComputed;

fn is_no_entity_pressed(
    entity_presseds:&HashSet<Option<(i32,bool)>>,
    device_presseds : &HashMap<(Entity,i32,bool),(Entity,bool)>,
    root_entity : Entity,
    press_always : bool,
) -> bool {
    let r=entity_presseds.iter().fold(true, |prev,entity_pressed|{
        let not_pressed=if let Some((device,is_cursor))=*entity_pressed {
            !(device_presseds.get(&(root_entity,device,is_cursor)).unwrap().1||press_always)
        } else {
            false
        };
        
        // let not_pressed=!entity_pressed.is_none() && !entity_pressed.unwrap().1;
        prev && not_pressed
    });


    r
}

// pub fn pre_update_press(
    
//     parent_query : Query<&Parent,With<UiLayoutComputed>>,
//     mut interact_event_reader: EventReader<UiInteractEvent>,

//     mut press_states:ResMut<UiPressStates>,

// ) {
//     for ev in interact_event_reader.read() {
//         match ev.event_type {
//             UiInteractEventType::FocusBegin { group } => {
//                 let root_entity=parent_query.iter_ancestors(ev.entity).last().unwrap_or(ev.entity);
//                 press_states.focuseds.insert((root_entity,group),ev.entity);
//             }
//             UiInteractEventType::FocusEnd { group } => {
//                 let root_entity=parent_query.iter_ancestors(ev.entity).last().unwrap_or(ev.entity);
//                 press_states.focuseds.remove(&(root_entity,group)).unwrap(); //.then_some(()).unwrap();
//             }
//             _ =>{}
//         }
//     }
// }


pub fn update_press_events(
    root_query: Query<Entity,(Without<Parent>,With<UiLayoutComputed>)>,
    parent_query : Query<&Parent,With<UiLayoutComputed>>,
    layout_computed_query: Query<&UiLayoutComputed>, //,With<UiPressable>
    mut pressable_query: Query<(Entity,&mut UiPressable)>,

    mut device_cursors : Local<HashMap<(Entity,i32),Vec2>>, //[(root_entity,device)]=cursor
    // mut device_presseds : Local<HashMap<(Entity,i32,bool),(Entity,bool)>>, //[(root_entity,device,is_cursor)]=(pressed_entity,pressed)
    mut entities_presseds : Local<HashMap<(Entity,Entity),HashSet<Option<(i32,bool)>>>>, //[(root_entity,press_entity)]=set<Some((device,is_cursor))>, the None is a separate device representing pressable.pressed

    mut press_states:ResMut<UiPressStates>,

    focusable_query : Query<&UiFocusable>,    
    focus_states : Res<UiFocusStates>,

    mut input_event_reader: EventReader<UiInteractInputEvent>,
    mut ui_output_event_writer: EventWriter<UiInteractEvent>,


) {
    // let use_physical = false; //if one device is holding press, then other devices cannot unpress/click

    //remove dead roots/pressed entities

    device_cursors.retain(|&(root_entity,_device),_|{
        root_query.contains(root_entity)
    });

    entities_presseds.retain(|&(root_entity,pressed_entity),_|{
        root_query.contains(root_entity) && pressable_query.contains(pressed_entity)
    });

    press_states.device_presseds.retain(|&(root_entity,device,is_cursor),(pressed_entity,_pressed)|{
        root_query.contains(root_entity) &&
            entities_presseds.get(&(root_entity,*pressed_entity))
                .map(|x|x.contains(&Some((device,is_cursor))))
                .unwrap_or_default()
        // && pressable_query.contains(*pressed_entity)
    });

    //unpress
    for (root_entity,pressed_entity) in entities_presseds.keys().cloned().collect::<Vec<_>>() {
        let pressable=pressable_query.get_mut(pressed_entity).map(|x|x.1);

        // //pressable.entity_presseds==false but previously true
        // if let Ok(mut pressable)=pressable.as_ref() {
        //     let entity_presseds=entities_presseds.get_mut(&(root_entity,pressed_entity)).unwrap();

        //     if !pressable.pressed &&
        //         entity_presseds.remove(&None) 
        //         && is_no_entity_pressed(entity_presseds,&device_presseds,root_entity,pressable.always) 
        //     {
        //         ui_output_event_writer.send(UiInteractEvent{entity:pressed_entity,event_type:UiInteractEventType::PressEnd});
        //     }
        // }

        //focus pressed but unfocused
        //  ... removed
        
        //inactive/disabled/invisible/no_devices/
        let root_entity_alive=layout_computed_query.get(root_entity).is_ok();
        let unlocked=layout_computed_query.get(pressed_entity).map(|x|x.unlocked).unwrap_or_default();

        let pressable_enable=pressable.as_ref().map(|x|x.enable).unwrap_or_default();
        //let pressable_pressed=pressable.as_ref().map(|x|x.pressed).unwrap_or_default();
        
        // let entity_presseds=entities_presseds.get(&(root_entity,pressed_entity)).unwrap();
        // let no_devices_pressed=entity_presseds.is_empty();//is_no_entity_pressed(entity_presseds);//;
        
        if !root_entity_alive || !unlocked || !pressable_enable //|| no_devices_pressed 
        {
            let entity_presseds= entities_presseds.remove(&(root_entity,pressed_entity)).unwrap();

            if !entity_presseds.is_empty() {
                ui_output_event_writer.send(UiInteractEvent{entity:pressed_entity,event_type:UiInteractEventType::PressEnd});
            }

            // if let Ok(mut pressable)=pressable {
            //     pressable.pressed=false;
            // }

            //does device_presseds need to be cleared too?
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
    //         ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressBegin});
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

        if !parent_query.contains(entity) { //roots can't be pressable
            continue;
        }

        let Ok(computed) = layout_computed_query.get(entity) else {
            continue;
        };

        if !computed.unlocked {
            continue;
        }

        let root_entity=parent_query.iter_ancestors(entity).last().unwrap();
        roots_pressable_entities.entry(root_entity).or_default().push(entity);
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
    for (&(root_entity,device,is_cursor),(entity,pressed)) in press_states.device_presseds.iter_mut() {
        let entity=*entity;
        let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();

        // if !is_cursor {
        //     continue;
        // }

        if is_cursor {
            if pressable.always && *pressed==false {
                // ui_output_event_writer.send(UiEvent{entity,event_type:UiEventType::PressBegin});
                // *pressed=true;
            } else if !pressable.always {
                if let Some(&cursor)=device_cursors.get(&(root_entity,device)) {
                    let computed = layout_computed_query.get(entity).unwrap();
                    let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                    let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor);
        
                    if cursor_inside && *pressed==false {
                        ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressBegin});
                        *pressed=true;
                    } else if !cursor_inside && *pressed==true {
                        ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                        *pressed=false;
                    }
                } else if *pressed==true {
                    ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                    *pressed=false;
                }
            }
        } else { //focus
            let focused = focusable_query.get(entity).map(|x|x.focused).unwrap_or_default();
            
            if focused && *pressed==false {
                if !pressable.always {
                    ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressBegin});
                }

                *pressed=true;
            } else if !focused && *pressed==true { //re enable to work like cursor when pressed+unfocus
                if !pressable.always {
                    ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                }

                *pressed=false;
            }
        }
    }

    //
    for ev in input_event_reader.read() {
        match ev.clone() {
            UiInteractInputEvent::CursorMoveTo{root_entity,device,cursor} => {
                if let Some(cursor)=cursor {
                    device_cursors.insert((root_entity,device),cursor);
                } else {
                    device_cursors.remove(&(root_entity,device));
                }
                
                if let Some((entity,pressed)) = press_states.device_presseds.get_mut(&(root_entity,device,true)) {
                    let entity=*entity;
                    // let entity_presseds=entities_presseds.entry((root_entity,entity)).or_default();
                    let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
                 

                    if let Some(&cursor)=device_cursors.get(&(root_entity,device)) {
                        let computed = layout_computed_query.get(entity).unwrap();
                        let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                        let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor);
            
                        if cursor_inside && *pressed==false {
                            *pressed=true;

                            if !pressable.always {
                                // if entity_presseds.len() == 1 {
                                ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressBegin});
                                // }
                            }

                        } else if !cursor_inside && *pressed==true {
                            *pressed=false;

                            if !pressable.always { 
                                ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                            }
                        }
                    } else if *pressed==true {
                        *pressed=false;

                        if !pressable.always { 
                            ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                        }

                    }
                }
            }

            //if same device used for cursor and button press, then a depress of one will depress the other
            UiInteractInputEvent::CursorPressBegin{root_entity,device} => {
                let Some(entities)=roots_pressable_entities.get(&root_entity) else {
                    continue;
                };
        
                let cursor = device_cursors.get(&(root_entity,device)).cloned();

                //device can't press if it is alway pressed
                if press_states.device_presseds.get(&(root_entity,device,true)).is_none() && cursor.is_some() {
                    for &entity in entities.iter() {
                        let computed = layout_computed_query.get(entity).unwrap();
                        let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                        let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor.unwrap());
    
                        if cursor_inside { //begin press
                            let entity_presseds=entities_presseds.entry((root_entity,entity)).or_default();
                            let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
                            let no_entity_pressed=is_no_entity_pressed(entity_presseds,&press_states.device_presseds,root_entity,pressable.always);

                            if no_entity_pressed {
                                ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressBegin});
                            }

                            press_states.device_presseds.insert((root_entity,device,true),(entity,true));
                            entity_presseds.insert(Some((device,true)));
                            break;
                        }
                    }
                }
            }
            UiInteractInputEvent::CursorPressEnd{root_entity,device} => {
                let mut to_unpress=None;

                if let Some((entity,pressed))=press_states.device_presseds.remove(&(root_entity,device,true)) {
                    if let Some(entity_presseds)=entities_presseds.get_mut(&(root_entity,entity)) {
                        entity_presseds.remove(&Some((device,true)));

                        let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
                        let no_entity_pressed=is_no_entity_pressed(entity_presseds,&press_states.device_presseds,root_entity,pressable.always);

                        if !pressable.physical || no_entity_pressed {
                            let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();

                            if pressable.always || pressed {
                                ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                            }

                            if pressed {
                                ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::Click});
                            }

                            if pressed || no_entity_pressed {
                                to_unpress=Some(entity);
                            }
                        }
                    }
                }

                if let Some(pressed_entity)=to_unpress {
                    entities_presseds.remove(&(root_entity,pressed_entity)).unwrap();
                }
            }
            UiInteractInputEvent::CursorPressCancel{root_entity,device} => {
                let mut to_unpress=None;

                if let Some(&(entity,pressed)) = press_states.device_presseds.get(&(root_entity,device,true)) {
                    if let Some(entity_presseds)=entities_presseds.get_mut(&(root_entity,entity)) {
                        let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();

                        entity_presseds.remove(&Some((device,true)));

                        if pressable.always || pressed { //always means it will always be pressed (when cursor/focus is no longer on entity)
                            ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                        }
                        
                        to_unpress=Some(entity);
                    }
                }
                
                press_states.device_presseds.remove(&(root_entity,device,true));

                if let Some(pressed_entity)=to_unpress {
                    entities_presseds.remove(&(root_entity,pressed_entity)).unwrap();
                }
            }
            UiInteractInputEvent::FocusPressBegin{root_entity,group,device} => {
                let Some(entity)=focus_states.cur_focuses.get(&root_entity).and_then(|x|x.get(&group)).and_then(|x|x.0) else {
                    continue;
                };

                // let Some(&entity)=press_states.focuseds.get(&(root_entity,group)) else {
                //     continue;
                // };

                if !pressable_query.contains(entity) {
                    continue;
                }

                //device can't press if it is alway pressed
                if press_states.device_presseds.get(&(root_entity,device,false)).is_none() {
                    
                    let entity_presseds=entities_presseds.entry((root_entity,entity)).or_default();
                    let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
                    let no_entity_pressed=is_no_entity_pressed(entity_presseds,&press_states.device_presseds,root_entity,pressable.always);

                    if no_entity_pressed {
                        ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressBegin});
                    }

                    press_states.device_presseds.insert((root_entity,device,false),(entity,true));
                    entity_presseds.insert(Some((device,false)));
                }
            }
            UiInteractInputEvent::FocusPressEnd{root_entity,device} => {
                let mut to_unpress=None;

                if let Some((entity,pressed))=press_states.device_presseds.remove(&(root_entity,device,false)) {
                    if let Some(entity_presseds)=entities_presseds.get_mut(&(root_entity,entity)) {
                        entity_presseds.remove(&Some((device,false)));

                        let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
                        let no_entity_pressed=is_no_entity_pressed(entity_presseds,&press_states.device_presseds,root_entity,pressable.always);
                    
                        if !pressable.physical || no_entity_pressed {
                            let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();

                            if pressable.always || pressed { //always means it will always be pressed (when cursor/focus is no longer on entity)
                                ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                            }

                            if pressed {
                                ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::Click});
                            }

                            if pressed || no_entity_pressed {
                                to_unpress=Some(entity);
                            }
                        }
                    }
                }
                
                if let Some(pressed_entity)=to_unpress {
                    entities_presseds.remove(&(root_entity,pressed_entity)).unwrap();
                }
            }
            UiInteractInputEvent::FocusPressCancel{root_entity,device} => {
                let mut to_unpress=None;

                if let Some(&(entity,pressed)) = press_states.device_presseds.get(&(root_entity,device,false)) {
                    if let Some(entity_presseds)=entities_presseds.get_mut(&(root_entity,entity)) {
                        entity_presseds.remove(&Some((device,false)));

                        let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();

                        if pressable.always || pressed {
                            ui_output_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::PressEnd});
                        }
                        
                        to_unpress=Some(entity);
                    }
                }
                
                press_states.device_presseds.remove(&(root_entity,device,false));

                if let Some(pressed_entity)=to_unpress {
                    entities_presseds.remove(&(root_entity,pressed_entity)).unwrap();
                }
            }
            _=>{}
        } //match
    } //for
}