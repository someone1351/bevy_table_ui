
use bevy::app::App;
use bevy::asset::{load_internal_asset, weak_handle, Handle};
use bevy::prelude::Shader;


//shaders
// pub const COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(5312396983770130001);
pub const MY_COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> = weak_handle!("0a991ecd-134f-4f82-adf1-0fcc86f02227");

pub fn setup_shaders(app: &mut App) {
    load_internal_asset!(app, MY_COLORED_MESH2D_SHADER_HANDLE, "my_mesh2d_col.wgsl", Shader::from_wgsl);
}
