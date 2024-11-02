
use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use bevy::reflect::Reflect;
// use bevy::render::color::Color;
// use bevy::render::texture::Image;
// use bevy::asset::Handle;
// use bevy::text::{TextSection, TextStyle, };

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
// pub struct UiRemove;

// #[derive(Reflect,Component, Default, Debug, Copy, Clone)]
// pub struct UiChange {
//     pub pos : bool,
//     pub size : bool,
// }


//should it just resize the cell size instead ? 
#[derive(Reflect,Component, Default, Debug, Copy, Clone)]
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
pub struct UiGap {
    pub hgap : UiVal,
    pub vgap : UiVal,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
pub struct UiExpand {
    pub hexpand : UiVal,
    pub vexpand : UiVal,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
pub struct UiFill {
    pub hfill : UiVal,
    pub vfill : UiVal,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
pub struct UiScroll {
    pub hscroll : UiVal,
    pub vscroll : UiVal,
}

#[derive(Reflect,Component, Default, Debug, Clone,Copy)]
pub struct UiFloat {
    pub float : bool,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
pub struct UiDisable {
    pub disable : bool,
}


#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
pub struct UiHide {
    pub hide : bool,
}


#[derive(Reflect,Component, Default, Debug, Clone,Copy)]
pub struct UiLock {
    pub lock : bool,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
pub struct UiSpan {
    pub span : u32,
}

#[derive(Reflect,Component, Default, Debug, Clone, Copy)]
pub struct UiAlign {
    pub halign : UiVal,
    pub valign : UiVal,
}


#[derive(Reflect,Component, Default, Debug, Copy, Clone)]
pub struct UiAspect {
    pub aspect : UiAspectType,
}

#[derive(Reflect,Component, Default, Debug, Copy, Clone)]
pub struct UiSize {
    pub width : UiVal,
    pub height : UiVal,
    
    // pub inner_width : f32, //px
    // pub inner_height : f32, //px
}

#[derive(Reflect,Component, Default, Debug, Copy, Clone)]
pub struct UiInnerSize { //used for external things like text and images
    pub width : f32,
    pub height : f32,
}


//have bool for calcd ?
#[derive(Reflect,Component, Debug, Copy, Clone)]
pub struct UiLayoutComputed {

    pub pos : Vec2,
    pub size : Vec2,
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

    //disabled, hidden,


        // pub x : f32, 
    // pub y : f32, 
    // pub w : f32, 
    // pub h : f32, 

    //add last_w, last_h?
    // pub gap_w:f32,
    // pub gap_h:f32,

    // pub scroll_x:f32,
    // pub scroll_y:f32,

    // pub children_w:f32,
    // pub children_h:f32,

}

impl UiLayoutComputed {

    // pub fn init(&mut self) {
    //     *self = Default::default();
    //     self.pos=Vec2::ZERO;
    //     self.size=Vec2::NEG_ONE;
    //     // self.x=-1.0;
    //     // self.y=-1.0;
    //     // self.w=-1.0;
    //     // self.h=-1.0;

    //     self.clamped_rect=UiRect::default();
    //     self.clamped_cell_rect=UiRect::default();

    //     self.border_size=UiRect::default();
    //     self.padding_size=UiRect::default();
    //     self.margin_size=UiRect::default();
    //     self.cell_size=UiRect::default();

    //     self.gap_size=Vec2::ZERO;
    //     self.scroll_pos=Vec2::ZERO;
    //     self.children_size=Vec2::ZERO;

    //     // self.gap_w=0.0;
    //     // self.gap_h=0.0;

    //     // self.scroll_x=0.0;
    //     // self.scroll_y=0.0;

    //     // self.children_w=0.0;
    //     // self.children_h=0.0;

    //     // self.depth=-1.0;
    //     self.depth=0;
    //     self.visible=false;
    // }
    
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
        }
    }
}