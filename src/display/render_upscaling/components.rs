
use bevy::ecs::prelude::*;
use bevy::render::render_resource::*;




#[derive(Component)]
pub struct ViewUpscalingPipeline(pub CachedRenderPipelineId);
