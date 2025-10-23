
use std::collections::{HashMap, HashSet};

use bevy::ecs::prelude::*;

#[derive(Resource,Debug, Default)]
pub struct UiFocuseds(pub HashSet<Entity>); //replaces focusable.focused, is it cleared if entity is deleted?


#[derive(Resource,Debug, Default)]
pub struct UiFocusStates {
    pub cur_focuses:HashMap<
        Entity, //root_entity
        HashMap<
            i32, //group
            (
                Option<Entity>, //cur_focus_entity
                Vec<Entity>, //focus_entity_stk
                HashMap<Entity,u64>, //hist
            )
        >
    >,
    // pub focuseds:HashSet<Entity>, //replaces focusable.focused, is it cleared if entity is deleted?
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

#[derive(Resource,Debug, Default)]
pub struct UiPressStates {
    // // // pub device_cursors : HashMap<(Entity,i32),Vec2>, //[(root_entity,device)]=cursor
    // // // pub entities_presseds : HashMap<(Entity,Entity),HashSet<Option<(i32,bool)>>>, //[(root_entity,press_entity)]=set<Some((device,is_cursor))>, the None is a separate device representing pressable.pressed
    // // pub focuseds : HashMap<(Entity,i32),Entity>,//[(root_entity,group)]=focused_entity

    // pub device_presseds : HashMap<(Entity,i32,bool),(Entity,bool)>, //[(root_entity,device,is_cursor)]=(pressed_entity,pressed)
    pub device_presseds : HashMap<i32,HashMap<(Entity,DeviceType),(Entity,bool)>>, //[button][(root_entity,device,is_cursor)]=(pressed_entity,pressed)
}

// #[derive(Resource,Debug, Default)]
// pub struct UiDragStates {
//     pub presseds : HashMap<(Entity,i32),Entity>,//[(root_entity,device)]=pressed_entity
// }