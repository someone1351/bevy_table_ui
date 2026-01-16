
use bevy::app::{App, Plugin};
use bevy::ecs::prelude::*;
use bevy::render::{
    extract_component::ExtractComponentPlugin,
    render_phase::{
        sort_phase_system, DrawFunctions, ViewBinnedRenderPhases,
        ViewSortedRenderPhases,
    },
    ExtractSchedule, Render, RenderApp, RenderSystems,
};


use super::camera::*;
use super::phases::*;

use super::systems::*;
use super::graph::setup_graph;

use bevy::render::render_resource::TextureFormat;



pub const CORE_2D_DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
pub struct CoreMyPlugin;

impl Plugin for CoreMyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraUi>()
            .add_plugins(ExtractComponentPlugin::<CameraUi>::default())
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
                    sort_phase_system::<TransparentMy>.in_set(RenderSystems::PhaseSort),
                    // sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort), //
                    prepare_core_2d_depth_textures.in_set(RenderSystems::PrepareResources),
                ),
            );
        setup_graph(render_app);

    }
}
