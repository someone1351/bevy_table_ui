
use bevy::ecs::component::Component;
use bevy::prelude::{Entity, Vec2};
use bevy::reflect::Reflect;

use super::values::*;

// #[derive(Component, Default, Debug, Clone, Copy)]
// pub struct UiWrap {
//     pub wrap : bool,
// }

// #[derive(Component, Default, Debug, Clone, Copy)]
// pub struct UiFlow {
//     pub flow : FlowVal,
// }

// #[derive(Reflect,Component, Default, Debug, Copy, Clone)]
// pub struct UiChange {
//     pub pos : bool,
//     pub size : bool,
// }


//should it just resize the cell size instead ?
#[derive(Reflect,Component, Default, Debug, Copy, Clone)]
#[require(UiLayoutComputed)]
pub struct UiCongruent { //congruence, congruent, congruous, consonant, coincide,  adjacent_size
    pub row_width_scale : f32,
    pub col_height_scale : f32,
}

/*
shared both width and height
width sharing horizontal, where percent adding up less than eq 100% means the width of each is to make up those percents
but if over 100%, can normalise, or come up with size that is 100%, and make each of them the percent of that? no?
*/
#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiEdge { //neg scale used for using size w/h to calc edge h/w size aka (transverse)
    pub padding : UiRectVal,
    pub border : UiRectVal,
    pub margin : UiRectVal,
}

impl UiEdge {
    pub fn hvals(&self) -> [UiVal;6] {
        [self.padding.left,self.padding.right,self.border.left,self.border.right,self.margin.left,self.margin.right]
    }
    pub fn vvals(&self) -> [UiVal;6] {
        [self.padding.top,self.padding.bottom,self.border.top,self.border.bottom,self.margin.top,self.margin.bottom]
    }

    pub fn h_px(&self) -> f32 {
        self.hvals().iter().map(|&x|if let UiVal::Px(y)=x{y.max(0.0)}else{0.0}).sum()
    }
    pub fn v_px(&self) -> f32 {
        self.vvals().iter().map(|&x|if let UiVal::Px(y)=x{y.max(0.0)}else{0.0}).sum()
    }
    pub fn h_scale(&self) -> f32 {
        self.hvals().iter().map(|&x|if let UiVal::Scale(y)=x{y.max(0.0)}else{0.0}).sum()
    }
    pub fn v_scale(&self) -> f32 {
        self.vvals().iter().map(|&x|if let UiVal::Scale(y)=x{y.max(0.0)}else{0.0}).sum()
    }
    pub fn h_transverse_scale(&self) -> f32 {
        self.hvals().iter().map(|&x|if let UiVal::Scale(y)=x{y.min(0.0).abs()}else{0.0}).sum()
    }
    pub fn v_transverse_scale(&self) -> f32 {
        self.vvals().iter().map(|&x|if let UiVal::Scale(y)=x{y.min(0.0).abs()}else{0.0}).sum()
    }
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiGap {
    pub hgap : UiVal,
    pub vgap : UiVal,
}

impl UiGap {
    pub fn px(hgap:f32,vgap:f32) -> Self{
        Self { hgap:UiVal::Px(hgap), vgap:UiVal::Px(vgap) }
    }
}
#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiExpand {
    pub hexpand : UiVal,
    pub vexpand : UiVal,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiFill {
    pub hfill : UiVal,
    pub vfill : UiVal,
}

impl UiFill {
    pub fn max() -> Self {
        Self { hfill: UiVal::Scale(1.0), vfill: UiVal::Scale(1.0) }
    }
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiScroll {
    pub hscroll : UiVal,
    pub vscroll : UiVal,
}

#[derive(Reflect,Component, Default, Debug, Clone,Copy)]
#[require(UiLayoutComputed)]
pub struct UiFloat {
    pub float : bool,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiDisable {
    pub disable : bool,
}


#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiHide {
    pub hide : bool,
}


#[derive(Reflect,Component, Default, Debug, Clone,Copy)]
#[require(UiLayoutComputed)]
pub struct UiLock {
    pub lock : bool,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiSpan {
    pub span : u32,
}

impl UiSpan {
    pub fn new(span:u32) -> Self {
        Self{span}
    }
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
#[require(UiLayoutComputed)]
pub struct UiAlign {
    pub halign : UiVal,
    pub valign : UiVal,
}

impl UiAlign {

    pub fn scale(halign:f32,valign:f32) -> Self {
        Self { halign: UiVal::Scale(halign), valign: UiVal::Scale(valign) }
    }
    pub fn px(halign:f32,valign:f32) -> Self {
        Self { halign: UiVal::Px(halign), valign: UiVal::Px(valign) }
    }
    pub fn top_left() -> Self {
        Self { halign: UiVal::Scale(0.0), valign: UiVal::Scale(0.0) }
    }
    pub fn top_right() -> Self {
        Self { halign: UiVal::Scale(1.0), valign: UiVal::Scale(0.0) }
    }
    pub fn bottom_left() -> Self {
        Self { halign: UiVal::Scale(0.0), valign: UiVal::Scale(1.0) }
    }
    pub fn bottom_right() -> Self {
        Self { halign: UiVal::Scale(1.0), valign: UiVal::Scale(1.0) }
    }
    pub fn left() -> Self {
        Self { halign: UiVal::Scale(0.0), valign: UiVal::None }
    }
    pub fn right() -> Self {
        Self { halign: UiVal::Scale(1.0), valign: UiVal::None }
    }
    pub fn top() -> Self {
        Self { halign: UiVal::None, valign: UiVal::Scale(0.0) }
    }
    pub fn bottom() -> Self {
        Self { halign: UiVal::None, valign: UiVal::Scale(1.0) }
    }
    pub fn center() -> Self {
        Self::default()
    }
}

// #[derive(Reflect,Component, Default, Debug, Copy, Clone)]
// pub struct UiAspect {
//     pub aspect : UiAspectType,
// }

#[derive(Reflect,Component, Default, Debug, Copy, Clone)]
#[require(UiLayoutComputed)]
pub struct UiSize {
    pub width : UiVal,
    pub height : UiVal,

    // pub inner_width : f32, //px
    // pub inner_height : f32, //px
}

impl UiSize {
    pub fn scale(w:f32,h:f32) -> UiSize{
        UiSize { width: UiVal::Scale(w), height: UiVal::Scale(h) }
    }
    pub fn px(w:f32,h:f32) -> UiSize{
        UiSize { width: UiVal::Px(w), height: UiVal::Px(h) }
    }
    pub fn max() -> UiSize{
        Self::scale(1.0,1.0)
    }
}

// #[derive(Reflect,Component, Default, Debug, Copy, Clone)]
// #[require(UiLayoutComputed)]
// pub struct UiInnerSize { //used for external things like text and images
//     pub width : f32,
//     pub height : f32,
// }

//should UiRoot be empty and camera provide its fields?

#[derive(Reflect,Component, Debug, Copy, Clone)]
#[require(UiLayoutComputed)]
pub struct UiRoot {
    pub x:f32,
    pub y:f32,
    pub width:f32,
    pub height:f32,
    pub scaling:f32,
    pub text_scaling:f32,
    pub order:i32,
}

impl Default for UiRoot {
    fn default() -> Self {
        Self {
            x: 0.0, y: 0.0, width: 0.0, height: 0.0, scaling: 1.0, text_scaling:1.0, order: 0,
        }
    }
}

//have bool for calcd ?
//todo swap ui_rect with rect
#[derive(Reflect,Component, Debug, Copy, Clone)]

pub struct UiLayoutComputed {

    pub pos : Vec2,
    pub size : Vec2,
    pub custom_size : Vec2,
    //last_size?
    pub clamped_rect : UiRect,
    pub clamped_cell_rect : UiRect,

    pub border_size : UiRect,
    pub padding_size : UiRect,
    pub margin_size : UiRect,
    pub cell_size : UiRect,

    pub gap_size:Vec2,
    pub scroll_pos:Vec2,
    pub scroll_size:Vec2,
    pub children_size:Vec2,

    pub depth : u32, pub order : u32,
    pub row:u32, pub col:u32,
    pub rows:u32, pub cols:u32,

    pub visible : bool, //replace with hidden instead, so default can be false instead of true?
    pub enabled : bool,
    pub unlocked : bool,

    pub root_entity : Entity,
    pub camera_entity : Entity,
}

impl UiLayoutComputed {
    pub fn inner_rect(&self) -> UiRect {
        UiRect {
            left: self.pos.x,
            top: self.pos.y,
            right: self.pos.x+self.size.x,
            bottom: self.pos.y+self.size.y,
        }
    }
    pub fn padding_rect(&self) -> UiRect {
        self.inner_rect().expand_by(self.padding_size)
    }
    pub fn border_rect(&self) -> UiRect {
        self.padding_rect().expand_by(self.border_size)
    }
    pub fn margin_rect(&self) -> UiRect {
        self.border_rect().expand_by(self.margin_size)
    }
    pub fn cell_rect(&self) -> UiRect {
        self.margin_rect().expand_by(self.cell_size)
    }
    pub fn clamped_padding_rect(&self) -> UiRect {
        self.padding_rect().clamp(self.clamped_cell_rect)
    }
    pub fn clamped_border_rect(&self) -> UiRect {
        self.border_rect().clamp(self.clamped_cell_rect)
    }
    pub fn clamped_margin_rect(&self) -> UiRect {
        self.margin_rect().clamp(self.clamped_cell_rect)
    }
}
impl Default for UiLayoutComputed {
    fn default() -> Self {
        Self {
            // x: 0.0, y: 0.0, w: 0.0, h: 0.0,
            pos:Vec2::ZERO,
            size:Vec2::NEG_ONE,
            custom_size:Vec2::ZERO,

            clamped_rect:UiRect::default(),
            clamped_cell_rect:UiRect::default(),

            border_size:UiRect::default(),
            padding_size:UiRect::default(),
            margin_size:UiRect::default(),
            cell_size:UiRect::default(),

            // gap_w:0.0,gap_h:0.0,
            // scroll_x:0.0,scroll_y:0.0,
            // children_w:0.0,children_h:0.0,

            gap_size:Vec2::ZERO,
            scroll_pos:Vec2::ZERO,
            scroll_size:Vec2::ZERO,
            children_size:Vec2::ZERO,

            depth: 0, order: 0,
            row: 0, col: 0,
            rows: 0, cols: 0,
            visible: false,
            enabled: false,
            unlocked: false,
            root_entity: Entity::PLACEHOLDER,
            camera_entity: Entity::PLACEHOLDER,
        }
    }
}