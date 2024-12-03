use bevy::{ecs::system::SystemParam, prelude::*, render::camera::RenderTarget, window::{PrimaryWindow, WindowRef}};

// use super::components::*;


// #[derive(SystemParam)]
// pub struct MyDefaultUiCamera<'w, 's> {
//     cameras: Query<'w, 's, (Entity, &'static Camera)>,
//     default_cameras: Query<'w, 's, Entity, (With<Camera>, With<IsMyDefaultUiCamera>)>,
//     primary_window: Query<'w, 's, Entity, With<PrimaryWindow>>,
// }

// impl<'w, 's> MyDefaultUiCamera<'w, 's> {
//     pub fn get(&self) -> Option<Entity> {
//         self.default_cameras.get_single().ok().or_else(|| {
//             // If there isn't a single camera and the query isn't empty, there is two or more cameras queried.
//             if !self.default_cameras.is_empty() {
//                 warn_once!("Two or more Entities with IsDefaultUiCamera found when only one Camera with this marker is allowed.");
//             }
//             self.cameras
//                 .iter()
//                 .filter(|(_, c)| match c.target {
//                     RenderTarget::Window(WindowRef::Primary) => true,
//                     RenderTarget::Window(WindowRef::Entity(w)) => self.primary_window.get(w).is_ok(),
//                     _ => false,
//                 })
//                 .max_by_key(|(e, c)| (c.order, *e))
//                 .map(|(e, _)| e)
//         })
//     }
// }



#[derive(Component)]
pub struct MyIsDefaultUiCamera;

#[derive(SystemParam)]
pub struct MyDefaultUiCamera<'w, 's> {
    cameras: Query<'w, 's, (Entity, &'static Camera)>,
    default_cameras: Query<'w, 's, Entity, (With<Camera>, With<MyIsDefaultUiCamera>)>,
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
                    RenderTarget::Window(WindowRef::Entity(w)) => self.primary_window.get(w).is_ok(),
                    _ => false,
                })
                .max_by_key(|(e, c)| (c.order, *e))
                .map(|(e, _)| e)
        })
    }
}




