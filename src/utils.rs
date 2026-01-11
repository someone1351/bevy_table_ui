use bevy::{math::{Rect, Vec2}};

use crate::UiRect;

pub fn rect_to_ui_rect(r:Rect)-> UiRect {
    UiRect { left: r.min.x, right: r.max.x, top: r.min.y, bottom: r.max.y }
}
pub fn ui_rect_to_rect(r:UiRect)-> Rect {
    Rect::new(r.left,r.top, r.right, r.bottom)
}
pub fn rect_right_top(r:Rect) -> Vec2 {
    Vec2::new(r.max.x, r.min.y)
}
pub fn rect_left_bottom(r:Rect) -> Vec2 {
    Vec2::new(r.min.x, r.max.y)
}

pub fn rect_is_zero(r:Rect) -> bool {
    r==Rect::default()
}

pub fn rect_expand(rect:Rect,other: Rect) -> Rect {
    Rect::from_corners(
        rect.min-other.min,
        rect.max+other.max,
    )
    // UiRect {
    //     left : self.left - other.left,
    //     top : self.top - other.top, //y is down
    //     right :self.right + other.right,
    //     bottom : self.bottom + other.bottom, //y is down
    // }
}


pub fn rect_clamp(rect:Rect,other:Rect) -> Rect {
    Rect::from_corners(
        rect.min.clamp(other.min, other.max),
        rect.max.clamp(other.min, other.max),
    )
    // UiRect {
    //     left:self.left.clamp(other.left, other.right),
    //     top:self.top.clamp(other.top, other.bottom), //y is down
    //     right:self.right.clamp(other.left, other.right),
    //     bottom:self.bottom.clamp(other.top, other.bottom), //y is down
    // }
}

// pub fn rect_sum<R:IntoIterator<Item = Rect>>(rects:R) -> Rect {
//     let mut out=Rect::default();

//     for rect in rects {
//         out.min+=rect.min;
//         out.max+=rect.max;
//     }
//     out
// }