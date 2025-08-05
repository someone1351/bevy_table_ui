
use bevy::platform::collections::{HashMap, HashSet};
use bevy::render::{
    batching::gpu_preprocessing::GpuPreprocessingMode,
    view::{ExtractedView, RetainedViewEntity},
};


use bevy::ecs::prelude::*;
use bevy::render::{
    camera::{Camera, ExtractedCamera},
    render_phase::{
        ViewBinnedRenderPhases,
        ViewSortedRenderPhases,
    },
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension,
        TextureUsages,
    },
    renderer::RenderDevice,
    texture::TextureCache,
    view::{Msaa, ViewDepthTexture},
    Extract,
};

use super::{phases::*, CORE_2D_DEPTH_FORMAT};
use super::camera::*;

pub fn extract_core_2d_camera_phases(
    mut transparent_2d_phases: ResMut<ViewSortedRenderPhases<TransparentMy>>,
    // mut my_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
    mut opaque_2d_phases: ResMut<ViewBinnedRenderPhases<OpaqueMy>>,
    mut alpha_mask_2d_phases: ResMut<ViewBinnedRenderPhases<AlphaMaskMy>>,
    cameras_2d: Extract<Query<(Entity, &Camera), With<CameraMy>>>,
    mut live_entities: Local<HashSet<RetainedViewEntity>>,
) {
    live_entities.clear();

    for (main_entity, camera) in &cameras_2d {
        if !camera.is_active {
            continue;
        }

        // This is the main 2D camera, so we use the first subview index (0).
        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);

        transparent_2d_phases.insert_or_clear(retained_view_entity);
        // my_render_phases.insert_or_clear(retained_view_entity); //
        opaque_2d_phases.prepare_for_new_frame(retained_view_entity, GpuPreprocessingMode::None);
        alpha_mask_2d_phases
            .prepare_for_new_frame(retained_view_entity, GpuPreprocessingMode::None);

        live_entities.insert(retained_view_entity);
    }

    // Clear out all dead views.
    transparent_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
    // my_render_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
    opaque_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
    alpha_mask_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
}

pub fn prepare_core_2d_depth_textures(
    mut commands: Commands,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
    transparent_2d_phases: Res<ViewSortedRenderPhases<TransparentMy>>,
    // my_render_phases: Res<ViewSortedRenderPhases<MyTransparentUi>>,
    opaque_2d_phases: Res<ViewBinnedRenderPhases<OpaqueMy>>,
    views_2d: Query<(Entity, &ExtractedCamera, &ExtractedView, &Msaa), (With<CameraMy>,)>,
) {
    let mut textures = <HashMap<_, _>>::default();
    for (view, camera, extracted_view, msaa) in &views_2d {
        if !opaque_2d_phases.contains_key(&extracted_view.retained_view_entity)
            || !transparent_2d_phases.contains_key(&extracted_view.retained_view_entity)
            // || !my_render_phases.contains_key(&extracted_view.retained_view_entity)

        {
            continue;
        };

        let Some(physical_target_size) = camera.physical_target_size else {
            continue;
        };

        let cached_texture = textures
            .entry(camera.target.clone())
            .or_insert_with(|| {
                // The size of the depth texture
                let size = Extent3d {
                    depth_or_array_layers: 1,
                    width: physical_target_size.x,
                    height: physical_target_size.y,
                };

                let descriptor = TextureDescriptor {
                    label: Some("view_depth_texture"),
                    size,
                    mip_level_count: 1,
                    sample_count: msaa.samples(),
                    dimension: TextureDimension::D2,
                    format: CORE_2D_DEPTH_FORMAT,
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                };

                texture_cache.get(&render_device, descriptor)
            })
            .clone();

        commands
            .entity(view)
            .insert(ViewDepthTexture::new(cached_texture, Some(0.0)));
    }
}




//camera


// #[derive(Component,Clone)]
// pub struct MyCameraView(pub Entity);

// pub fn extract_camera_view(
//     mut commands: Commands,
//     // mut my_render_phases: ResMut<ViewSortedRenderPhases<TransparentMy>>,
//     // // camera_query: Extract<Query<(RenderEntity, &Camera), With<CameraTest>, >>,
//     camera_query: Extract<Query<(RenderEntity, &Camera), With<CameraMy>, >>,
//     // : Extract<Query<(Entity, &Camera), With<CameraMy>>>,

//     // // camera_query: Extract<Query<(RenderEntity, &Camera), With<CameraMyTest>, >>,
//     // // mut live_camera_entities: Local<EntityHashSet>,
//     // mut live_camera_entities: Local<HashSet<RetainedViewEntity>>,

// ) {
//     //what are MainEntity and RenderEntity?
//     //why does viewport xy not being zero, not render scene at its topleft?
//     //  probably something to do with using Camera2d/3d, maybe should use own

//     const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;
//     const UI_CAMERA_FAR: f32 = 1000.0;

//     // live_camera_entities.clear();


//     // for (main_entity, camera) in &cameras_2d {
//     //     if !camera.is_active {
//     //         continue;
//     //     }
//     // }

//     for (camera_render_entity, camera) in &camera_query {
//         if !camera.is_active {
//             // let mut entity_commands = commands.get_entity(camera_render_entity).expect("Camera entity wasn't synced.");
//             // entity_commands.remove::<MyCameraView>();
//             continue;
//         }

//         /// The ID of the subview associated with a camera on which UI is to be drawn.
//         ///
//         /// When UI is present, cameras extract to two views: the main 2D/3D one and a
//         /// UI one. The main 2D or 3D camera gets subview 0, and the corresponding UI
//         /// camera gets this subview, 1.
//         const MYUI_CAMERA_SUBVIEW: u32 = 1;
//         let retained_view_entity = RetainedViewEntity::new(camera_render_entity.into(), None, MYUI_CAMERA_SUBVIEW); //needs main entity (not render entity)?

//         if let Some(physical_viewport_rect) = camera.physical_viewport_rect() {
//             let projection_matrix = Mat4::orthographic_rh(
//                 0.0,
//                 physical_viewport_rect.width() as f32,
//                 physical_viewport_rect.height() as f32,
//                 0.0,
//                 0.0,
//                 UI_CAMERA_FAR,
//             );
//             // println!("size {:?} {:?} {:?} {:?}",physical_viewport_rect,physical_viewport_rect.size(),physical_viewport_rect.width(),physical_viewport_rect.height());

//             // println!("projection_matrix {projection_matrix:?}");

//             // let view_entity =
//             commands.spawn((
//                 ExtractedView {
//                     clip_from_view: projection_matrix,
//                     world_from_view: GlobalTransform::from_xyz(0.0, 0.0, UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,),
//                     clip_from_world: None,
//                     hdr: camera.hdr,
//                     viewport: UVec4::from((
//                         // physical_viewport_rect.min,
//                         UVec2::ZERO,
//                         physical_viewport_rect.size(),
//                     )),
//                     color_grading: Default::default(),
//                     retained_view_entity, //added
//                 },
//                 TemporaryRenderEntity,
//             ))
//             // .id()
//             ;

//             // commands.get_entity(camera_render_entity).expect("Camera entity wasn't synced.")
//             //     .insert(MyCameraView(view_entity));

//             // // println!("camera_render_entity0 {camera_render_entity}");
//             // // println!("retained_view_entity0 {retained_view_entity:?}");
//             // // println!("view_entity0 {view_entity}");

//             // // my_render_phases.insert_or_clear(retained_view_entity); //camera_render_entity

//             // // live_camera_entities.insert(retained_view_entity); //camera_render_entity
//         }
//     }

//     // my_render_phases.retain(|camera_entity, _| live_camera_entities.contains(camera_entity));
// }
