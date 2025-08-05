
pub mod core_my;
pub mod upscaling;

use bevy::app::{App, Plugin};

use core_my::CoreMyPlugin;
use upscaling::UpscalingPlugin;

#[derive(Default)]
pub struct CorePipelinePlugin;

impl Plugin for CorePipelinePlugin {
    fn build(&self, app: &mut App) {


        app
            .add_plugins((
                CoreMyPlugin,
                UpscalingPlugin,
            ))
            ;
    }
}
