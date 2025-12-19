
use std::collections::HashMap;

use bevy::{ecs::prelude::*, math::Vec2};


#[derive(Resource,Debug, Default)]

//[device][root_entity][group]=(cur_focus_entity,focus_entity_stk)
pub struct FocusStates(pub HashMap<i32,HashMap<Entity,HashMap<i32,(Option<Entity>,Vec<Entity>)>>>);


#[derive(Resource,Debug, Default)]
pub struct FocusMoveHist2(pub HashMap<(i32,Entity),(u32,u32)>); //[(device,entity)]=(row,col)

#[derive(Resource,Debug, Default)]
//[device][entity]=(left,top,right,bottom)
// pub struct FocusMoveHists(pub HashMap<i32,HashMap<Entity,[Entity;4]>>);

pub struct FocusMoveHists(pub HashMap<(Entity,i32),Vec<Entity>>); //[(root_entity,device)][ind]=entity_hist

type DevicePresseds = HashMap<i32,HashMap<(Entity,i32),(Entity,bool)>>; //[button][(root_entity,device)]=(pressed_entity,is_pressed)


#[derive(Resource,Debug, Default)]
pub struct FocusDevicePresseds(pub DevicePresseds);

#[derive(Resource,Debug, Default)]
pub struct CursorDevicePresseds(pub DevicePresseds);

#[derive(Resource,Debug, Default)]
pub struct CursorDevicePointers(pub HashMap<(Entity,i32),Vec2>); // [(root_entity,device)]=cursor

#[derive(Resource,Debug, Default)]
pub struct CursorHovers(pub HashMap<(Entity,i32),(Entity,Vec2)>); //[(root_entity,device)]=cur_hover_entity



// #[derive(PartialEq, Eq,Hash,Debug)]
// pub struct DragKey {
//     pub root_entity:Entity,pub device:i32,
// }

#[derive(Debug)]
pub struct Drag {
    pub dragged_entity:Entity,
    pub start_cursor:Vec2,
    pub cursor:Vec2,
    // pub _size:Vec2,
    //pub offset:Vec2,
    // pub buttons:HashSet<i32>,
    // pub buttons:HashMap<i32,>,
}
#[derive(Resource,Debug, Default)]
pub struct CursorDrags(pub HashMap<(Entity,i32),HashMap<i32,Drag>>); //[(root_entity,device,)][button]=(dragged_entity,size,offset,cursor,)

// #[derive(Resource,Debug, Default)]
// pub struct UiFocuseds(pub HashMap<i32,HashSet<Entity>>); //[device][entity_focused] replaces focusable.focused, is it cleared if entity is deleted?


// #[derive(Resource,Debug, Default)]
// pub struct UiFocusStates {
//     pub cur_focuses:HashMap<i32,HashMap<Entity,HashMap<i32,(Option<Entity>,Vec<Entity>,
//         HashMap<Entity,u64>, //replace with left/right/top/bottom eg [Option<Entity>;4] or [(Option<Entity>,u64);4]
//     )>>>,
//         //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk,hist)
// }

// #[derive(Debug,Clone, Copy,PartialEq, Eq,Hash)]
// pub enum DeviceType {
//     Cursor(i32),
//     Focus(i32),
// }
// impl DeviceType {
//     pub fn is_cursor(&self) -> bool {
//         match self {
//             DeviceType::Cursor(_) => true,
//             DeviceType::Focus(_) => false,
//         }
//     }
//     pub fn is_focus(&self) -> bool {
//         match self {
//             DeviceType::Cursor(_) => false,
//             DeviceType::Focus(_) => true,
//         }
//     }
//     pub fn device(&self) -> i32 {
//         match self {
//             &DeviceType::Cursor(device) => device,
//             &DeviceType::Focus(device) => device,
//         }
//     }
// }

//using for drag, but requires draggable to have pressable ...  which is bad
// #[derive(Resource,Debug, Default)]
// pub struct UiPressStates {
//     pub device_presseds : HashMap<i32,HashMap<(Entity,DeviceType),(Entity,bool)>>, //[button][(root_entity,device,is_cursor)]=(pressed_entity,pressed)
// }

// #[derive(Resource,Debug, Default)]
// pub struct UiDragStates {
//     pub presseds : HashMap<(Entity,i32),Entity>,//[(root_entity,device)]=pressed_entity
// }