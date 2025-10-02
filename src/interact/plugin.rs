
use super::resources;
use super::messages;
// use super::components;
use super::systems;
use super::super::layout;

use bevy::ecs::prelude::*;


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiInteractSystem;

#[derive(Default)]
pub struct UiInteractPlugin;

impl bevy::app::Plugin for UiInteractPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app

            .add_message::<messages::UiInteractEvent>()
            .add_message::<messages::UiInteractInputMessage>()
            // .add_message::<UiCustomEvent>()UiPressStates

            .init_resource::<resources::UiFocusStates>()
            .init_resource::<resources::UiPressStates>()
            // .init_resource::<resources::UiDragStates>()


            .add_systems(bevy::app::PostUpdate, (
                (
                    // update_active_nodes,

                    systems::update_hover_events, //cursor(x,y)

                    systems::update_focus_events, //up/down/left/right/prev/next/ok/cancel
                    // systems::pre_update_press,
                    systems::update_press_events, //ok,cancel, cursor(x,y)
                    // systems::pre_update_drag,
                    systems::update_drag_events, //ok, cursor(mx,my)
                    systems::update_select_events, //

                    // update_scripting,
                    // forward_custom_events,

                    // ui_asset_modified,
                    // ui_asset_load,
                ).chain().in_set(UiInteractSystem)
                    .after(layout::plugin::UiLayoutSystem)

                    ,
            ))


            // .add_systems(PostUpdate, (
            //     update_text_image,
            // )

            //     .after(table_ui::ui_init_computeds)
            //     .before(table_ui::ui_calc_computeds),
            // )
            ;

    }
    // fn finish(&self, app: &mut bevy::prelude::App) {
    //     // render::setup(app);
    // }
}


        // let render_app = match app.get_sub_app_mut(RenderApp) {
        //     Ok(render_app) => render_app,
        //     Err(_) => return,
        // };

        // render_app
        //     .init_resource::<RenderResourceNeedingDevice>();

            // .add_systems(PostUpdate, (
            //     remover_system,
            // ).chain()
            //     .after(table_ui::ui_calc_computeds)
            // )
  // .add_message::<loader::UiAssetEvent>()
            // .add_message::<input_map::InputMapEvent<UiInputMapping>>()

            // .register_type::<UiText>()
            // .register_type::<UiImage>()
            // .register_type::<UiId>()

            // .register_type::<interact::UiFocusable>()

            // .init_resource::<UiCustomOutput>()


                // (
                //     ui_asset_modified,
                //     ui_asset_load,
                // ).chain().before(table_ui::ui_init_computeds),