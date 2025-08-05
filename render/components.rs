use bevy::prelude::Image;
use bevy::reflect::Reflect;
use bevy::render::render_resource::BindGroup;
use bevy::ecs::prelude::*;

use bevy::asset::Handle;

use core::ops::Range;


#[derive(Component, Default, Debug, Clone)]
pub struct MyUiBatch {
    pub range: Range<u32>,
    pub image_handle: Option<Handle<Image>>,
    // pub z: f32,
}

#[derive(Component)]
pub struct MyViewBindGroup {
    pub value: BindGroup,
}


#[derive(Component)]
pub struct MyCameraView(pub Entity);

// #[derive(Component)]
// pub struct IsMyDefaultUiCamera;


#[derive(Component, Clone, Debug, Reflect, Eq, PartialEq)]
pub struct MyTargetCamera(pub Entity);

// impl MyTargetCamera {
//     pub fn entity(&self) -> Entity {
//         self.0
//     }
// }

/// This is the inverse of [`UiCameraView`].
#[derive(Component)]
pub struct MyUiViewTarget(pub Entity);