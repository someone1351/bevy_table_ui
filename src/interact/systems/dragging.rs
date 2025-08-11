
// use std::collections::HashMap;

// use bevy::window::CursorGrabMode;

use std::collections::BTreeMap;
use std::collections::HashMap;

use bevy::ecs::prelude::*;
// use bevy::hierarchy::prelude::*;

use bevy::math::Vec2;



// use crate::input_map::{self, InputMapEvent};
// use crate::table_ui::UiComputed;
use super::super::super::layout::components::{UiLayoutComputed,UiRoot};
use super::super::super::layout::values::UiRect;


use super::super::components::*;
// use super::super::resources::*;
use super::super::events::*;
// use super::super::utils::*;
// use super::super::values::*;


pub fn update_drag_events(
    mut device_cursors : Local<HashMap<(Entity,i32),Vec2>>, //[(root_entity,device)]=cursor
    mut device_drags : Local<HashMap<(Entity,i32,),(Entity,Vec2,Vec2,Vec2)>>, //[(root_entity,device,)]=(dragged_entity,size,offset,cursor,)
    // mut entities_presseds : Local<HashMap<(Entity,Entity),HashSet<Option<i32>>>>, //[(root_entity,press_entity)]=set<Some(device)>, the None is a separate device representing pressable.pressed

    // mut last_cursors : Local<HashMap<(Entity,i32),Vec2>>, //[(root_entity,device)]=cursor


    // press_states:Res<UiPressStates>,
    draggable_query: Query<(Entity,&UiLayoutComputed,&UiDraggable)>,
    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,

    // root_query: Query<(Entity,&UiLayoutComputed), (With<UiRoot>, With<UiLayoutComputed> )>,
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,


    mut input_event_reader: EventReader<UiInteractInputEvent>,
    mut ui_output_event_writer: EventWriter<UiInteractEvent>,
    // root_query: Query<Entity,(Without<Parent>,With<UiLayoutComputed>)>,
    // children_query: Query<&Children,(With<UiLayoutComputed>,)>,

) {
    //

    device_cursors.retain(|&(root_entity,_device,),_cursor|{
        root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default()
    });

    device_drags.retain(|&(root_entity,_device,),( _dragged_entity,_size,_offset,_cursor,)|{
        root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default()
    });

    //remove disabled or without drag component
    device_drags.retain(|(_root_entity,_device,),( dragged_entity,_size,_offset,_cursor,)|{
        draggable_query.get(*dragged_entity).map(|(_,computed,draggable)|computed.unlocked&&draggable.enable).unwrap_or_default()
    });

    //
    let mut roots_pressable_entities: HashMap<Entity, BTreeMap<u32,(Entity,Vec2,UiRect)>> = HashMap::new(); //[root_entity][order]=draggable_entity

    //get root entities with their pressable descendants
    for (entity,computed,draggable) in draggable_query.iter() {
        if !draggable.enable || !computed.unlocked {
            continue;
        }

        // if !parent_query.contains(entity) { //roots can't be dragable //why not?
        //     continue;
        // }


        let Some(root_entity)=[entity].into_iter().chain(parent_query.iter_ancestors(entity)).find(|&ancestor_entity|root_query.contains(ancestor_entity)) else {
            continue;
        };

        // let root_entity=parent_query.iter_ancestors(entity).last().unwrap();

        roots_pressable_entities.entry(root_entity).or_default().insert(computed.order,(entity,computed.cell_size.sum(),computed.clamped_border_rect()));
    }


    //

    //
    for ev in input_event_reader.read() {

        if !root_query.get(ev.get_root_entity()).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
            continue;
        }
        match ev.clone() {
            UiInteractInputEvent::CursorMoveTo{root_entity,device,cursor} => {
                // println!("cursor {cursor:?}");
                let Some(cursor)=cursor else {
                    device_cursors.remove(&(root_entity,device));
                    continue;
                };

                // if !root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
                //     continue;
                // }

                device_cursors.insert((root_entity,device),cursor);

                let Some((entity,size,offset,start_cursor)) = device_drags.get_mut(&(root_entity,device)) else {
                    continue;
                };

                let entity=*entity;

                // let entity_presseds=entities_presseds.entry((root_entity,entity)).or_default();
                // let (_,layout_computed,pressable)=draggable_query.get(entity).unwrap();
                // *drag_start_offset=start_cursor - offset;
                let dragged_px = cursor-*offset-(*start_cursor - *offset);
                let dragged_scale = dragged_px/ *size; //computed.cell_size.sum()

                *start_cursor=cursor;

                if dragged_px.x != 0.0 {
                    ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractEventType::DragX {px:dragged_px.x,scale:dragged_scale.x}});
                }

                if dragged_px.y != 0.0 {
                    ui_output_event_writer.write(UiInteractEvent{entity,event_type:UiInteractEventType::DragY {px:dragged_px.y,scale:dragged_scale.y}});
                }
            }

            //if same device used for cursor and button press, then a depress of one will depress the other
            UiInteractInputEvent::CursorPressBegin{root_entity,device} => {
                //remove any prev (not normally needed)
                device_drags.remove(&(root_entity,device));

                //

                // if !root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
                //     continue;
                // }

                //
                let Some(draggable_entities)=roots_pressable_entities.get(&root_entity) else {
                    continue;
                };

                //
                let Some(cursor) = device_cursors.get(&(root_entity,device)).cloned() else {
                    continue;
                };

                //
                let Some((_,&(found_entity,cell_size,border_rect)))=draggable_entities.iter().rev().find(|&(_,&(_entity,_,rect))|{
                    rect.contains_point(cursor)
                }) else {
                    continue;
                };

                //
                device_drags.insert((root_entity,device), (found_entity,cell_size,border_rect.left_top(),cursor));
            }
            UiInteractInputEvent::CursorPressEnd{root_entity,device} => {
                device_drags.remove(&(root_entity,device));
            }
            UiInteractInputEvent::CursorPressCancel{root_entity,device} => {
                device_drags.remove(&(root_entity,device));
            }
            _=>{}
        } //match
    } //for
}
