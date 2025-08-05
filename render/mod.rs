// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// // #![allow(unused_imports)]
// #![allow(path_statements)]

/*
TODO
* remove usage of window size for root node, and replace with cameras viewport size?
* figure how ui works with multiple cameras

*/

use bevy::app::prelude::*;
use bevy::core_pipeline::{core_2d::graph::Node2d,core_3d::graph::Node3d};
use bevy::ecs::prelude::*;
// use bevy::ecs::system::SystemParamItem;

use bevy::asset::{load_internal_asset, weak_handle, Handle};


use bevy::app::App;

// use bevy::render::render_graph::{NodeRunError, RenderGraphContext};
// use bevy::render::renderer::RenderContext;
use bevy::render::{

    render_resource::*,

    render_graph::{RenderLabel, RenderSubGraph,RenderGraph,RunGraphOnViewNode},
    render_phase::*,
    Render,RenderApp, RenderSet, ExtractSchedule,
};

pub mod systems;
pub mod pipeline;
pub mod phase;
pub mod draw;
pub mod pass;
pub mod resources;
pub mod components;
pub mod camera;

// use components::MyDefaultCameraView;
use systems::*;
use pipeline::*;
use phase::*;
use draw::*;
use pass::*;
use resources::*;
// use components::*;
// use camera::*;

pub const COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> = weak_handle!("0a991ecd-134f-4f82-adf5-0fcc86f02227");
// pub const COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(5312396983770130001);

pub fn setup_shaders(app: &mut bevy::app::App) {
    load_internal_asset!(app, COLORED_MESH2D_SHADER_HANDLE, "mesh2d_col.wgsl", Shader::from_wgsl);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct MyUiPassNodeLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
struct MyUiSubGraphLabel;


/// A [`RenderGraphNode`] that executes the UI rendering subgraph on the UI
/// view.
// struct MyRunUiSubgraphOnUiViewNode;
// bevy::render::render_graph::node::RenderGraphNode
// impl bevy::render::render_graph::node::RenderGraphNode for MyRunUiSubgraphOnUiViewNode {
//     fn run<'w>(
//         &self,
//         graph: &mut RenderGraphContext,
//         _: &mut RenderContext<'w>,
//         world: &'w World,
//     ) -> Result<(), NodeRunError> {
//         // Fetch the UI view.
//         let Some(mut render_views) = world.try_query::<&MyDefaultCameraView>() else {
//             return Ok(());
//         };
//         let Ok(default_camera_view) = render_views.get(world, graph.view_entity()) else {
//             return Ok(());
//         };

//         // Run the subgraph on the UI view.
//         graph.run_sub_graph(MyUiSubGraphLabel, vec![], Some(default_camera_view.0))?;
//         Ok(())
//     }
// }

fn setup_graph2d(app: &mut App) {
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();
    let ui_pass_node = MyUiPassNode::new(render_app.world_mut());

    let mut ui_graph_2d = RenderGraph::default();
    ui_graph_2d.add_node( MyUiPassNodeLabel, ui_pass_node);

    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();

    if let Some(graph_2d) = render_graph.get_sub_graph_mut(bevy::core_pipeline::core_2d::graph::Core2d) {
        graph_2d.add_sub_graph(MyUiSubGraphLabel,ui_graph_2d);
        graph_2d.add_node(MyUiPassNodeLabel,RunGraphOnViewNode::new(MyUiSubGraphLabel),);
        // graph_2d.add_node(NodeUi::UiPass, MyRunUiSubgraphOnUiViewNode);
        graph_2d.add_node_edge(Node2d::EndMainPass, MyUiPassNodeLabel);
    }
}

fn setup_graph3d(app: &mut App) {
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();
    let ui_pass_node = MyUiPassNode::new(render_app.world_mut());

    let mut ui_graph_3d = RenderGraph::default();
    ui_graph_3d.add_node( MyUiPassNodeLabel, ui_pass_node);

    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();

    if let Some(graph_3d) = render_graph.get_sub_graph_mut(bevy::core_pipeline::core_3d::graph::Core3d) {
        graph_3d.add_sub_graph(MyUiSubGraphLabel , ui_graph_3d);
        graph_3d.add_node(MyUiPassNodeLabel,RunGraphOnViewNode::new(MyUiSubGraphLabel),);
        // graph_3d.add_node(NodeUi::UiPass, MyRunUiSubgraphOnUiViewNode);
        graph_3d.add_node_edge(Node3d::EndMainPass, MyUiPassNodeLabel);
    }
}



pub fn setup(app: &mut bevy::app::App) {
    app
        .add_systems(Startup, (dummy_image_setup,))
        .init_resource::<DummyImage>();

    // Register our custom draw function and pipeline, and add our render systems
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();

    render_app
        .init_resource::<MyUiMeta>()
        .init_resource::<MyUiExtractedElements>()
        .allow_ambiguous_resource::<MyUiExtractedElements>()
        .init_resource::<DummyGpuImage>()
        .init_resource::<MyUiImageBindGroups>()
        .init_resource::<MyUiPipeline>()
        .init_resource::<SpecializedRenderPipelines<MyUiPipeline>>()
        .init_resource::<DrawFunctions<MyTransparentUi>>()
        .init_resource::<ViewSortedRenderPhases<MyTransparentUi>>()
        .add_render_command::<MyTransparentUi, DrawMesh>()
        .add_systems(ExtractSchedule,(
            extract_default_ui_camera_view,
            extract_dummy_image_setup,
            extract_uinodes
        ).chain())
        .add_systems( Render,(
            queue_uinodes.in_set(RenderSet::Queue),
            sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort),
            prepare_uinodes.in_set(RenderSet::PrepareBindGroups),
        ))
    ;

    setup_shaders(app);
    setup_graph2d(app);
    setup_graph3d(app);
}

