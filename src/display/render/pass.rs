use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
// use bevy::render::camera::ExtractedCamera;
use bevy::render::render_graph::*;
use bevy::render::render_graph::Node;
use bevy::render::renderer::RenderContext;
use bevy::render::render_phase::*;
use bevy::render::view::*;
use bevy::render::render_resource::*;
use tracing::error;
use super::components::*;
use super::phase::*;
// use super::MyDefaultCameraView;

pub struct MyUiPassNode {
    // ui_view_query: QueryState<(&'static ViewTarget, &'static ExtractedCamera), With<ExtractedView>>,

    ui_view_query: QueryState<(&'static ExtractedView, &'static MyUiViewTarget)>,
    // ui_view_query: QueryState<(&'static ExtractedView, &'static ViewTarget)>,
    ui_view_target_query: QueryState<(&'static ViewTarget, &'static ExtractedCamera)>,
    default_camera_view_query: QueryState<&'static MyDefaultCameraView>,
}


impl MyUiPassNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            ui_view_query: world.query_filtered(),
            default_camera_view_query: world.query(),
            ui_view_target_query: world.query(),
        }
    }
}

impl Node for MyUiPassNode {
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
        //
        let input_view_entity = graph.view_entity();

        // Query the UI view components.
        let Ok((view, ui_view_target)) = self.ui_view_query.get_manual(world, input_view_entity)
        else {
            return Ok(());
        };

        let Ok((target, _camera)) = self
            .ui_view_target_query
            .get_manual(world, ui_view_target.0)
        else {
            return Ok(());
        };

        //
        let Some(transparent_render_phases) = world.get_resource::<ViewSortedRenderPhases<MyTransparentUi>>() else {
            return Ok(());
        };

        //
        let Some(transparent_phase) = transparent_render_phases.get(&view.retained_view_entity) else { //&input_view_entity
            return Ok(());
        };

        //
        // let Ok((target, _camera_ui)) = self.ui_view_query.get_manual(world, input_view_entity) else {
        //     return Ok(());
        // };

        //
        if transparent_phase.items.is_empty() {
            return Ok(());
        }

        // Don't render UI for cameras where it is explicitly disabled
        // if matches!(camera_ui, Some(&MyUiCameraConfig { show_ui: false })) {
        //     return Ok(());
        // }

        //
        let view_entity = self.default_camera_view_query.get_manual(world, input_view_entity)
            .and_then(|default_view|Ok(default_view.0))
            .unwrap_or(input_view_entity);

        //
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("my_ui_pass"),
            color_attachments: &[Some(target.get_unsampled_color_attachment())],
            depth_stencil_attachment: None,
            occlusion_query_set : None,
            timestamp_writes : None,
        });

        //
        if let Err(err) = transparent_phase.render(&mut render_pass, world, view_entity) {
            error!("Error encountered while rendering the ui phase {err:?}");
        }

        //
        Ok(())
    }
}