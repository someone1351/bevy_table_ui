use bevy::app::PostUpdate;
// use bevy::app::PostUpdate;
use bevy::ecs::prelude::*;
// use bevy::render;

use crate::MyText;

use super::super::layout;
use super::render::render_setup;
// use super::TestRenderPlugin;
use super::render_core::CorePipelinePlugin;
use super::systems;
// use super::render;

#[derive(Default)]
pub struct UiDisplayPlugin;

impl bevy::app::Plugin for UiDisplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app

            // // .add_message::<loader::UiAssetEvent>()
            // // .add_message::<input_map::InputMapEvent<core::UiInputMapping>>()
            // // .register_type::<core::UiText>()
            // // .register_type::<core::UiImage>()
            // // .register_type::<core::UiId>()
            // // .register_type::<interact::UiFocusable>()
            // // .init_resource::<core::UiCustomOutput>()

            // .add_message::<core::UiEvent>()
            // .add_message::<core::UiInputEvent>()
            // .add_message::<core::UiCustomEvent>()


            // .init_resource::<core::UiGcScope>()
            // .init_resource::<core::UiLibScope>()
            // .init_resource::<core::UiConfDef>()
            // .init_resource::<core::UiActiveNodes>()
            // .init_resource::<core::UiFocusState>()



            // .add_systems(Startup, (
            //     core::init,
            // ))

            // .add_systems(PostUpdate, (
            //     // (
            //     //     core::ui_asset_modified,
            //     //     core::ui_asset_load,
            //     // ).chain().before(table_ui::ui_init_computeds),
            //     (
            //         core::update_active_nodes,

            //         core::update_hover_events, //cursor(x,y)
            //         // core::update_drag_events, //ok, cursor(mx,my)

            //         core::update_focus_events, //up/down/left/right/prev/next/ok/cancel
            //         core::update_press_events, //ok,cancel, cursor(x,y)
            //         core::update_select_events, //

            //         core::update_scripting,
            //         core::forward_custom_events,

            //         core::ui_asset_modified,
            //         core::ui_asset_load,
            //     ).chain().after(layout::systems::ui_calc_computeds),
            // ))


            // .add_systems(PostUpdate, (
            //     core::remover_system,
            // ).chain()
            //     .after(table_ui::ui_calc_computeds)
            // )
            //TestRenderPlugin,
            .add_plugins((CorePipelinePlugin, ))
            // .add_systems(
            //     PostUpdate, (
            //         bevy::text::detect_text_needs_rerender::<MyText>,
            //         systems::my_text_update_system2,
            //     ).chain()
            //         .after(bevy::text::remove_dropped_font_atlas_sets)
            // )
            .add_systems(
                PostUpdate,
                (
                    bevy::text::detect_text_needs_rerender::<MyText>,
                    systems::my_text_update_system2
                ).chain()
                    .after(bevy::text::remove_dropped_font_atlas_sets)
                    // .after(bevy::camera::CameraUpdateSystems)
                    // .after(bevy::app::AnimationSystems)
                    // calculate_bounds_text2d.in_set(VisibilitySystems::CalculateBounds),
                ,

            )
            .add_systems(
                // bevy::app::Update,
                PostUpdate,
                (
                    // bevy::text::detect_text_needs_rerender::<MyText>,
                    systems::update_image,
                    // systems::update_text_bounds, //need to run before text_bounds is checked for change
                    // systems::update_text,
                    // systems::my_text_update_system2,
                ).chain()
                    .after(layout::systems::ui_init_computeds)
                    .before(layout::systems::ui_calc_rows_cols) // layout::plugin::UiLayoutSystem
                    // .after(bevy::text::remove_dropped_font_atlas_sets)
                    //
                    // .after(bevy::camera::CameraUpdateSystems)
                    // .after(bevy::app::AnimationSystems)
                    // // calculate_bounds_text2d.in_set(VisibilitySystems::CalculateBounds),
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