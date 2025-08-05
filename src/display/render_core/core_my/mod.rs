
#![allow(unused_imports)]

mod camera;
mod passes;
mod graph;
mod systems;

pub use camera::*;
use systems::*;

use graph::setup_graph;
pub use passes::*;


use bevy::render::render_resource::TextureFormat;


use bevy::app::{App, Plugin};
use bevy::ecs::prelude::*;
use bevy::render::{
    extract_component::ExtractComponentPlugin,
    render_phase::{
        sort_phase_system, DrawFunctions, ViewBinnedRenderPhases,
        ViewSortedRenderPhases,
    },
    ExtractSchedule, Render, RenderApp, RenderSet,
};

pub mod phases;

pub use phases::*;


pub const CORE_2D_DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
pub struct CoreMyPlugin;

impl Plugin for CoreMyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraMy>()
            .add_plugins(ExtractComponentPlugin::<CameraMy>::default())
            ;

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<DrawFunctions<OpaqueMy>>()
            .init_resource::<DrawFunctions<AlphaMaskMy>>()
            .init_resource::<DrawFunctions<TransparentMy>>()
            // .init_resource::<DrawFunctions<MyTransparentUi>>() //
            // .init_resource::<ViewSortedRenderPhases<MyTransparentUi>>() //
            .init_resource::<ViewSortedRenderPhases<TransparentMy>>()
            .init_resource::<ViewBinnedRenderPhases<OpaqueMy>>()
            .init_resource::<ViewBinnedRenderPhases<AlphaMaskMy>>()
            .add_systems(ExtractSchedule, (
                extract_core_2d_camera_phases,
                // extract_camera_view,
            ).chain() )
            .add_systems(
                Render,
                (
                    sort_phase_system::<TransparentMy>.in_set(RenderSet::PhaseSort),
                    // sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort), //
                    prepare_core_2d_depth_textures.in_set(RenderSet::PrepareResources),
                ),
            );
        setup_graph(render_app);

    }
}
