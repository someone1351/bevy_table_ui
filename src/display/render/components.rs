use bevy::{render::{camera::Camera, render_resource::BindGroup}, render::extract_component::ExtractComponent, ecs::query::QueryItem};
use bevy::ecs::prelude::*;

// use bevy::render::color::Color;
use bevy::ecs::system::{lifetimeless::*, SystemParamItem};
use bevy::asset::prelude::*;
use bevy::render::texture::Image;
use bevy::asset::{Assets, Handle};

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