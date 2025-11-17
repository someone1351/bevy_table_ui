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

use std::collections::HashMap;

use bevy::ecs::prelude::*;
// use bevy::hierarchy::prelude::*;
use bevy::math::Vec2;

use crate::UiRect;

use super::super::components::*;
use super::super::resources::*;
use super::super::messages::*;
// use super::super::utils::*;
// use super::super::values::*;

use super::super::super::layout::components::{UiLayoutComputed,UiRoot};


pub fn hover_cleanup(
    hoverable_query: Query<(Entity,&UiLayoutComputed,&UiPressable)>,
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    mut ui_event_writer: MessageWriter<UiInteractEvent>,
    // mut cur_hover_entities : Local<HashMap<(Entity,i32),(Entity,Vec2)>>, //[(root_entity,device)]=cur_hover_entity
    mut cur_hover_entities : ResMut<CursorHover>, //[(root_entity,device)]=cur_hover_entity
) {

    //un hover inactive/disabled/invisible, and cursor no longer inside due to node pos/size change
    //remove inactive root nodes

    cur_hover_entities.0.retain(|&(root_entity,device),&mut (entity,cursor)|{
        let root_alive = root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();
        let hoverable_alive = hoverable_query.get(entity).map(|(_,layout_computed,hoverable)|{
            hoverable.enable && layout_computed.unlocked && layout_computed.clamped_border_rect().contains_point(cursor)
        }).unwrap_or_default();

        // if let Ok((_,layout_computed,hoverable)) = hoverable_query.get(*entity) {
        //     if hoverable.enable && layout_computed.unlocked && layout_computed.clamped_border_rect().contains_point(*cursor) {
        //         return true;
        //     }
        // }

        if root_alive && hoverable_alive {
            true
        } else {
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::HoverEnd{device}}); //what if entity removed? ok to return a dead one?
            false
        }
    });

    //todo need to handle if new entity is created/moved under the cursor

}

pub fn cursor_press_cleanup(
    root_query: Query<&UiLayoutComputed, With<UiRoot>>,
    layout_computed_query: Query<&UiLayoutComputed>,
    pressable_query: Query<(Entity,& UiPressable)>,

    // focus_states : Res<UiFocusStates>,
    // focuseds : Res<UiFocuseds>,

    mut device_cursors : ResMut<CursorDevicePointers>,
    mut device_presseds : ResMut<CursorDevicePresseds>,
    // mut device_cursors : Local<DeviceCursors>, //[(root_entity,device)]=cursor

    // mut device_presseds :   Local<DevicePresseds>, //[button][(root_entity,device_type)]=(pressed_entity,is_pressed)

    // mut input_focus_event_reader: MessageReader<UiInteractInputFocusMessage>,
    mut ui_output_event_writer: MessageWriter<UiInteractEvent>,
) {
    //device_cursors: remove dead roots/pressed entities from device_cursors
    device_cursors.0.retain(|&(root_entity,_device),_|{
        let root_unlocked= root_query.get(root_entity).map(|computed|computed.unlocked).unwrap_or_default();
        root_unlocked
    });


    //remove dead roots/pressed entities from device_presseds
    device_presseds.0.retain(|&button,button_device_presseds|{
        button_device_presseds.retain(|&(root_entity,device),&mut (pressed_entity,is_pressed)|{
            let root_alive= root_query.get(root_entity).map(|computed|computed.unlocked).unwrap_or_default();
            let (computed_root_entity,unlocked)=layout_computed_query.get(pressed_entity).map(|c|(c.root_entity,c.unlocked)).unwrap_or((Entity::PLACEHOLDER,false));
            let pressable_enabled=pressable_query.get(pressed_entity).map(|(_,c)|c.enable&&c.pressable.contains(&button)).unwrap_or_default();

            let b=root_alive && unlocked && pressable_enabled && computed_root_entity==root_entity; //&& entities_presseds_contains

            if !b && is_pressed {
                ui_output_event_writer.write(UiInteractEvent{entity:pressed_entity,event_type:UiInteractMessageType::CursorPressEnd{ device, button }});
            }

            b
        });

        !button_device_presseds.is_empty()
    });
}

fn do_hover(
    cursor:Option<Vec2>,
    root_entity:Entity,
    device:i32,
    cur_hover_entities:&mut CursorHover,
    hover_root_entities: &mut HashMap<Entity, Vec<Entity>>,
    ui_event_writer: &mut MessageWriter<UiInteractEvent>,

    layout_computed_query: Query<&UiLayoutComputed>,
    pressable_query: Query<(Entity,& UiPressable)>,
) {


    let Some(entities)=hover_root_entities.get(&root_entity) else {
        return;
    };

    //
    if let Some((entity,_))=cur_hover_entities.0.get_mut(&(root_entity,device)).cloned() {
        let layout_computed=layout_computed_query.get(entity).unwrap(); //cleanup makes sure it has layoutcomputed
        let rect=layout_computed.clamped_border_rect();

        if cursor.is_none() || rect.is_zero() || !rect.contains_point(cursor.unwrap()) {
            cur_hover_entities.0.remove(&(root_entity,device)).unwrap();
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::HoverEnd{device}});
        }
    }

    //
    let Some(cursor)=cursor else {
        return;
    };

    //
    let old_hover_entity=cur_hover_entities.0.get(&(root_entity,device)).cloned().map(|x|x.0);


    //could check if old_hover_entity.is_some() and return here, skip the search
    // if old_hover_entity.is_some() {
    //     return;
    // }

    //
    let new_hover_entity= entities.iter().find(|&&entity|{
        let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
        let layout_computed=layout_computed_query.get(entity).unwrap();
        let rect=layout_computed.clamped_border_rect();
        pressable.hoverable && !rect.is_zero() && rect.contains_point(cursor)
    }).cloned();

    //
    if let Some(entity)=new_hover_entity {
        if old_hover_entity != Some(entity) {
            //
            if let Some(old_hover_entity)=old_hover_entity {
                cur_hover_entities.0.remove(&(root_entity,device)).unwrap();
                ui_event_writer.write(UiInteractEvent{entity:old_hover_entity,event_type:UiInteractMessageType::HoverEnd{device}});
            }

            //
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::HoverBegin{device}});
            cur_hover_entities.0.insert((root_entity,device), (entity,cursor));
        }
    }
}
pub fn update_press_events(
    root_query: Query<&UiLayoutComputed, With<UiRoot>>,
    layout_computed_query: Query<&UiLayoutComputed>,
    pressable_query: Query<(Entity,& UiPressable)>,

    // focus_states : Res<UiFocusStates>,
    // focuseds : Res<UiFocuseds>,


    mut device_cursors : ResMut<CursorDevicePointers>,
    mut device_presseds : ResMut<CursorDevicePresseds>,
    mut cur_hover_entities:ResMut<CursorHover>,

    // mut device_cursors : Local<DeviceCursors>, //[(root_entity,device)]=cursor

    // mut device_presseds :   Local<DevicePresseds>, //[button][(root_entity,device_type)]=(pressed_entity,is_pressed)

    mut input_event_reader: MessageReader<UiInteractInputMessage>,
    // mut input_focus_event_reader: MessageReader<UiInteractInputFocusMessage>,
    mut ui_event_writer: MessageWriter<UiInteractEvent>,
) {




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



    //
    for ev in input_event_reader.read()

    {

        //
        if !ev.root_entity()
            .and_then(|root_entity|root_query.get(root_entity).ok())
            .map(|computed|computed.unlocked)
            .unwrap_or_default()
        {
            continue;
        }

        //
        match ev.clone() {
            UiInteractInputMessage::CursorMoveTo{root_entity,device,cursor} => {
                if let Some(cursor)=cursor {
                    device_cursors.0.insert((root_entity,device),cursor);
                } else {
                    device_cursors.0.remove(&(root_entity,device));
                }

                //
                for (button,pressed_entity,is_pressed) in device_presseds.0.iter_mut()
                    .filter_map(|(&button,button_device_presseds)|button_device_presseds.get_mut(&(root_entity,device)).map(|x|(button,x)))
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
                            ui_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorPressBegin{ device, button }});
                        }
                    } else if !cursor_inside && *is_pressed {
                        *is_pressed=false;

                        if !pressable_always {
                            ui_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorPressEnd{ device, button }});
                        }
                    }
                }

                //
                do_hover(
                    cursor,
                    root_entity,
                    device,
                    &mut cur_hover_entities,
                    &mut roots_pressable_entities,
                    &mut ui_event_writer,

                    layout_computed_query,
                    pressable_query,
                );
            }

            //if same device used for cursor and button press, then a depress of one will depress the other
            UiInteractInputMessage::CursorPressBegin{root_entity,device, button } => {
                let cursor = device_cursors.0.get(&(root_entity,device)).cloned();

                //already pressed on an entity
                if device_presseds.0.get(&button).map(|button_device_presseds|{
                    button_device_presseds.contains_key(&(root_entity,device))
                }).unwrap_or_default() {
                    continue;
                }

                //get entity cursor is on
                let pressable_entity=cursor.and_then(|cursor|{
                    roots_pressable_entities.get(&root_entity).and_then(|pressable_entities|{
                        pressable_entities.iter().find(|&&entity|{
                            let computed = layout_computed_query.get(entity).unwrap();
                            let pressable=pressable_query.get(entity).map(|(_,c)|c.pressable.contains(&button)).unwrap_or_default();
                            if !pressable {
                                return false;
                            }

                            let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                            let cursor_inside= !outer_rect.is_zero() && outer_rect.contains_point(cursor);
                            cursor_inside
                        })
                    })
                }).cloned();

                //
                if let Some(entity)=pressable_entity {
                    ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::CursorPressBegin{ device, button }});

                    device_presseds.0.entry(button).or_default().insert((root_entity,device),(entity,true));

                }
            }

            UiInteractInputMessage::CursorPressEnd{root_entity,device, button } => {

                let Some((pressed_entity,is_pressed))=device_presseds.0.get_mut(&button)
                    .and_then(|button_device_presseds|button_device_presseds.remove(&(root_entity,device)))
                else {
                    continue;
                };

                //
                let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap(); //can use unwrap, wouldn't be in device_presseds otherwise

                //
                if pressable.always || is_pressed { //always means it will always be pressed (when cursor/focus is no longer on entity)
                    ui_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorPressEnd{ device, button }});
                }

                if is_pressed {
                    ui_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorClick{ device, button }});
                }

            }

            UiInteractInputMessage::CursorPressCancel{root_entity,device, button }
            => {

                let Some((pressed_entity,is_pressed))=device_presseds.0.get_mut(&button)
                    .and_then(|button_device_presseds|button_device_presseds.remove(&(root_entity,device)))
                else {
                    continue;
                };


                //
                let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap(); //can use unwrap, wouldn't be in device_presseds otherwise

                if pressable.always || is_pressed { //always means it will always be pressed (when cursor/focus is no longer on entity)
                    ui_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorPressEnd{ device, button }});
                }
            }

            _=>{}
        } //match
    } //for
}
