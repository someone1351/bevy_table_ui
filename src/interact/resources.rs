
use std::collections::{HashMap, HashSet};

use bevy::{ecs::prelude::*, math::Vec2};


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
    >
}


#[derive(Resource,Debug, Default)]
pub struct UiPressStates {
    // // pub device_cursors : HashMap<(Entity,i32),Vec2>, //[(root_entity,device)]=cursor
    pub device_presseds : HashMap<(Entity,i32,bool),(Entity,bool)>, //[(root_entity,device,is_cursor)]=(pressed_entity,pressed)
    // // pub entities_presseds : HashMap<(Entity,Entity),HashSet<Option<(i32,bool)>>>, //[(root_entity,press_entity)]=set<Some((device,is_cursor))>, the None is a separate device representing pressable.pressed
    // pub focuseds : HashMap<(Entity,i32),Entity>,//[(root_entity,group)]=focused_entity
}


// #[derive(Resource,Debug, Default)]
// pub struct UiDragStates {
//     pub presseds : HashMap<(Entity,i32),Entity>,//[(root_entity,device)]=pressed_entity
// }