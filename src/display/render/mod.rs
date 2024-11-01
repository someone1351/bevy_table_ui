#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(path_statements)]


// use bevy::prelude::{AssetEvent, Msaa};


use bevy::app::prelude::*;

use bevy::core_pipeline::core_2d::graph::Node2d;
use bevy::core_pipeline::core_3d::graph::Node3d;
use bevy::prelude::URect;
use bevy::render::Render;
// use bevy::prelude::IntoSystemAppConfig;
// use bevy::prelude::*;
use bevy::sprite::{self, TextureAtlas};
use bevy::ecs::prelude::*;
use bevy::ecs::system::{lifetimeless::*, SystemParamItem,SystemState};

use bevy::asset::{Assets, Handle, load_internal_asset};
use bevy::math::{Mat4, Vec2, UVec4};
// use bevy::reflect::TypeUuid;
use bevy::transform::components::GlobalTransform;
// use bevy::ui::RenderUiSystem;
use core::ops::Range;

// use bevy::core_pipeline::CorePipelineRenderSystems;

use bevy::render::{
    // color::Color,
    camera::{CameraPlugin,Camera,CameraProjection,OrthographicProjection},
    mesh::{GpuBufferInfo, Mesh},
    render_asset::RenderAssets,
    // render_component::{ComponentUniforms, DynamicUniformIndex, UniformComponentPlugin},

    render_resource::*,//{std140::AsStd140 , *},
    renderer::{RenderContext,RenderDevice, RenderQueue},
    texture::{BevyDefault, GpuImage, Image, TextureFormatPixelInfo,DefaultImageSampler, ImageSampler},
    view::{ ExtractedView, ViewUniform, ViewUniformOffset, ViewUniforms,ViewTarget},
    render_graph::{RenderGraph,SlotType,SlotInfo,Node, NodeRunError, RenderGraphContext, SlotValue,RunGraphOnViewNode},
    render_phase::*,
    RenderApp, //RenderStage,
    Extract, RenderSet, ExtractSchedule,
};
// use bevy::core::FloatOrd;
use std::cmp::Ordering;

use bevy::text::{FontAtlasSet, Font, };
use bevy::utils::HashMap;

use bevy::app::App;

use bevy::core_pipeline::{core_2d::Camera2d,core_3d::Camera3d};

pub mod systems;
pub mod pipeline;
pub mod phase;
pub mod draw;
pub mod pass;
pub mod resources;
pub mod components;
pub mod utils;
pub mod camera;

use systems::*;
use pipeline::*;
use phase::*;
use draw::*;
use pass::*;
use resources::*;
use components::*;
use utils::*;
use camera::*;
// use super::{UiComputed,UiColor,UiSize,UiImage,UiText, UiBorder, Val};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum MyRenderUiSystem {
    ExtractNode,
}
// NOTE: These must match the bit flags in bevy_sprite/src/mesh2d/mesh2d.wgsl!
// bitflags::bitflags! {
//     #[repr(transparent)]
//     struct MyModelFlags: u32 { const NONE = 0; const UNINITIALIZED = 0xFFFF; }
// }


// pub fn extract_core_pipeline_camera_phases(
//     mut commands: Commands,
//     active_cameras: Res<ActiveCamera<MyCameraUi>>,
// ) {
//     // if let Some(my_camera_ui) = active_cameras.get(CAMERA_UI) {
//     //     if let Some(entity) = my_camera_ui.entity {
//     //         commands.get_or_spawn(entity).insert(RenderPhase::<MyTransparentUi>::default());
//     //     }
//     // }
//     if let Some(entity) = active_cameras.get() {
//         commands
//             .get_or_spawn(entity)
//             .insert(RenderPhase::<MyTransparentUi>::default());
//     }
// }


/// Handle to the custom shader with a unique random ID
pub const COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> =
    // HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845428412094839);
    Handle::weak_from_u128(5312396983770130001);



pub fn setup_shaders(app: &mut bevy::app::App) {

    // using `include_str!()`, or loaded like any other asset with `asset_server.load()`.
    // let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
    // shaders.set_untracked( COLORED_MESH2D_SHADER_HANDLE,Shader::from_wgsl(include_str!("mesh2d_col.wgsl")),);


    load_internal_asset!(app, COLORED_MESH2D_SHADER_HANDLE, "mesh2d_col.wgsl", Shader::from_wgsl);
}


fn get_ui_graph(render_app: &mut SubApp) -> RenderGraph {
    // let ui_pass_node = MyUiPassNode::new(&mut render_app.world);
    let ui_pass_node = MyUiPassNode::new(render_app.world_mut());

    let mut ui_graph = RenderGraph::default();

    ui_graph.add_node(
        // my_draw_ui_graph::node::UI_PASS
        my_draw_ui_graph::NodeUi::UiPass
        , ui_pass_node);

    // let input_node_id = ui_graph.set_input(vec![SlotInfo::new(
    //     my_draw_ui_graph::input::VIEW_ENTITY,
    //     SlotType::Entity,
    // )]);

    // ui_graph
    //     .add_slot_edge(
    //         input_node_id,
    //         my_draw_ui_graph::input::VIEW_ENTITY,
    //         my_draw_ui_graph::node::UI_PASS,
    //         MyUiPassNode::IN_VIEW,
    //     );

    ui_graph
}

fn setup_graph(app: &mut App) {            
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();

    // let pass_node_2d = MyUiPassNode::new(&mut render_app.world);
    
    // let mut draw_2d_graph = RenderGraph::default();

    // draw_2d_graph.add_node(my_draw_ui_graph::node::MAIN_PASS, pass_node_2d);
    
    // let input_node_id = draw_2d_graph.set_input(vec![SlotInfo::new(
    //     my_draw_ui_graph::input::VIEW_ENTITY,
    //     SlotType::Entity,
    // )]);

    // draw_2d_graph .add_slot_edge(
    //     input_node_id,
    //     my_draw_ui_graph::input::VIEW_ENTITY,
    //     my_draw_ui_graph::node::MAIN_PASS,
    //     MyUiPassNode::IN_VIEW,
    // ).unwrap();

    let ui_graph_2d = get_ui_graph(render_app);
    let ui_graph_3d = get_ui_graph(render_app);
    // let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
    let mut graph = render_app.world_mut().resource_mut::<RenderGraph>();


    // graph.add_sub_graph(my_draw_ui_graph::NAME, draw_2d_graph);
    // graph.add_node(my_ui_node::UI_PASS_DRIVER, MyUiPassDriverNode);
    // graph.add_node_edge(MAIN_PASS_DRIVER,my_ui_node::UI_PASS_DRIVER).unwrap();

    
    if let Some(graph_2d) = graph.get_sub_graph_mut(bevy::core_pipeline::core_2d::graph::Core2d) {
        graph_2d.add_sub_graph(
            // my_draw_ui_graph::NAME,
            my_draw_ui_graph::SubGraphUi,
             ui_graph_2d);
        graph_2d.add_node(
            // my_draw_ui_graph::node::UI_PASS,
            my_draw_ui_graph::NodeUi::UiPass,
            RunGraphOnViewNode::new(
                // my_draw_ui_graph::NAME
                my_draw_ui_graph::SubGraphUi
            ),
        );
        // graph_2d.add_node_edge(
        //     bevy::core_pipeline::core_2d::graph::Node2d::MainPass,
        //     // bevy::core_pipeline::core_2d::graph::node::MAIN_PASS,
        //     // my_draw_ui_graph::node::UI_PASS, //my_draw_ui_graph::NodeUi::UiPass,
        //     my_draw_ui_graph::NodeUi::UiPass,
        // );

        //
        
        graph_2d.add_node_edge(Node2d::EndMainPass, my_draw_ui_graph::NodeUi::UiPass);
        graph_2d.add_node_edge(Node2d::EndMainPassPostProcessing, my_draw_ui_graph::NodeUi::UiPass);
        graph_2d.add_node_edge(my_draw_ui_graph::NodeUi::UiPass, Node2d::Upscaling);

        //
        // graph_2d.add_node_edge(
        //     bevy::core_pipeline::core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING,
        //     my_draw_ui_graph::node::UI_PASS,
        // );
        // graph_2d.add_node_edge(
        //     my_draw_ui_graph::node::UI_PASS,
        //     bevy::core_pipeline::core_2d::graph::node::UPSCALING,
        // );

        // graph_2d
        //     .add_slot_edge(
        //         graph_2d.input_node().id,
        //         bevy::core_pipeline::core_2d::graph::input::VIEW_ENTITY,
        //         my_draw_ui_graph::node::UI_PASS,
        //         RunGraphOnViewNode::IN_VIEW,
        //     );
    }

    if let Some(graph_3d) = graph.get_sub_graph_mut(bevy::core_pipeline::core_3d::graph::Core3d) {
        graph_3d.add_sub_graph(
            // my_draw_ui_graph::NAME
            my_draw_ui_graph::SubGraphUi
            , ui_graph_3d);
        graph_3d.add_node(
            // my_draw_ui_graph::node::UI_PASS,
            my_draw_ui_graph::NodeUi::UiPass,
            RunGraphOnViewNode::new(
                // my_draw_ui_graph::NAME
                my_draw_ui_graph::SubGraphUi
            ),
        );
        // graph_3d.add_node_edge(
        //     bevy::core_pipeline::core_3d::graph::Node3d::EndMainPass,
        //     // bevy::core_pipeline::core_3d::graph::node::END_MAIN_PASS,//MAIN_PASS,
        //     // my_draw_ui_graph::node::UI_PASS,
        //     my_draw_ui_graph::NodeUi::UiPass,
        // );

        graph_3d.add_node_edge(Node3d::EndMainPass, my_draw_ui_graph::NodeUi::UiPass);
        graph_3d.add_node_edge(Node3d::EndMainPassPostProcessing, my_draw_ui_graph::NodeUi::UiPass);
        graph_3d.add_node_edge(my_draw_ui_graph::NodeUi::UiPass, Node3d::Upscaling);

        //
        // graph_3d.add_node_edge(
        //     bevy::core_pipeline::core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
        //     my_draw_ui_graph::node::UI_PASS,
        // );
        // graph_3d.add_node_edge(
        //     my_draw_ui_graph::node::UI_PASS,
        //     bevy::core_pipeline::core_3d::graph::node::UPSCALING,
        // );
        // graph_3d
        //     .add_slot_edge(
        //         graph_3d.input_node().id,
        //         bevy::core_pipeline::core_3d::graph::input::VIEW_ENTITY,
        //         my_draw_ui_graph::node::UI_PASS,
        //         RunGraphOnViewNode::IN_VIEW,
        //     );
    }

    //todo add addtional node edges from crates\bevy_ui\src\render\mod.rs
    
}


// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
// pub enum MyCorePipelineRenderSystems {
//     SortTransparent2d,
// }


// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
// pub enum RenderUiSystem {
//     ExtractNode,
// }

///////

pub fn setup(app: &mut bevy::app::App) {
    //let mut active_cameras = app.world.get_resource_mut::<ActiveCamera<MyCameraUi>>().unwrap();
//    return;
    //active_cameras.add(CAMERA_UI);

    app
        .add_systems(Startup, (dummy_image_setup,))
        .init_resource::<DummyImage>();

    // app.add_startup_system(dummy_image_setup)
    //     .init_resource::<DummyImage>();

    // Register our custom draw function and pipeline, and add our render systems
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();
 
    render_app
        ////.init_resource::<MyImageBindGroup>()
        .init_resource::<MyUiMeta>()
        .init_resource::<MyUiExtractedElements>()
        .init_resource::<DummyGpuImage>()
        .init_resource::<MyUiImageBindGroups>()
        .init_resource::<MyUiPipeline>()
        .init_resource::<SpecializedRenderPipelines<MyUiPipeline>>()
        .init_resource::<DrawFunctions<MyTransparentUi>>()
        .init_resource::<ViewSortedRenderPhases<MyTransparentUi>>()
        .add_render_command::<MyTransparentUi, DrawMesh>()
        
        .add_systems(ExtractSchedule,(
            // extract_default_ui_camera_view::<Camera2d>,
            // extract_default_ui_camera_view::<Camera3d>,
            extract_default_ui_camera_view,
            extract_dummy_image_setup,
            extract_uinodes
        ).chain())
        
        .add_systems( Render,(
            // ( 
            //     prepare_uinodes,
            //     // queue_view_bind_groups, 
            //     // queue_image_bind_group,
            // ).chain()
            // .in_set(RenderSet::Prepare),
            // .in_set(RenderSet::PrepareBindGroups),

            // prepare_uinodes.in_set(RenderSet::Prepare),

            queue_uinodes.in_set(RenderSet::Queue), 
            sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort),
            prepare_uinodes.in_set(RenderSet::PrepareBindGroups),
            
        ))


        // .add_system((extract_default_ui_camera_view::<Camera2d>).in_schedule(ExtractSchedule))
        // .add_systems(prepare_uinodes.in_set(RenderSet::Prepare))
        // .add_system(sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort))
        ;


    setup_shaders(app);
    setup_graph(app);


    // let mut images = render_app.world.get_resource_mut::<Assets<Image>>().unwrap();
}



        // .add_system_to_stage(
        //     RenderStage::Extract,
        //     extract_default_ui_camera_view::<Camera2d>,
        // )
        // .add_system_to_stage(
        //     RenderStage::Extract,
        //     extract_default_ui_camera_view::<Camera3d>,
        // )
        
        // // .add_system_to_stage(RenderStage::Extract, extract_core_pipeline_camera_phases)

            //
        // .add_system_to_stage(RenderStage::PhaseSort, sort_phase_system::<MyTransparentUi>
        //     .label(MyCorePipelineRenderSystems::SortTransparent2d))
        // .add_system_to_stage(RenderStage::PhaseSort, batch_phase_system::<MyTransparentUi>
        //     .after(MyCorePipelineRenderSystems::SortTransparent2d))
        // .add_system_to_stage(RenderStage::PhaseSort,sort_phase_system::<MyTransparentUi>)
        // .add_system_to_stage(RenderStage::PhaseSort,batch_phase_system::<MyTransparentUi>)
 
        // .add_system_to_stage(RenderStage::Extract, extract_uinodes.after(extract_dummy_image_setup)) //.label(RenderUiSystem::ExtractNode)
        // .add_system_to_stage(RenderStage::Extract, extract_dummy_image_setup)
        // .add_system_to_stage(RenderStage::Prepare, prepare_uinodes)
        
            //
        // .add_system_to_stage(RenderStage::Queue, queue_view_bind_groups)
        // .add_system_to_stage(RenderStage::Queue, queue_image_bind_group)
        // .add_system_to_stage(RenderStage::Queue, queue_mesh_draw) 

