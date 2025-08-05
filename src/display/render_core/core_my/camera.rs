// use crate::{
//     core_2d::graph::Core2d,
//     // tonemapping::{DebandDither, Tonemapping},
// };
use bevy::ecs::prelude::*;
use bevy::math::{Mat4, Rect, Vec2, Vec3A};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::camera::{ScalingMode, SubCameraView};
use bevy::render::{
    camera::{Camera, CameraProjection, CameraRenderGraph, Projection},
    extract_component::ExtractComponent,
};
use bevy::transform::prelude::Transform;

use super::graph::CoreMy;

const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;
const UI_CAMERA_FAR: f32 = 1000.0;
// GlobalTransform::from_xyz(0.0, 0.0, UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,)

/// A 2D camera component. Enables the 2D render graph for a [`Camera`].
#[derive(Component, Default, Reflect, Clone, ExtractComponent)]
#[extract_component_filter(With<Camera>)]
#[reflect(Component, Default, Clone)]
#[require(
    Camera,
    // // DebandDither,
    CameraRenderGraph::new(CoreMy),
    // Projection::Orthographic(OrthographicProjection::default_2d()),
    Projection::custom(MyOrthographicProjection::default()),
    // Frustum = OrthographicProjection::default_2d().compute_frustum(&GlobalTransform::from(Transform::default())),
    // // Tonemapping::None,

    Transform::from_xyz( 0.0, 0.0, UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET, ),
)]
pub struct CameraMy;
// struct Aa(Frustum);
//handled in bevy\crates\bevy_render\src\camera\camera.rs
// #[derive(Debug,Clone)]
// pub struct MyCustomProjection {
//     ortho: OrthographicProjection,

// }

// impl Default for MyCustomProjection {
//     fn default() -> Self {
//         // OrthographicProjection::
//         Self { ortho: OrthographicProjection {
//             near: 0.0,
//             far: 1000.0,
//             viewport_origin: Vec2::new(0.0, 0.0),
//             scaling_mode: bevy::render::camera::ScalingMode::WindowSize,
//             scale: 1.0,
//             area:Rect::default(),
//             // area: Rect { min: Vec2::new(x, y), max: Vec2::new(x, y) },
//             // ..Default::default()
//         } }
//     }
// }

// impl CameraProjection  for MyCustomProjection {
//     fn get_clip_from_view(&self) -> bevy::prelude::Mat4 {
//         self.ortho.get_clip_from_view()
//     }

//     fn get_clip_from_view_for_sub(&self, sub_view: &bevy::render::camera::SubCameraView) -> bevy::prelude::Mat4 {
//         self.ortho.get_clip_from_view_for_sub(sub_view)
//     }

//     fn update(&mut self, width: f32, height: f32) {
//         // self.ortho.update(width, height);
//         // self.ortho.area

//         self.ortho.area = Rect::new(
//             0.0,
//             height,
//             width,
//             0.0,
//         );

//     }

//     fn far(&self) -> f32 {
//         self.ortho.far()
//     }

//     fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [bevy::prelude::Vec3A; 8] {
//         self.ortho.get_frustum_corners(z_near, z_far)
//     }
// }



#[derive(Debug, Clone, Reflect)]
#[reflect(Debug, FromWorld, Clone)]
pub struct MyOrthographicProjection {
    pub near: f32,
    pub far: f32,
    pub viewport_origin: Vec2,
    pub scaling_mode: ScalingMode,
    pub scale: f32,
    pub area: Rect,
}
impl CameraProjection for MyOrthographicProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.area.min.x,
            self.area.max.x,
            // self.area.min.y,
            // self.area.max.y,
            self.area.max.y,
            self.area.min.y,
            self.far,
            self.near,
        )
    }
    fn get_clip_from_view_for_sub(&self, sub_view: &SubCameraView) -> Mat4 {
        let full_width = sub_view.full_size.x as f32;
        let full_height = sub_view.full_size.y as f32;
        let offset_x = sub_view.offset.x;
        let offset_y = sub_view.offset.y;
        let sub_width = sub_view.size.x as f32;
        let sub_height = sub_view.size.y as f32;
        let full_aspect = full_width / full_height;
        let top = self.area.max.y;
        let bottom = self.area.min.y;
        let ortho_height = top - bottom;
        let ortho_width = ortho_height * full_aspect;
        let center_x = (self.area.max.x + self.area.min.x) / 2.0;
        let left = center_x - ortho_width / 2.0;
        let right = center_x + ortho_width / 2.0;
        let scale_w = (right - left) / full_width;
        let scale_h = (top - bottom) / full_height;
        let left_prime = left + scale_w * offset_x;
        let right_prime = left_prime + scale_w * sub_width;
        let top_prime = top - scale_h * offset_y;
        let bottom_prime = top_prime - scale_h * sub_height;
        Mat4::orthographic_rh(
            left_prime,
            right_prime,
            // bottom_prime,
            // top_prime,
            top_prime,
            bottom_prime,
            // 0.0,
            // full_width,
            // full_height,
            // 0.0,
            self.far,
            self.near,
        )
    }
    fn update(&mut self, width: f32, height: f32) {
        let (projection_width, projection_height) = match self.scaling_mode {
            ScalingMode::WindowSize => (width, height),
            ScalingMode::AutoMin {
                min_width,
                min_height,
            } => {
                if width * min_height > min_width * height {
                    (width * min_height / height, min_height)
                } else {
                    (min_width, height * min_width / width)
                }
            }
            ScalingMode::AutoMax {
                max_width,
                max_height,
            } => {
                if width * max_height < max_width * height {
                    (width * max_height / height, max_height)
                } else {
                    (max_width, height * max_width / width)
                }
            }
            ScalingMode::FixedVertical { viewport_height } => {
                (width * viewport_height / height, viewport_height)
            }
            ScalingMode::FixedHorizontal { viewport_width } => {
                (viewport_width, height * viewport_width / width)
            }
            ScalingMode::Fixed { width, height } => (width, height),
        };
        let origin_x = projection_width * self.viewport_origin.x;
        let origin_y = projection_height * self.viewport_origin.y;
        self.area = Rect::new(
            self.scale * -origin_x,
            self.scale * -origin_y,
            self.scale * (projection_width - origin_x),
            self.scale * (projection_height - origin_y),
        );
    }
    fn far(&self) -> f32 {
        self.far
    }
    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        let area = self.area;
        [
            // Vec3A::new(area.max.x, area.min.y, z_near), // bottom right
            // Vec3A::new(area.max.x, area.max.y, z_near), // top right
            // Vec3A::new(area.min.x, area.max.y, z_near), // top left
            // Vec3A::new(area.min.x, area.min.y, z_near), // bottom left

            Vec3A::new(area.max.x, area.max.y, z_near), // top right
            Vec3A::new(area.max.x, area.min.y, z_near), // bottom right
            Vec3A::new(area.min.x, area.min.y, z_near), // bottom left
            Vec3A::new(area.min.x, area.max.y, z_near), // top left

            // Vec3A::new(area.max.x, area.min.y, z_far),  // bottom right
            // Vec3A::new(area.max.x, area.max.y, z_far),  // top right
            // Vec3A::new(area.min.x, area.max.y, z_far),  // top left
            // Vec3A::new(area.min.x, area.min.y, z_far),  // bottom left

            Vec3A::new(area.max.x, area.max.y, z_far),  // top right
            Vec3A::new(area.max.x, area.min.y, z_far),  // bottom right
            Vec3A::new(area.min.x, area.min.y, z_far),  // bottom left
            Vec3A::new(area.min.x, area.max.y, z_far),  // top left
        ]
    }
}
// impl FromWorld for MyOrthographicProjection {
//     fn from_world(_world: &mut World) -> Self {
//         MyOrthographicProjection::default_3d()
//     }
// }
impl MyOrthographicProjection {
    pub fn default_2d() -> Self {
        MyOrthographicProjection {
            scale: 1.0,
            far: UI_CAMERA_FAR,//1000.0,
            viewport_origin: Vec2::new(0.0, 0.0),
            scaling_mode: ScalingMode::WindowSize,
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),

            near: 0.0,
            // ..OrthographicProjection2::default_3d()
        }
    }
    // pub fn default_3d() -> Self {
    //     MyOrthographicProjection {
    //         scale: 1.0,
    //         near: 0.0,
    //         far: 1000.0,
    //         viewport_origin: Vec2::new(0.5, 0.5),
    //         scaling_mode: ScalingMode::WindowSize,
    //         area: Rect::new(-1.0, -1.0, 1.0, 1.0),
    //     }
    // }
}

impl Default for MyOrthographicProjection {
    fn default() -> Self {
        Self::default_2d()
    }
}