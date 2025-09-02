
#![allow(dead_code)]

use bevy::asset::{AssetId, Handle};
use bevy::color::Color;
use bevy::ecs::resource::Resource;

use bevy::image::Image;
use bevy::math::Vec2;
use bevy::platform::collections::HashMap;
use bevy::prelude::Entity;
use bevy::render::render_resource::{BindGroup, BufferUsages, RawBufferVec};
use bevy::render::sync_world::MainEntity;
use bevy::render::view::RenderLayers;



//render resources


#[derive(Clone,Debug)]
pub struct MyUiExtractedElement{
    // pub x:f32,
    // pub y:f32,
    // pub x2:f32,
    // pub y2:f32,

    pub bl : Vec2, pub br : Vec2,pub tl : Vec2, pub tr : Vec2,
    pub bl_uv : Vec2, pub br_uv : Vec2,pub tl_uv : Vec2, pub tr_uv : Vec2,
    pub depth:u32,

    pub color : Color,
    pub entity:Entity,
    pub main_entity:MainEntity,
    // pub camera_entity:Entity,
    // pub render_layers:Option<RenderLayers>,
    pub render_layers:RenderLayers,

    pub image : Option<Handle<Image>>,
}

impl Default for MyUiExtractedElement {
    fn default() -> Self {
        Self {
            bl: Vec2::new(0.0,0.0),
            br: Vec2::new(0.0,0.0),
            tl: Vec2::new(0.0,0.0),
            tr: Vec2::new(0.0,0.0),
            bl_uv: Vec2::new(0.0,0.0),
            br_uv: Vec2::new(1.0,0.0),
            tl_uv: Vec2::new(0.0,1.0),
            tr_uv: Vec2::new(1.0,1.0),
            depth: 0,
            color: Color::WHITE,
            entity: Entity::PLACEHOLDER,
            main_entity: Entity::PLACEHOLDER.into(),
            render_layers: RenderLayers::none(),
            image: None,
        }
    }
}
#[derive(Resource,Default,Debug)]
pub struct MyUiExtractedElements {
    pub elements : Vec<MyUiExtractedElement>,
    // pub cameras : HashMap<MainEntity>,
    // pub max_depth:u32,

}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MyUiVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],//u32,
    pub uv: [f32; 2], //added
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


#[derive(Resource,Default)]
pub struct MyUiImageBindGroups {
    pub values: HashMap<Option<AssetId<Image>>, BindGroup>,
}


