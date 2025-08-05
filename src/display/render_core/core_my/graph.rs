


use bevy::render::render_graph::{EmptyNode, RenderGraphApp, ViewNodeRunner};


use bevy::{app::SubApp, render::render_graph::{RenderLabel, RenderSubGraph}};

use super::super::upscaling::UpscalingNode;

use super::passes::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
pub struct CoreMy;

// pub mod input {
//     pub const VIEW_ENTITY: &str = "view_entity";
// }

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub enum NodeMy {
    // MsaaWriteback,
    StartMainPass,
    // MainOpaquePass,
    MainTransparentPass,
    EndMainPass,
    Upscaling,
    // EndMainPassPostProcessing,
}

pub fn setup_graph(render_app:&mut SubApp) {

    render_app
        .add_render_sub_graph(CoreMy)
        .add_render_graph_node::<EmptyNode>(CoreMy, NodeMy::StartMainPass)
        // // .add_render_graph_node::<ViewNodeRunner<MainOpaquePass2dNode>>(Core2d,Node2d::MainOpaquePass,)
        .add_render_graph_node::<ViewNodeRunner<MainTransparentPassMyNode>>(CoreMy,NodeMy::MainTransparentPass,)
        // .add_render_graph_node::<ViewNodeRunner<MyMainTransparentPass2dNode>>(Core2d,Node2d::MyMainTransparentPass)
        .add_render_graph_node::<EmptyNode>(CoreMy, NodeMy::EndMainPass)
        // // // .add_render_graph_node::<ViewNodeRunner<TonemappingNode>>(Core2d, Node2d::Tonemapping)
        // // // .add_render_graph_node::<EmptyNode>(Core2d, Node2d::EndMainPassPostProcessing)
        .add_render_graph_node::<ViewNodeRunner<UpscalingNode>>(CoreMy, NodeMy::Upscaling)
        .add_render_graph_edges(
            CoreMy,
            (
                NodeMy::StartMainPass,
                // // Node2d::MainOpaquePass,
                NodeMy::MainTransparentPass,
                // Node2d::MyMainTransparentPass,
                NodeMy::EndMainPass,
                // // // Node2d::Tonemapping,
                // // // Node2d::EndMainPassPostProcessing,
                NodeMy::Upscaling,
            ),
        );
}