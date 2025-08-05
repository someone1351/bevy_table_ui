
// use bevy::ecs::prelude::*;
use core::ops::Range;


use bevy::asset::Handle;
use bevy::ecs::component::Component;




use bevy::image::Image;
use bevy::render::render_resource::*;


#[derive(Component)]
pub struct MyViewBindGroup {
    pub value: BindGroup,
}

#[derive(Component, Default, Debug, Clone)]
pub struct MyUiBatch {
    pub range: Range<u32>,
    pub image_handle: Option<Handle<Image>>,
}