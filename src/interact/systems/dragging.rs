/*
TODO
* change cell_size if updated? no?
*/
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
use super::super::messages::*;
// use super::super::utils::*;
// use super::super::values::*;

#[derive(PartialEq, Eq,Hash)]
pub struct DragKey {
    root_entity:Entity,device:i32,
}

pub struct Drag {
    dragged_entity:Entity,
    start_cursor:Vec2,
    cursor:Vec2,
    _size:Vec2,
    // offset:Vec2,
}
pub fn update_drag_events(
    mut device_cursors : Local<HashMap<DragKey,Vec2>>, //[(root_entity,device)]=cursor
    mut device_drags : Local<HashMap<DragKey,Drag>>, //[(root_entity,device,)]=(dragged_entity,size,offset,cursor,)
    // mut entities_presseds : Local<HashMap<(Entity,Entity),HashSet<Option<i32>>>>, //[(root_entity,press_entity)]=set<Some(device)>, the None is a separate device representing pressable.pressed

    // mut last_cursors : Local<HashMap<(Entity,i32),Vec2>>, //[(root_entity,device)]=cursor


    // press_states:Res<UiPressStates>,
    draggable_query: Query<(Entity,&UiLayoutComputed,&UiDraggable)>,
    // parent_query : Query<&ChildOf,With<UiLayoutComputed>>,

    // root_query: Query<(Entity,&UiLayoutComputed), (With<UiRoot>, With<UiLayoutComputed> )>,
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,


    mut input_event_reader: MessageReader<UiInteractInputMessage>,
    mut ui_output_event_writer: MessageWriter<UiInteractEvent>,
    // root_query: Query<Entity,(Without<Parent>,With<UiLayoutComputed>)>,
    // children_query: Query<&Children,(With<UiLayoutComputed>,)>,

) {
    //

    device_cursors.retain(|key,_|{
        root_query.get(key.root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default()
    });

    device_drags.retain(|key,_|{
        root_query.get(key.root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default()
    });

    //remove disabled or without drag component
    device_drags.retain(|_,drag|{
        draggable_query.get(drag.dragged_entity).map(|(_,computed,draggable)|computed.unlocked&&draggable.enable).unwrap_or_default()
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


        // let Some(root_entity)=[entity].into_iter().chain(parent_query.iter_ancestors(entity)).find(|&ancestor_entity|root_query.contains(ancestor_entity)) else {
        //     continue;
        // };

        // let root_entity=parent_query.iter_ancestors(entity).last().unwrap();

        roots_pressable_entities.entry(computed.root_entity).or_default().insert(computed.order,(entity,computed.cell_size.sum(),computed.clamped_border_rect()));
    }


    //

    //
    for ev in input_event_reader.read() {

        if !root_query.get(ev.get_root_entity()).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
            continue;
        }
        match ev.clone() {
            UiInteractInputMessage::CursorMoveTo{root_entity,device,cursor} => {
                // println!("cursor {cursor:?}");
                let Some(cursor)=cursor else {
                    device_cursors.remove(&DragKey{root_entity,device});
                    continue;
                };

                // if !root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
                //     continue;
                // }

                device_cursors.insert(DragKey {root_entity,device},cursor);

                let Some(drag) = device_drags.get_mut(&DragKey {root_entity,device}) else {
                    continue;
                };


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
                    ui_output_event_writer.write(UiInteractEvent{
                        entity:drag.dragged_entity,
                        event_type:UiInteractMessageType::DragX {px:dragged_px.x,} //scale:dragged_scale.x
                    });
                }

                if dragged_delta_px.y != 0.0 {
                    ui_output_event_writer.write(UiInteractEvent{
                        entity:drag.dragged_entity,
                        event_type:UiInteractMessageType::DragY {px:dragged_px.y,} //scale:dragged_scale.y
                    });
                }
            }

            //if same device used for cursor and button press, then a depress of one will depress the other
            UiInteractInputMessage::CursorPressBegin{root_entity,device} => {
                //remove any prev (not normally needed)
                device_drags.remove(&DragKey {root_entity,device});

                //

                // if !root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default() {
                //     continue;
                // }

                //
                let Some(draggable_entities)=roots_pressable_entities.get(&root_entity) else {
                    continue;
                };

                //
                let Some(cursor) = device_cursors.get(&DragKey {root_entity,device}).cloned() else {
                    continue;
                };

                //
                let Some((_,&(found_entity,cell_size,_)))=draggable_entities.iter().rev().find(|&(_,&(_entity,_,rect))|{
                    rect.contains_point(cursor)
                }) else {
                    continue;
                };

                //
                device_drags.insert(
                    DragKey {root_entity,device},
                    Drag { dragged_entity: found_entity, cursor: cursor, start_cursor: cursor,
                        _size: cell_size,
                        // offset: border_rect.left_top()
                    },
                );
            }
            UiInteractInputMessage::CursorPressEnd{root_entity,device} => {
                device_drags.remove(&DragKey {root_entity,device});
            }
            UiInteractInputMessage::CursorPressCancel{root_entity,device} => {
                device_drags.remove(&DragKey {root_entity,device});
            }
            _=>{}
        } //match
    } //for
}
