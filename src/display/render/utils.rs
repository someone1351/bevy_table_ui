
// fn intersect_rects(r1 : Rect, r2 : Rect) -> bool {
//     !(r2.left > r1.right || r2.right < r1.left || r2.top < r1.bottom || r2.bottom > r1.top)
//     !(r2.min.x > r1.max.x || r2.max.x < r1.min.x || r2.max.y < r1.min.y || r2.min.y > r1.max.y)
// }

// use crate::table_ui::{UiVal, UiRect};

// use super::resources::{MyUiExtractedElements, MyUiExtractedElement};

// use bevy::prelude::Vec2;
// use bevy::render::color::Color;

// use bevy::asset::*;
// use bevy::render::texture::*;

// pub fn intersect_rects(ax1:f32,ay1:f32,ax2:f32,ay2:f32,bx1:f32,by1:f32,bx2:f32,by2:f32) -> bool {
//     !(bx1 > ax2 || bx2 < ax1 || by2 < ay1 || by1 > ay2)
// }
