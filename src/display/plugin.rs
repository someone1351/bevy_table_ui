use bevy::app::PostUpdate;
// use bevy::app::PostUpdate;
use bevy::ecs::prelude::*;
// use bevy::render;

// use crate::UiText;

use crate::display::render_upscaling::UpscalingPlugin;

use super::components::*;

use super::super::layout;
use super::render::render_setup;
// use super::TestRenderPlugin;
use super::render_core::CoreMyPlugin;
use super::systems::*;
// use super::render;

#[derive(Default)]
pub struct UiDisplayPlugin;

impl bevy::app::Plugin for UiDisplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app


            .add_plugins((CoreMyPlugin, UpscalingPlugin))
            // .add_systems(
            //     PostUpdate, (
            //         bevy::text::detect_text_needs_rerender::<MyText>,
            //         my_text_update_system2,
            //     ).chain()
            //         .after(bevy::text::remove_dropped_font_atlas_sets)
            // )
            // .add_systems(
            //     PostUpdate,
            //     (
            //         bevy::text::detect_text_needs_rerender::<UiText>,
            //         my_text_update_system2
            //     ).chain()
            //         .after(bevy::text::remove_dropped_font_atlas_sets)
            //         // .after(bevy::camera::CameraUpdateSystems)
            //         // .after(bevy::app::AnimationSystems)
            //         // calculate_bounds_text2d.in_set(VisibilityCalculateBounds),
            //     ,

            // )
            .add_systems(
                // bevy::app::Update,
                PostUpdate,
                (
                    update_image,
                    update_text_bounds, //need to run before text_bounds is checked for change
                    // update_text,
                    // update_text2,
                    // my_text_update_system2,
                    bevy::text::detect_text_needs_rerender::<UiText>,
                ).chain()
                    .after(layout::ui_init_computeds)
                    .before(layout::ui_calc_rows_cols) // layout::plugin::UiLayoutSystem
                    // .after(bevy::text::remove_dropped_font_atlas_sets)
                    .after(bevy::text::free_unused_font_atlases_system)
                    //
                    // .after(bevy::camera::CameraUpdateSystems)
                    // .after(bevy::app::AnimationSystems)
                    // // calculate_bounds_text2d.in_set(VisibilityCalculateBounds),
                    //
                ,
            )
        ;

    }
    fn finish(&self, app: &mut bevy::prelude::App) {
        // let render_app = match app.get_sub_app_mut(RenderApp) {
        //     Ok(render_app) => render_app,
        //     Err(_) => return,
        // };

        // render_app
        //     .init_resource::<RenderResourceNeedingDevice>();

        render_setup(app);
    }
}