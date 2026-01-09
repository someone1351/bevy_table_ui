
// use bevy::ecs::prelude::*;

use bevy::app::App;


use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::render::{render_phase::*, ExtractSchedule, Render, RenderApp, RenderSystems};



use bevy::render::render_resource::*;


use draws::DrawMesh;
use pipelines::MyUiPipeline;
use shaders::setup_shaders;

use super::render_core::core_my::TransparentMy;

// use bevy::transform::components::GlobalTransform;

//use crate::core::core_2d::mypass::MyTransparentUi;

// use bevy::prelude::*;
//render component
pub mod pipelines;
pub mod shaders;
pub mod draws;

pub mod components;
pub mod resources;
pub mod systems;
pub mod dummy_image;

use resources::*;
// use components::*;
use systems::*;


pub fn render_setup(app: &mut App) {
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();

    // let render_device=render_app.world().resource::<RenderDevice>();
    // let render_queue=render_app.world().resource::<RenderQueue>();

    // let gpu_image=create_dummy_image(&render_device,&render_queue);

    // let bind_group=render_device.create_bind_group(
    //     "my_ui_material_bind_group",
    //     &mesh2d_pipeline.image_layout, &[
    //         BindGroupEntry {binding: 0, resource: BindingResource::TextureView(&gpu_image.texture_view),},
    //         BindGroupEntry {binding: 1, resource: BindingResource::Sampler(&gpu_image.sampler),},
    //     ]
    // );
    // render_app.world_mut().insert_resource(value)
    render_app
        .init_resource::<MyUiMeta>()
        .init_resource::<MyUiExtractedElements>()
        .init_resource::<MyUiImageBindGroups>()
        .init_resource::<MyUiPipeline>()
        .init_resource::<SpecializedRenderPipelines<MyUiPipeline>>()
        .init_resource::<MySpriteAssetEvents>()

        // // .init_resource::<DrawFunctions<MyTransparentUi>>()
        // // .init_resource::<ViewSortedRenderPhases<MyTransparentUi>>()
        // .add_render_command::<MyTransparentUi, DrawMesh>()
        .add_render_command::<TransparentMy, DrawMesh>()
        // .add_systems(
        //     // Startup,
        //     ExtractSchedule,
        //     (dummy_image_setup,))
        .add_systems(ExtractSchedule,(
            // // extract_camera_view,
            dummy_image_setup,
            // extract_images,
            extract_images2,
            extract_sprite_events, //only works after extract_images, but then image bind_groups are then updated one frame late, think updating texture makes the iamge flash
            // extract_events,
            // extract_uinodes,
            extract_uinodes2,
        ).chain())
        .add_systems( Render,(
            queue_uinodes.in_set(RenderSystems::Queue),
            // // sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort),
            (
                prepare_views,
                prepare_uinodes,
            ).chain().in_set(RenderSystems::PrepareBindGroups),
        )) ;

    setup_shaders(app);

    // let render_app = app.get_sub_app_mut(RenderApp).unwrap();
    // // graphs::setup_graph2d(render_app);
    // // graphs::setup_graph3d(render_app);
    // graphs::setup_graph(render_app);

}


