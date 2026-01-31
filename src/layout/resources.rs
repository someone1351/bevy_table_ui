use std::collections::HashMap;

use bevy::ecs::{entity::Entity, resource::Resource};

use crate::UiLayoutComputed;

#[derive(Resource,Default)]
pub struct UiOldLayoutComputeds(pub HashMap<Entity,UiLayoutComputed>);

// #[derive(Resource,Default)]
// pub struct UiTempLayoutComputeds(pub HashMap<Entity,UiLayoutComputed>);