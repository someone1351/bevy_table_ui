use bevy::{ecs::{entity::EntityHashSet, system::SystemParam}, prelude::*, render::{camera::RenderTarget, render_phase::ViewSortedRenderPhases, view::ExtractedView, Extract}, window::{PrimaryWindow, WindowRef}};

use super::MyTransparentUi;


/// Indicates that this root [`Node`] entity should be rendered to a specific camera.
/// UI then will be laid out respecting the camera's viewport and scale factor, and
/// rendered to this camera's [`bevy_render::camera::RenderTarget`].
///
/// Setting this component on a non-root node will have no effect. It will be overridden
/// by the root node's component.
///
/// Optional if there is only one camera in the world. Required otherwise.
#[derive(Component, Clone, Debug, Reflect, Eq, PartialEq)]
pub struct MyTargetCamera(pub Entity);

impl MyTargetCamera {
    pub fn entity(&self) -> Entity {
        self.0
    }
}

#[derive(Component)]
pub struct IsMyDefaultUiCamera;

#[derive(SystemParam)]
pub struct MyDefaultUiCamera<'w, 's> {
    cameras: Query<'w, 's, (Entity, &'static Camera)>,
    default_cameras: Query<'w, 's, Entity, (With<Camera>, With<IsMyDefaultUiCamera>)>,
    primary_window: Query<'w, 's, Entity, With<PrimaryWindow>>,
}

impl<'w, 's> MyDefaultUiCamera<'w, 's> {
    pub fn get(&self) -> Option<Entity> {
        self.default_cameras.get_single().ok().or_else(|| {
            // If there isn't a single camera and the query isn't empty, there is two or more cameras queried.
            if !self.default_cameras.is_empty() {
                warn_once!("Two or more Entities with IsDefaultUiCamera found when only one Camera with this marker is allowed.");
            }
            self.cameras
                .iter()
                .filter(|(_, c)| match c.target {
                    RenderTarget::Window(WindowRef::Primary) => true,
                    RenderTarget::Window(WindowRef::Entity(w)) => {
                        self.primary_window.get(w).is_ok()
                    }
                    _ => false,
                })
                .max_by_key(|(e, c)| (c.order, *e))
                .map(|(e, _)| e)
        })
    }
}











/// The UI camera is "moved back" by this many units (plus the [`UI_CAMERA_TRANSFORM_OFFSET`]) and also has a view
/// distance of this many units. This ensures that with a left-handed projection,
/// as ui elements are "stacked on top of each other", they are within the camera's view
/// and have room to grow.
// TODO: Consider computing this value at runtime based on the maximum z-value.
const UI_CAMERA_FAR: f32 = 1000.0;

// This value is subtracted from the far distance for the camera's z-position to ensure nodes at z == 0.0 are rendered
// TODO: Evaluate if we still need this.
const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;

#[derive(Component)]
pub struct MyDefaultCameraView(pub Entity);

/// Extracts all UI elements associated with a camera into the render world.
pub fn extract_default_ui_camera_view(
    mut commands: Commands,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
    ui_scale: Extract<Res<UiScale>>,
    query: Extract<Query<(Entity, &Camera), Or<(With<Camera2d>, With<Camera3d>)>>>,
    mut live_entities: Local<EntityHashSet>,
) {
    live_entities.clear();

    let scale = ui_scale.0.recip();
    for (entity, camera) in &query {
        // ignore inactive cameras
        if !camera.is_active {
            continue;
        }

        if let (
            Some(logical_size),
            Some(URect {
                min: physical_origin,
                ..
            }),
            Some(physical_size),
        ) = (
            camera.logical_viewport_size(),
            camera.physical_viewport_rect(),
            camera.physical_viewport_size(),
        ) {
            // use a projection matrix with the origin in the top left instead of the bottom left that comes with OrthographicProjection
            let projection_matrix = Mat4::orthographic_rh(
                0.0,
                logical_size.x * scale,
                logical_size.y * scale,
                0.0,
                0.0,
                UI_CAMERA_FAR,
            );
            let default_camera_view = commands
                .spawn(ExtractedView {
                    clip_from_view: projection_matrix,
                    world_from_view: GlobalTransform::from_xyz(
                        0.0,
                        0.0,
                        UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,
                    ),
                    clip_from_world: None,
                    hdr: camera.hdr,
                    viewport: UVec4::new(
                        physical_origin.x,
                        physical_origin.y,
                        physical_size.x,
                        physical_size.y,
                    ),
                    color_grading: Default::default(),
                })
                .id();
            commands
                .get_or_spawn(entity)
                .insert(MyDefaultCameraView(default_camera_view));
            transparent_render_phases.insert_or_clear(entity);

            live_entities.insert(entity);
        }
    }

    transparent_render_phases.retain(|entity, _| live_entities.contains(entity));
}










/*
pub fn extract_default_ui_camera_view<T: Component>(
    mut commands: Commands,
    query: Extract<Query<(Entity, &Camera, Option<&MyUiCameraConfig>), With<T>>>,
) {
    
/// The UI camera is "moved back" by this many units (plus the [`UI_CAMERA_TRANSFORM_OFFSET`]) and also has a view
/// distance of this many units. This ensures that with a left-handed projection,
/// as ui elements are "stacked on top of each other", they are within the camera's view
/// and have room to grow.
// TODO: Consider computing this value at runtime based on the maximum z-value.

// This value is subtracted from the far distance for the camera's z-position to ensure nodes at z == 0.0 are rendered
// TODO: Evaluate if we still need this.


    const UI_CAMERA_FAR: f32 = 1000.0;
    const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;
    for (entity, camera, camera_ui) in query.iter() {
        // ignore cameras with disabled ui
        if matches!(camera_ui, Some(&MyUiCameraConfig { show_ui: false, .. })) {
            continue;
        }
        if let (Some(logical_size), Some(physical_size)) = (
            camera.logical_viewport_size(),
            camera.physical_viewport_size(),
        ) {
            let mut projection = OrthographicProjection {
                far: UI_CAMERA_FAR,
                window_origin: WindowOrigin::BottomLeft,
                depth_calculation: DepthCalculation::ZDifference,
                ..Default::default()
            };
            projection.update(logical_size.x, logical_size.y);
            let default_camera_view = commands
                .spawn(())
                .insert(ExtractedView {
                    projection: projection.get_projection_matrix(),
                    transform: GlobalTransform::from_xyz(
                        0.0,
                        0.0,
                        UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,
                    ),
                    width: physical_size.x,
                    height: physical_size.y,
                })
                .id();
            commands.get_or_spawn(entity).insert_bundle((
                MyDefaultCameraView(default_camera_view),
                RenderPhase::<MyTransparentUi>::default(),
            ));
        }
    }
}

*/


// // #[derive(Component)]
// // pub struct DefaultCameraView(pub Entity);

// pub fn extract_default_ui_camera_view( //<T: Component>
//     mut commands: Commands,
//     // query: Extract<Query<(Entity, &Camera, Option<&MyUiCameraConfig>), With<T>>>,

    
//     query: Extract<Query<(Entity, &Camera), Or<(With<Camera2d>, With<Camera3d>)>>>,
//     mut transparent_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
// ) {
    

//     /// The UI camera is "moved back" by this many units (plus the [`UI_CAMERA_TRANSFORM_OFFSET`]) and also has a view
//     /// distance of this many units. This ensures that with a left-handed projection,
//     /// as ui elements are "stacked on top of each other", they are within the camera's view
//     /// and have room to grow.
//     // TODO: Consider computing this value at runtime based on the maximum z-value.
//     const UI_CAMERA_FAR: f32 = 1000.0;

//     // This value is subtracted from the far distance for the camera's z-position to ensure nodes at z == 0.0 are rendered
//     // TODO: Evaluate if we still need this.
//     const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;

//     for (entity, camera, camera_ui) in &query {
//         // ignore cameras with disabled ui
//         if matches!(camera_ui, Some(&MyUiCameraConfig { show_ui: false, .. })) {
//             continue;
//         }
//         if let (Some(logical_size), 
//             Some(URect { min: physical_origin, .. }),
//             Some(physical_size)) 
//         = (
//             camera.logical_viewport_size(),
//             camera.physical_viewport_rect(),
//             camera.physical_viewport_size(),
//         ) {
//             // use a projection matrix with the origin in the top left instead of the bottom left that comes with OrthographicProjection
//             let projection_matrix =
//                 Mat4::orthographic_rh(
//                     // 0.0, logical_size.x, logical_size.y, 0.0, 
//                     0.0, logical_size.x,  
//                     // 0.0, logical_size.y, //ydir
//                     logical_size.y,0.0, //ydir2

//                     0.0, UI_CAMERA_FAR
//                 );
//             let default_camera_view = commands
//                 .spawn(ExtractedView {
//                     // projection: projection_matrix,
//                     clip_from_view: projection_matrix,
//                     // transform: GlobalTransform::from_xyz(
//                     //     0.0,
//                     //     0.0,
//                     //     UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,
//                     // ),
//                     world_from_view: GlobalTransform::from_xyz(
//                         0.0,
//                         0.0,
//                         UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,
//                     ),
//                     clip_from_world: None,
//                     hdr: camera.hdr,
//                     viewport: UVec4::new(
//                         physical_origin.x,
//                         physical_origin.y,
//                         physical_size.x,
//                         physical_size.y,
//                     ),
//                     // view_projection: None,
//                     color_grading: Default::default(),
//                 })
//                 .id();
//             // commands.get_or_spawn(entity).insert((
//             //     MyDefaultCameraView(default_camera_view),
//             //     RenderPhase::<MyTransparentUi>::default(),
//             // ));
//             commands.get_or_spawn(entity).insert(MyDefaultCameraView(default_camera_view));
//         }
//     }
// }


// /// Configuration for cameras related to UI.
// ///
// /// When a [`Camera`] doesn't have the [`UiCameraConfig`] component,
// /// it will display the UI by default.
// ///
// /// [`Camera`]: bevy_render::camera::Camera

// #[derive(Component, Clone,ExtractComponent)]
// #[extract_component_filter(With<Camera>)]
// pub struct MyUiCameraConfig {
//     /// Whether to output UI to this camera view.
//     ///
//     /// When a `Camera` doesn't have the [`UiCameraConfig`] component,
//     /// it will display the UI by default.
//     pub show_ui: bool,
// }

// impl Default for MyUiCameraConfig {
//     fn default() -> Self {
//         Self { show_ui: true }
//     }
// }

// // impl ExtractComponent for MyUiCameraConfig {
// //     type Query = &'static Self;
// //     type Filter = With<Camera>;

// //     fn extract_component(item: QueryItem<Self::Query>) -> Self {
// //         item.clone()
// //     }
// // }

// #[derive(Component)]
// pub struct MyDefaultCameraView(pub Entity);
