

use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
use bevy::render::render_graph::*;
use bevy::render::render_graph::Node;
use bevy::render::renderer::RenderContext;
use bevy::render::render_phase::*;
use bevy::render::view::*;
use bevy::render::render_resource::*;
use super::components::*;
use super::phase::*;
use super::MyDefaultCameraView;

//#[derive(RenderLabel)]
pub mod my_draw_ui_graph {
    use bevy::render::render_graph::{RenderLabel, RenderSubGraph};

    pub const NAME: &str = "my_draw_ui";
    pub mod input {
        // pub const VIEW_ENTITY: &str = "my_view_entity";
        pub const VIEW_ENTITY: &str = "view_entity";
    }
    
    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
    pub struct SubGraphUi;
    
    pub mod node {
        

        pub const UI_PASS: &str = "my_ui_pass";
    }

    
    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
    pub enum NodeUi {
        UiPass,
    }
}

pub struct MyUiPassNode {
    // ui_view_query: QueryState<
    //     (
    //         &'static RenderPhase<MyTransparentUi>,
    //         &'static ViewTarget,
    //         Option<&'static MyUiCameraConfig>,
    //     ),
    //     With<ExtractedView>,
    // >,
    ui_view_query: QueryState<(&'static ViewTarget, &'static ExtractedCamera), With<ExtractedView>>,
    default_camera_view_query: QueryState<&'static MyDefaultCameraView>,
    
}


impl MyUiPassNode {
    // pub const IN_VIEW: &'static str = "my_view";
    // pub const IN_VIEW: &'static str = "view";


    pub fn new(world: &mut World) -> Self {
        Self {
            ui_view_query: world.query_filtered(),
            default_camera_view_query: world.query(),
        }
    }
}

impl Node for MyUiPassNode {

    // fn input(&self) -> Vec<SlotInfo> {
    //     vec![SlotInfo::new(MyUiPassNode::IN_VIEW, SlotType::Entity)]
    // }

    fn update(&mut self, world: &mut World) {
        self.ui_view_query.update_archetypes(world);
        self.default_camera_view_query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        
        let input_view_entity = graph.view_entity();

        //
        

        //
        // let Ok((transparent_phase, target, camera_ui)) = 
        //     self.ui_view_query.get_manual(world, input_view_entity)
        // else {return Ok(())};

        
        let Some(transparent_render_phases) = world.get_resource::<ViewSortedRenderPhases<MyTransparentUi>>() else {
            return Ok(());
        };
        
        let Some(transparent_phase) = transparent_render_phases.get(&input_view_entity) else {
            return Ok(());
        };

        let Ok((target, camera_ui)) = self.ui_view_query.get_manual(world, input_view_entity) else {
            return Ok(());
        };
        //
        if transparent_phase.items.is_empty() {
            return Ok(());
        }

        // Don't render UI for cameras where it is explicitly disabled
        // if matches!(camera_ui, Some(&MyUiCameraConfig { show_ui: false })) {
        //     return Ok(());
        // }

        let view_entity = self.default_camera_view_query.get_manual(world, input_view_entity)
            .and_then(|default_view|Ok(default_view.0))
            .unwrap_or(input_view_entity);

        //
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("my_ui_pass"),
            color_attachments: &[Some(target.get_unsampled_color_attachment(
                //Operations { load: LoadOp::Load, store: true, }
            ))],
            depth_stencil_attachment: None,
                // Some(RenderPassDepthStencilAttachment { 
                //     view: self.view, 
                //     depth_ops: Some(Operations {
                //         load: LoadOp::Clear(0.0),
                //         store: StoreOp::Store,
                //     }),
                //     stencil_ops: None,
                // }),
            occlusion_query_set : None,
            timestamp_writes : None,
        });
        // println!("pass {:?}",transparent_phase.iter_entities().collect::<Vec<_>>());
        transparent_phase.render(&mut render_pass, world, view_entity);

        Ok(())
    }
}