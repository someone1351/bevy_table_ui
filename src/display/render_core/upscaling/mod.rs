// use crate::blit::{BlitPipeline, BlitPipelineKey};
use bevy::app::prelude::*;
use bevy::ecs::prelude::*;
use bevy::render::render_resource::SpecializedRenderPipelines;
use bevy::render::{
    Render, RenderApp, RenderSet,
};

mod nodes;
mod systems;
mod components;

pub mod pipelines;
pub mod shaders;

use pipelines::*;
use systems::*;

pub use nodes::UpscalingNode;

// use super::blit::{BlitPipeline, BlitPipelineKey};

pub struct UpscalingPlugin;

impl Plugin for UpscalingPlugin {


    fn build(&self, app: &mut App) {
        // load_internal_asset!(app, BLIT_SHADER_HANDLE, "blit.wgsl", Shader::from_wgsl);
        shaders::setup_shaders(app);

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.allow_ambiguous_resource::<SpecializedRenderPipelines<BlitPipeline>>();
        }

        //
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(
                Render,
                // This system should probably technically be run *after* all of the other systems
                // that might modify `PipelineCache` via interior mutability, but for now,
                // we've chosen to simply ignore the ambiguities out of a desire for a better refactor
                // and aversion to extensive and intrusive system ordering.
                // See https://github.com/bevyengine/bevy/issues/14770 for more context.
                prepare_view_upscaling_pipelines
                    .in_set(RenderSet::Prepare)
                    .ambiguous_with_all(),
            );
        }
    }
    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_resource::<BlitPipeline>()
            .init_resource::<SpecializedRenderPipelines<BlitPipeline>>();
    }
}
