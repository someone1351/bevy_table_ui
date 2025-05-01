use std::collections::HashMap;
use bevy::color::Color;
use bevy::image::Image;
use bevy::prelude::{AssetId, Entity, Resource, Vec2};
use bevy::render::texture::GpuImage;
use bevy::asset::Handle;
use bevy::render::render_resource::{BindGroup, BufferUsages, RawBufferVec};
// use bevy::ecs::system::Resource;

#[derive(Resource,Default)]
pub struct DummyImage {pub handle : Handle<Image>}

#[derive(Resource,Default)]
pub struct DummyGpuImage {pub gpu_image : Option<GpuImage>}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MyUiVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],//u32,
    pub uv: [f32; 2],
}

#[derive(Resource)]
pub struct MyUiMeta {
    pub vertices: RawBufferVec<MyUiVertex>,
}

impl Default for MyUiMeta {
    fn default() -> Self {
        Self {
            vertices: RawBufferVec::new(BufferUsages::VERTEX),
        }
    }
}

#[derive(Clone,Debug)]
pub struct MyUiExtractedElement{
    pub bl : Vec2, pub br : Vec2,pub tl : Vec2, pub tr : Vec2,
    pub bl_uv : Vec2, pub br_uv : Vec2,pub tl_uv : Vec2, pub tr_uv : Vec2,
    pub color : Color,
    pub depth:u32,
    pub image : Option<Handle<Image>>,
    pub entity:Entity,
    pub camera_entity:Entity,
}

#[derive(Resource,Default,Debug)]
pub struct MyUiExtractedElements {
    pub elements : Vec<MyUiExtractedElement>,
}

#[derive(Resource,Default)]
pub struct MyUiImageBindGroups {
    pub values: HashMap<Option<AssetId<Image>>, BindGroup>,
}

