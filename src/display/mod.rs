use bevy::{asset::Handle, color::Color, image::Image, prelude::Component};

pub mod components;
// pub mod render;
pub mod values;
pub mod systems;
pub mod plugin;

pub mod render_core;
pub mod render;
// pub mod mesh;


#[derive(Component, Debug, Clone,)]
pub struct TestRenderComponent {
    pub col : Color,
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32,
    pub handle : Option<Handle<Image>>,
}
