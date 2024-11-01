

use std::collections::{HashMap, BTreeMap};

use bevy::color::Color;
use bevy::prelude::{Vec2, AssetId, Entity};
use bevy::render::texture::{Image,GpuImage};
use bevy::asset::Handle;
use bevy::render::render_resource::{BindGroup, BufferUsages, BufferVec, RawBufferVec};
use bevy::ecs::system::Resource;

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
    // mesh_handle : MyMeshHandle,
    // pub vertices: BufferVec<MyUiVertex>,
    pub vertices: RawBufferVec<MyUiVertex>,
    // model_bind_group: Option<BindGroup>,
    // view_bind_group: Option<BindGroup>,
}

impl Default for MyUiMeta {
    fn default() -> Self {
        Self {
            // vertices: BufferVec::new(BufferUsages::VERTEX),
            vertices: RawBufferVec::new(BufferUsages::VERTEX),
            // model_bind_group: None,
            // view_bind_group: None,
        }
    }
}

#[derive(Clone,Debug)]
pub struct MyUiExtractedElement{
    pub bl : Vec2, pub br : Vec2,pub tl : Vec2, pub tr : Vec2,
    pub bl_uv : Vec2, pub br_uv : Vec2,pub tl_uv : Vec2, pub tr_uv : Vec2,
    pub color : Color,
    // pub z : f32,
    // pub image : Handle<Image>,
    //ly,ry, tx,bx
    //lh,rh, tw,bw

    //lt_pos,lt_size, rb_pos,rb_size,
    //lt=ly,tx,lh,tw
    //rb=ry,bx,rh,bw
    //tl=tx,ly,tw,lh
    //br=bx,ry,bw,rh
    pub depth:u32,
    pub image : Option<Handle<Image>>,
    pub entity:Entity,
    pub camera_entity:Entity,
}

// impl Default for MyUiExtractedElement {
//     fn default() -> Self {
//         Self {
//             bl : Default::default(),
//             br : Default::default(),
//             tl : Default::default(),
//             tr : Default::default(),
//             bl_uv : Vec2::new(0.0,1.0), 
//             br_uv : Vec2::new(1.0,1.0),
//             tl_uv : Vec2::new(0.0,0.0), 
//             tr_uv : Vec2::new(1.0,0.0),
//             color :Color::NONE,
//             // z : 0.0,
//             // image : Default::default(),
//             depth:0,
//             image:None,
//         }
//     }
// }


#[derive(Resource,Default,Debug)]
pub struct MyUiExtractedElements {
    // pub elements: Vec<MyUiExtractedElement>,
    // pub elements : BTreeMap<(u32,Handle<Image>),(Entity,Vec<MyUiExtractedElement>)>,
    // pub elements2 : HashMap<(f32,AssetId<Image>),MyUiExtractedElement>
    // pub elements : HashMap<Entity,(u32,Handle<Image>,Vec<MyUiExtractedElement>)>,
    // pub x : HashMap<(u32,Handle<Image>),Entity>,
    // pub y : HashMap<(u32,Handle<Image>),Entity>,
    // pub elements : Vec<(u32,Option<Handle<Image>>,Entity,Vec<MyUiExtractedElement>)>,
    pub elements : Vec<MyUiExtractedElement>,

}


#[derive(Resource,Default)]
pub struct MyUiImageBindGroups {
    pub values: HashMap<Option<AssetId<Image>>, BindGroup>,
}

