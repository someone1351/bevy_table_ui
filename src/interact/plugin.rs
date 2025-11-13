
use super::resources;
use super::messages;
// use super::components;
use super::systems::*;
// use super::super::layout;

use bevy::app::*;
// use bevy::app::Update;
use bevy::ecs::prelude::*;
// use bevy::input::InputSystems;


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

            .init_resource::<resources::FocusStates>()
            .init_resource::<resources::FocusMoveHists>()

            .init_resource::<resources::FocusDevicePresseds>()
            .init_resource::<resources::CursorDevicePresseds>()
            .init_resource::<resources::CursorDevicePointers>()


            .init_resource::<resources::UiPressStates>()
            // .init_resource::<resources::UiDragStates>()


            .add_systems(Update, (
                (
                    // update_active_nodes,

                    update_hover_events, //cursor(x,y)

                    focus_move_cleanup,
                    focus_press_cleanup,
                    focus_update_press_events,
                    update_focus_events, //up/down/left/right/prev/next/ok/cancel

                    cursor_press_cleanup,
                    update_press_events, //ok,cancel, cursor(x,y)

                    update_drag_events, //ok, cursor(mx,my)
                    update_select_events, //

                    // pre_update_press,
                    // pre_update_drag,
                    // update_scripting,
                    // forward_custom_events,
                    // ui_asset_modified,
                    // ui_asset_load,
                ).chain().in_set(UiInteractSystem)
                    // .before(layout::plugin::UiLayoutSystem)
                    // // .after(InputSystems)

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