
use bevy::ecs::component::Component;
use bevy::math::Rect;
use bevy::prelude::{Entity, Vec2};
use bevy::reflect::Reflect;

use crate::utils::{ui_rect_clamp, ui_rect_expand};

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
#[derive(Reflect,Component, Debug, Copy, Clone, )]

pub struct UiLayoutComputed {

    pub pos : Vec2,
    pub size : Vec2,
    pub custom_size : Vec2,
    //last_size?
    pub clamped_rect : Rect,
    pub clamped_cell_rect : Rect,

    pub border_edge : Rect,
    pub padding_edge : Rect,
    pub margin_edge : Rect,
    pub cell_edge : Rect,

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
    // pub camera_entity : Entity,

    pub changed:bool,
    pub pos_changed:bool,
    pub size_changed:bool,
    pub scroll_changed:bool,
}


impl UiLayoutComputed {

    pub fn size_ne(&self,other:&Self) -> bool {
        self.size != other.size
            || self.clamped_rect != other.clamped_rect
            || self.clamped_cell_rect != other.clamped_cell_rect
            || self.border_edge != other.border_edge
            || self.padding_edge != other.padding_edge
            || self.margin_edge != other.margin_edge
            || self.cell_edge != other.cell_edge
    }
    pub fn scroll_ne(&self,other:&Self) -> bool {
        self.scroll_pos != other.scroll_pos
            || self.scroll_size != other.scroll_size
    }
    pub fn pos_ne(&self,other:&Self) -> bool {
        self.pos != other.pos
    }

    pub fn rest_ne(&self,other:&Self) -> bool {
        self.row != other.row
        || self.col != other.col
        || self.rows != other.rows
        || self.cols != other.cols

        || self.visible != other.visible
        || self.enabled != other.enabled
        || self.unlocked != other.unlocked
        || self.root_entity != other.root_entity


        // && self.custom_size == other.custom_size //
        // && self.gap_size == other.gap_size //
        // && self.children_size == other.children_size
        // && self.depth == other.depth
        // && self.order == other.order

    }

    // pub fn init() -> Self {
    //     Self {
    //         size:Vec2::NEG_ONE,
    //         ..Default::default()
    //     }

    // }

    // fn zero() -> Self {
    //     Self {
    //         pos:Vec2::ZERO,
    //         size:Vec2::ZERO,
    //         custom_size:Vec2::ZERO,

    //         clamped_rect:Rect::default(),
    //         clamped_cell_rect:Rect::default(),

    //         border_size:Rect::default(),
    //         padding_size:Rect::default(),
    //         margin_size:Rect::default(),
    //         cell_size:Rect::default(),

    //         gap_size:Vec2::ZERO,
    //         scroll_pos:Vec2::ZERO,
    //         scroll_size:Vec2::ZERO,
    //         children_size:Vec2::ZERO,

    //         depth: 0, order: 0,
    //         row: 0, col: 0,
    //         rows: 0, cols: 0,
    //         visible: false,
    //         enabled: false,
    //         unlocked: false,
    //         root_entity: Entity::PLACEHOLDER,
    //         camera_entity: Entity::PLACEHOLDER,
    //     }
    // }

    pub fn inner_rect(&self) -> Rect {
        // Rect {
        //     left: self.pos.x,
        //     top: self.pos.y,
        //     right: self.pos.x+self.size.x,
        //     bottom: self.pos.y+self.size.y,
        // }
        Rect { min: self.pos, max: self.pos+self.size }
    }
    pub fn padding_rect(&self) -> Rect {
        // self.inner_rect().expand_by(self.padding_size)
        ui_rect_expand(self.inner_rect(),self.padding_edge)
    }
    pub fn border_rect(&self) -> Rect {
        // self.padding_rect().expand_by(self.border_size)
        ui_rect_expand(self.padding_rect(),self.border_edge)
    }
    pub fn margin_rect(&self) -> Rect {
        // self.border_rect().expand_by(self.margin_size)
        ui_rect_expand(self.border_rect(),self.margin_edge)
    }
    pub fn cell_rect(&self) -> Rect {
        // self.margin_rect().expand_by(self.cell_size)
        ui_rect_expand(self.margin_rect(),self.cell_edge)
    }
    pub fn outer_rect(&self) -> Rect {
        self.margin_rect()
    }


    pub fn clamped_padding_rect(&self) -> Rect {
        // self.padding_rect().clamp(self.clamped_cell_rect)

        let a=self.padding_rect();
        let b=self.clamped_cell_rect;

        ui_rect_clamp(a,b)
    }
    pub fn clamped_border_rect(&self) -> Rect {
        // self.border_rect().clamp(self.clamped_cell_rect)

        let a=self.border_rect();
        let b=self.clamped_cell_rect;
        ui_rect_clamp(a,b)

    }
    pub fn clamped_margin_rect(&self) -> Rect {
        // self.margin_rect().clamp(self.clamped_cell_rect)

        let a=self.margin_rect();
        let b=self.clamped_cell_rect;
        ui_rect_clamp(a,b)
    }
    pub fn clamped_outer_rect(&self) -> Rect {
        self.clamped_margin_rect()
    }
    pub fn outer_size(&self) -> Vec2 {
        self.margin_rect().size()
    }
    pub fn inner_size(&self) -> Vec2 {
        self.size
    }
    // pub fn padding_size(&self) -> Vec2 {
    //     self.padding_rect().size()
    // }

}
impl Default for UiLayoutComputed {
    fn default() -> Self {
        Self {
            // x: 0.0, y: 0.0, w: 0.0, h: 0.0,
            pos:Vec2::ZERO,
            // size:Vec2::NEG_ONE,
            size:Vec2::ZERO,
            custom_size:Vec2::ZERO,

            clamped_rect:Rect::default(),
            clamped_cell_rect:Rect::default(),

            border_edge:Rect::default(),
            padding_edge:Rect::default(),
            margin_edge:Rect::default(),
            cell_edge:Rect::default(),

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
            // camera_entity: Entity::PLACEHOLDER,

            changed:false,
            pos_changed: false,
            size_changed: false,
            scroll_changed: false,
        }
    }
}