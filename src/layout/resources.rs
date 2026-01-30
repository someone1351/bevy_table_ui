use std::collections::HashMap;

use bevy::ecs::{entity::Entity, resource::Resource};

use crate::UiLayoutComputed;

#[derive(Resource,Default)]
pub struct UiOldComputedLayouts(pub HashMap<Entity,UiLayoutComputed>);