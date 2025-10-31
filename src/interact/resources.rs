
use std::collections::{HashMap, HashSet};

use bevy::ecs::prelude::*;

#[derive(Resource,Debug, Default)]
pub struct UiFocuseds(pub HashMap<i32,HashSet<Entity>>); //[device][entity_focused] replaces focusable.focused, is it cleared if entity is deleted?


#[derive(Resource,Debug, Default)]
pub struct UiFocusStates {
    pub cur_focuses:HashMap<i32,HashMap<Entity,HashMap<i32,(Option<Entity>,Vec<Entity>,
        HashMap<Entity,u64>, //replace with left/right/top/bottom eg [Option<Entity>;4] or [(Option<Entity>,u64);4]
    )>>>,
        //[device][root_entity][group]=(cur_focus_entity,focus_entity_stk,hist)
}

#[derive(Debug,Clone, Copy,PartialEq, Eq,Hash)]
pub enum DeviceType {
    Cursor(i32),
    Focus(i32),
}
impl DeviceType {
    pub fn is_cursor(&self) -> bool {
        match self {
            DeviceType::Cursor(_) => true,
            DeviceType::Focus(_) => false,
        }
    }
    pub fn is_focus(&self) -> bool {
        match self {
            DeviceType::Cursor(_) => false,
            DeviceType::Focus(_) => true,
        }
    }
    pub fn device(&self) -> i32 {
        match self {
            &DeviceType::Cursor(device) => device,
            &DeviceType::Focus(device) => device,
        }
    }
}

//using for drag, but requires draggable to have pressable ...  which is bad
#[derive(Resource,Debug, Default)]
pub struct UiPressStates {
    pub device_presseds : HashMap<i32,HashMap<(Entity,DeviceType),(Entity,bool)>>, //[button][(root_entity,device,is_cursor)]=(pressed_entity,pressed)
}

// #[derive(Resource,Debug, Default)]
// pub struct UiDragStates {
//     pub presseds : HashMap<(Entity,i32),Entity>,//[(root_entity,device)]=pressed_entity
// }