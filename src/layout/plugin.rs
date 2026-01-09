
use bevy::app::PostUpdate;
// use bevy::app::Update;
// use bevy::app::prelude::*;
use bevy::ecs::prelude::*;

// use super::values::*;
use super::components::*;
use super::systems::*;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// #[derive(SystemSet)]
// pub enum MySystems {
//     SystemA,
//     SystemB,
//     SystemC,
// }

/*
TODO
* move to pre update
*/

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiLayoutSystem;

#[derive(Default)]
pub struct UiLayoutPlugin;

impl bevy::app::Plugin for UiLayoutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app

            .register_type::<UiEdge>()
            .register_type::<UiGap>()
            .register_type::<UiExpand>()
            .register_type::<UiFill>()
            .register_type::<UiScroll>()
            .register_type::<UiFloat>()
            .register_type::<UiHide>()
            .register_type::<UiSpan>()
            .register_type::<UiAlign>()
            .register_type::<UiSize>()
            // .register_type::<UiInnerSize>()
            .register_type::<UiLayoutComputed>()

            .add_systems(
                // bevy::app::Update,
                PostUpdate,
                (
                    ui_init_computeds,
                    ui_calc_rows_cols,
                    ui_calc_computeds2,
                    ui_calc_computeds3,
                    ui_calc_computed_pos,
                    ui_calc_computed_clamp,
                ).chain().in_set(UiLayoutSystem)
                    // .after(bevy::text::remove_dropped_font_atlas_sets)
                    // .after(bevy::app::AnimationSystems)
                    // .after(bevy::camera::CameraUpdateSystems)
                    // .after(bevy::asset::AssetEventSystems)
                )
                ;

    }
}
