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

// use crate::UiRect;

use crate::utils::ui_rect_is_zero;

use super::super::components::*;
use super::super::resources::*;
use super::super::messages::*;
// use super::super::utils::*;
// use super::super::values::*;

use super::super::super::layout::components::{UiLayoutComputed,UiRoot};


pub fn hover_cleanup(
    hoverable_query: Query<(Entity,&UiLayoutComputed,&UiCursorable)>,
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    mut ui_event_writer: MessageWriter<UiInteractEvent>,
    // mut cur_hover_entities : Local<HashMap<(Entity,i32),(Entity,Vec2)>>, //[(root_entity,device)]=cur_hover_entity
    mut cur_hover_entities : ResMut<CursorHovers>, //[(root_entity,device)]=cur_hover_entity
) {

    //un hover inactive/disabled/invisible, and cursor no longer inside due to node pos/size change
    //remove inactive root nodes

    cur_hover_entities.0.retain(|&(root_entity,device),&mut (entity,cursor)|{
        let root_alive = root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();
        let hoverable_alive = hoverable_query.get(entity).map(|(_,layout_computed,hoverable)|{
            hoverable.hoverable && layout_computed.unlocked && layout_computed.clamped_border_rect().contains(cursor)
        }).unwrap_or_default();

        // if let Ok((_,layout_computed,hoverable)) = hoverable_query.get(*entity) {
        //     if hoverable.enable && layout_computed.unlocked && layout_computed.clamped_border_rect().contains_point(*cursor) {
        //         return true;
        //     }
        // }

        if root_alive && hoverable_alive {
            true
        } else {
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::CursorHoverEnd{device}}); //what if entity removed? ok to return a dead one?
            false
        }
    });

    //todo need to handle if new entity is created/moved under the cursor

}

pub fn cursor_press_cleanup(
    root_query: Query<&UiLayoutComputed, With<UiRoot>>,
    layout_computed_query: Query<&UiLayoutComputed>,
    pressable_query: Query<(Entity,& UiCursorable)>,

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
            let pressable_enabled=pressable_query.get(pressed_entity).map(|(_,c)|{
                c.pressable&&c.pressable && (c.press_onlys.is_empty() || c.press_onlys.contains(&button))
            }).unwrap_or_default();

            let b=root_alive && unlocked && pressable_enabled && computed_root_entity==root_entity; //&& entities_presseds_contains

            if !b && is_pressed {
                ui_output_event_writer.write(UiInteractEvent{entity:pressed_entity,event_type:UiInteractMessageType::CursorPressEnd{ device, button,last:true, }});
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
    cur_hover_entities:&mut CursorHovers,
    hover_root_entities: &mut HashMap<Entity, Vec<Entity>>,
    ui_event_writer: &mut MessageWriter<UiInteractEvent>,

    layout_computed_query: Query<&UiLayoutComputed>,
    pressable_query: Query<(Entity,& UiCursorable)>,
) {


    let Some(entities)=hover_root_entities.get(&root_entity) else {
        return;
    };

    //
    if let Some((entity,_))=cur_hover_entities.0.get_mut(&(root_entity,device)).cloned() {
        let layout_computed=layout_computed_query.get(entity).unwrap(); //cleanup makes sure it has layoutcomputed
        let rect=layout_computed.clamped_border_rect();

        if cursor.is_none() || ui_rect_is_zero(rect) || !rect.contains(cursor.unwrap()) {
            cur_hover_entities.0.remove(&(root_entity,device)).unwrap();
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::CursorHoverEnd{device}});
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
        pressable.hoverable && !ui_rect_is_zero(rect) && rect.contains(cursor)
    }).cloned();

    //
    if let Some(entity)=new_hover_entity {
        if old_hover_entity != Some(entity) {
            //
            if let Some(old_hover_entity)=old_hover_entity {
                cur_hover_entities.0.remove(&(root_entity,device)).unwrap();
                ui_event_writer.write(UiInteractEvent{entity:old_hover_entity,event_type:UiInteractMessageType::CursorHoverEnd{device}});
            }

            //
            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractMessageType::CursorHoverBegin{device, cursor }});
            cur_hover_entities.0.insert((root_entity,device), (entity,cursor));
        }
    }
}

pub fn drag_cleanup(
    draggable_query: Query<(Entity,&UiLayoutComputed,&UiCursorable)>,
    // root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    mut output_event_writer: MessageWriter<UiInteractEvent>,
    mut device_drags:ResMut<CursorDrags>,
    // mut device_cursors : Local<HashMap<DragKey,Vec2>>, //[(root_entity,device)]=cursor
) {

    // device_cursors.retain(|key,_|{
    //     root_query.get(key.root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default()
    // });

    //not necessary, handled below
    // device_drags.0.retain(|&(root_entity,_device),_|{
    //     root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default()
    // });

    //remove disabled or without drag component
    device_drags.0.retain(|&(_,device),button_drags|{
        button_drags.retain(|&button,drag|{
            let b=draggable_query.get(drag.dragged_entity)
                .map(|(_,computed,draggable)|computed.unlocked&&draggable.draggable)
                .unwrap_or_default();

            if !b {
                output_event_writer.write(UiInteractEvent{
                    entity:drag.dragged_entity,
                    event_type:UiInteractMessageType::CursorDragEnd {device, button } //scale:dragged_scale.y
                });
            }

            b
        });

        !button_drags.is_empty()
    });
}
fn do_drag_press_begin(
    root_entity:Entity,
    device:i32,
    button:i32,
    device_drags:&mut CursorDrags,
    roots_cursorable_entities:&HashMap<Entity, Vec<Entity>>,
    device_cursors:&CursorDevicePointers,
    layout_computed_query: Query<&UiLayoutComputed>,
    pressable_query: Query<(Entity,& UiCursorable)>,

    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    //remove any prev (not normally needed)
    if let Some(x)=device_drags.0.get_mut(&(root_entity,device)) {
        x.remove(&button);
    }

    //

    // if !root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
    //     continue;
    // }

    //
    let Some(cursorable_entities)=roots_cursorable_entities.get(&root_entity) else {
        return;
    };

    //
    let Some(cursor) = device_cursors.0.get(&(root_entity,device)).cloned() else {
        //stop drag?
        return;
    };


    //(_,&(found_entity,cell_size,_))
    let Some(found_entity)=cursorable_entities.iter().rev().find(|&&entity|{ //&(_,&(_entity,_,rect))
        let pressable=pressable_query.get(entity).map(|x|x.1).unwrap();
        let layout_computed=layout_computed_query.get(entity).unwrap();
        let rect=layout_computed.border_rect();
        pressable.draggable && rect.contains(cursor)
    }).cloned() else {
        return;
    };

    //
    device_drags.0.entry((root_entity,device)).or_default().entry(button).or_insert(Drag {
        dragged_entity: found_entity,
        cursor: cursor,
        start_cursor: cursor,
        // _size: cell_size,
        // buttons: todo!(),
        // offset: border_rect.left_top()
    });

    //
    let layout_computed=layout_computed_query.get(found_entity).unwrap();
    let outer_offset=cursor-layout_computed.outer_rect().min;
    let inner_offset=cursor-layout_computed.inner_rect().min;

    //
    output_event_writer.write(UiInteractEvent{
        entity:found_entity,
        event_type:UiInteractMessageType::CursorDragBegin {device, button, outer_offset, inner_offset, cursor } //scale:dragged_scale.y
    });

    // device_drags.0.get_mut(&(root_entity,device)).map(|x|x.entry(button));
    // //
    // device_drags.0.insert(
    //     DragKey {root_entity,device},
    //     Drag { dragged_entity: found_entity, cursor: cursor, start_cursor: cursor,
    //         // _size: cell_size,
    //         buttons: todo!(),
    //         // offset: border_rect.left_top()
    //     },
    // );
}

fn do_drag_press_end_cancel(
    root_entity:Entity,
    device:i32,
    button:i32,
    device_drags:&mut CursorDrags,
    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    if let Some(drags)=device_drags.0.get_mut(&(root_entity,device)) {
        if let Some(drag)=drags.remove(&button) {
            output_event_writer.write(UiInteractEvent{
                entity:drag.dragged_entity,
                event_type:UiInteractMessageType::CursorDragEnd {device, button } //scale:dragged_scale.y
            });
        }
    }
}

fn do_drag_move(
    root_entity:Entity,
    device:i32,
    cursor : Option<Vec2>,
    device_drags:&mut CursorDrags,
    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    // let Some(cursor)=cursor else {
    //     device_cursors.remove(&DragKey{root_entity,device});
    //     continue;
    // };

    // if !root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
    //     continue;
    // }

    // device_cursors.insert(DragKey {root_entity,device},cursor);

    // let Some(last_cursor)=last_cursor else {
    //     return;
    // };
    let Some(cursor)=cursor else {
        return;
    };

    // let Some(drag) = device_drags.0.get_mut(&DragKey {root_entity,device}) else {
    //     return;
    // };

    for (&button,drag) in device_drags.0.get_mut(&(root_entity,device)).map(|q|q.iter_mut()).unwrap_or_default() {

        // let entity_presseds=entities_presseds.entry((root_entity,entity)).or_default();
        // let (_,layout_computed,_)=draggable_query.get(drag.dragged_entity).unwrap();
        // *drag_start_offset=start_cursor - offset;
        // let dragged_px = cursor-drag.offset-(drag.cursor - drag.offset);
        let dragged_delta_px = cursor-drag.cursor;
        let dragged_px = cursor-drag.start_cursor;
        // let dragged_scale = dragged_px/ drag.size;
        // dragged_px/layout_computed.cell_size.sum()

        // let mut dragged_scale = Vec2::ZERO;
        // let cell_size=layout_computed.cell_size.sum();
        // // println!("hmm {cell_size:?}");
        // if cell_size.x>0.0 {
        //     dragged_scale.x=dragged_px.x/cell_size.x;
        // }
        // if cell_size.y>0.0 {
        //     dragged_scale.y=dragged_px.y/cell_size.y;
        // }

        // let dragged_scale = dragged_px;//Vec2::new(0.0,0.0);

        drag.cursor=cursor;

        if dragged_delta_px.x != 0.0 {
            output_event_writer.write(UiInteractEvent{
                entity:drag.dragged_entity,
                event_type:UiInteractMessageType::CursorDragX {dist:dragged_px.x,delta:dragged_delta_px.x, device, button } //scale:dragged_scale.x
            });
        }

        if dragged_delta_px.y != 0.0 {
            output_event_writer.write(UiInteractEvent{
                entity:drag.dragged_entity,
                event_type:UiInteractMessageType::CursorDragY {dist:dragged_px.y,delta:dragged_delta_px.y, device, button } //scale:dragged_scale.y
            });
        }
    }

}

fn do_press_move(
    cursor:Option<Vec2>,
    root_entity:Entity,
    device:i32,

    layout_computed_query: Query<&UiLayoutComputed>,
    device_presseds:&mut CursorDevicePresseds,

    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    for (button,pressed_entity,is_pressed) in device_presseds.0.iter_mut()
    .filter_map(|(&button,button_device_presseds)|button_device_presseds.get_mut(&(root_entity,device)).map(|x|(button,x)))
    .map(|(button,x)|(button,x.0,&mut x.1))
    {
        //
        // let pressable_always=pressable_query.get(pressed_entity).map(|x|x.1.always).unwrap(); //can use unwrap otherwise won't be in device_presseds

        //
        let layout_computed = layout_computed_query.get(pressed_entity).unwrap(); //can use unwrap otherwise won't be in device_presseds

        //
        let cursor_inside= cursor.map(|cursor|{
            let outer_rect=layout_computed.clamped_border_rect();//.clamped_padding_rect();
            let cursor_inside= !ui_rect_is_zero(outer_rect) && outer_rect.contains(cursor);
            cursor_inside
        }).unwrap_or_default();

        //handle mouse pressed and button prev pressed and moving on/off button
        if cursor_inside && !(*is_pressed) {
            *is_pressed=true;

            let cursor=cursor.unwrap();
            let outer_offset=cursor-layout_computed.outer_rect().min;
            let inner_offset=cursor-layout_computed.inner_rect().min;

            // if !pressable_always {
            output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorPressBegin{
                device,
                button,
                first:false,
                cursor,
                outer_offset,
                inner_offset,
            }});
            // }
        } else if !cursor_inside && *is_pressed {
            *is_pressed=false;

            // if !pressable_always {
            output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorPressEnd{ device, button,last:false, }});
            // }
        }
    }
}

fn do_press_begin(
    cursor:Option<Vec2>,
    root_entity:Entity,
    device:i32,
    button:i32,
    roots_pressable_entities:&HashMap<Entity, Vec<Entity>>,
    layout_computed_query: Query<&UiLayoutComputed>,
    pressable_query: Query<(Entity,& UiCursorable)>,
    device_presseds:&mut CursorDevicePresseds,
    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {
    //already pressed on an entity
    if device_presseds.0.get(&button).map(|button_device_presseds|{
        button_device_presseds.contains_key(&(root_entity,device))
    }).unwrap_or_default() {
        return;
    }

    //get entity cursor is on
    let pressable_entity=cursor.and_then(|cursor|{
        roots_pressable_entities.get(&root_entity).and_then(|pressable_entities|{
            pressable_entities.iter().find(|&&entity|{
                let layout_computed = layout_computed_query.get(entity).unwrap();
                let pressable=pressable_query.get(entity).map(|(_,c)|{
                    c.pressable && (c.press_onlys.is_empty() || c.press_onlys.contains(&button))
                }).unwrap_or_default();

                if !pressable {
                    return false;
                }

                let outer_rect=layout_computed.clamped_border_rect();//.clamped_padding_rect();
                let cursor_inside= !ui_rect_is_zero(outer_rect) && outer_rect.contains(cursor);
                cursor_inside
            })
        })
    }).cloned();

    //
    if let Some(entity)=pressable_entity {
        let layout_computed = layout_computed_query.get(entity).unwrap();
        let cursor=cursor.unwrap();
        let outer_offset=cursor-layout_computed.outer_rect().min;
        let inner_offset=cursor-layout_computed.inner_rect().min;

        output_event_writer.write(UiInteractEvent{
            entity,event_type:UiInteractMessageType::CursorPressBegin{
                device,
                button,
                first:true,
                cursor,
                outer_offset,
                inner_offset,
        }});

        device_presseds.0.entry(button).or_default().insert((root_entity,device),(entity,true));

    }
}

fn do_press_end(
    root_entity:Entity,
    device:i32,
    button:i32,
    device_presseds:&mut CursorDevicePresseds,
    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {

    let Some((pressed_entity,is_pressed))=device_presseds.0.get_mut(&button)
        .and_then(|button_device_presseds|button_device_presseds.remove(&(root_entity,device)))
    else {
        return;
    };

    //
    // let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap(); //can use unwrap, wouldn't be in device_presseds otherwise

    //
    // if is_pressed // || pressable.always //always means it will always be pressed (when cursor/focus is no longer on entity)
    { //may be called twice in a row if cursor is moved off the button (once), and then released (twice)
        output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorPressEnd{ device, button,last:true, }});
    }

    if is_pressed {
        output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorClick{ device, button }});
    }
}

fn do_press_cancel(
    root_entity:Entity,
    device:i32,
    button:i32,
    device_presseds:&mut CursorDevicePresseds,
    output_event_writer: &mut MessageWriter<UiInteractEvent>,
) {

    let Some((pressed_entity,is_pressed))=device_presseds.0.get_mut(&button)
        .and_then(|button_device_presseds|button_device_presseds.remove(&(root_entity,device)))
    else {
        return;
    };


    //
    // let pressable=pressable_query.get(pressed_entity).map(|x|x.1).unwrap(); //can use unwrap, wouldn't be in device_presseds otherwise

    if is_pressed // || pressable.always //always means it will always be pressed (when cursor/focus is no longer on entity)

    {
        output_event_writer.write(UiInteractEvent{entity: pressed_entity,event_type:UiInteractMessageType::CursorPressEnd{ device, button,last:true, }});
    }
}

pub fn update_press_events(
    root_query: Query<&UiLayoutComputed, With<UiRoot>>,
    layout_computed_query: Query<&UiLayoutComputed>,
    cursorable_query: Query<(Entity,& UiCursorable)>,

    // focus_states : Res<UiFocusStates>,
    // focuseds : Res<UiFocuseds>,


    mut device_cursors : ResMut<CursorDevicePointers>,
    mut device_presseds : ResMut<CursorDevicePresseds>,
    mut cur_hover_entities:ResMut<CursorHovers>,
    mut device_drags:ResMut<CursorDrags>,

    // mut device_cursors : Local<DeviceCursors>, //[(root_entity,device)]=cursor

    // mut device_presseds :   Local<DevicePresseds>, //[button][(root_entity,device_type)]=(pressed_entity,is_pressed)

    mut input_event_reader: MessageReader<UiInteractInputMessage>,
    // mut input_focus_event_reader: MessageReader<UiInteractInputFocusMessage>,
    mut output_event_writer: MessageWriter<UiInteractEvent>,
) {




    //
    let mut roots_pressable_entities: HashMap<Entity, Vec<Entity>> = HashMap::new(); //[root_entity]=pressable_entities

    //get root entities with their pressable descendants
    for (cursorable_entity,_cursorable) in cursorable_query.iter() {
        // if !pressable.pressable {
        //     continue;
        // }

        let Ok(computed) = layout_computed_query.get(cursorable_entity) else { continue; };

        if !computed.unlocked {
            continue;
        }

        roots_pressable_entities.entry(computed.root_entity).or_default().push(cursorable_entity);
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
                do_drag_move(
                    root_entity,
                    device,
                    cursor,
                    &mut device_drags,
                    &mut output_event_writer,
                );

                //
                do_press_move(
                    cursor,
                    root_entity,
                    device,
                    layout_computed_query,
                    &mut device_presseds,
                    &mut output_event_writer,
                );

                //
                do_hover(
                    cursor,
                    root_entity,
                    device,
                    &mut cur_hover_entities,
                    &mut roots_pressable_entities,
                    &mut output_event_writer,

                    layout_computed_query,
                    cursorable_query,
                );
            }

            //if same device used for cursor and button press, then a depress of one will depress the other
            UiInteractInputMessage::CursorPressBegin{root_entity,device, button } => {
                let cursor = device_cursors.0.get(&(root_entity,device)).cloned();

                //
                do_press_begin(
                    cursor,
                    root_entity,
                    device,
                    button,
                    &roots_pressable_entities,
                    layout_computed_query,
                    cursorable_query,
                    &mut device_presseds,
                    &mut output_event_writer,
                );
                //
                do_drag_press_begin(
                    root_entity,
                    device,
                    button,
                    &mut device_drags,
                    &roots_pressable_entities,
                    &device_cursors,
                    layout_computed_query,
                    cursorable_query,
                    &mut output_event_writer,
                );

            }

            UiInteractInputMessage::CursorPressEnd{root_entity,device, button } => {

                //
                do_press_end(
                    root_entity,
                    device,
                    button,
                    &mut device_presseds,
                    &mut output_event_writer,
                );


                //
                do_drag_press_end_cancel(
                    root_entity,
                    device,
                    button,
                    &mut device_drags,
                    &mut output_event_writer,
                );
            }

            UiInteractInputMessage::CursorPressCancel{root_entity,device, button }
            => {

                //
                do_press_cancel(
                    root_entity,
                    device,
                    button,
                    &mut device_presseds,
                    &mut output_event_writer
                );

                //
                do_drag_press_end_cancel(
                    root_entity,
                    device,
                    button,
                    &mut device_drags,
                    &mut output_event_writer,
                );
            }
            UiInteractInputMessage::CursorScroll { root_entity, device, axis, scroll } => {
                let cursor = device_cursors.0.get(&(root_entity,device)).cloned();

                let Some(cursor)=cursor else {
                    continue;
                };

                let Some(pressable_entities)=roots_pressable_entities.get(&root_entity) else {
                    continue;
                };

                let entity=pressable_entities.iter().find(|&&entity|{
                    let computed = layout_computed_query.get(entity).unwrap();
                    let pressable=cursorable_query.get(entity).map(|(_,c)|c.scrollable).unwrap_or_default();

                    if !pressable {
                        return false;
                    }

                    let outer_rect=computed.clamped_border_rect();//.clamped_padding_rect();
                    let cursor_inside= !ui_rect_is_zero(outer_rect) && outer_rect.contains(cursor);
                    cursor_inside
                }).cloned();

                if let Some(entity)=entity {
                    output_event_writer.write(UiInteractEvent { entity, event_type: UiInteractMessageType::CursorScroll {scroll, device, axis }});
                }
            }
            _=>{}
        } //match
    } //for
}
