use bevy::{asset::Handle, color::Color, ecs::prelude::*, math::Vec2, prelude::Image, reflect::Reflect, text::{ComputedTextBlock, FontHinting, LineHeight, TextLayoutInfo}};



// use super::super::layout::components::{UiInnerSize};
use super::super::layout::components::UiLayoutComputed;
// use super::super::layout::values::*;
// use bevy::prelude::*;
// use super::values::*;

#[derive(Reflect,Component, Debug, Clone,Copy)]
#[require(UiLayoutComputed)]
// pub struct UiColor(pub Color);

pub struct UiColor {
    pub back : Color,
    pub padding : Color,
    pub border : Color,
    pub margin : Color,
    pub cell : Color,
}

impl UiColor {
    pub fn back(&self,back:Color) -> Self {
        Self { back, ..*self }
    }
    pub fn padding(&self,padding:Color) -> Self {
        Self { padding, ..*self }
    }
    pub fn border(&self,border:Color) -> Self {
        Self { border, ..*self }
    }
    pub fn margin(&self,margin:Color) -> Self {
        Self { margin, ..*self }
    }
    pub fn cell(&self,cell:Color) -> Self {
        Self { cell, ..*self }
    }
}

impl Default for UiColor {
    fn default() -> Self {
        Self { back: Color::NONE, padding: Color::NONE, border: Color::NONE, margin: Color::NONE, cell: Color::NONE }
    }
}

// pub enum ImageAlpha {
//     Transparent,
//     AlphaTest(Color),
// }
#[derive(Reflect,Component,  Debug, Clone)]
#[require(UiLayoutComputed)]


pub struct UiImage {
    pub handle : Handle<Image>,
    // pub keep_aspect_ratio : bool,
    pub width_scale : f32,//Val,
    pub height_scale : f32,//Val,
    pub color : Color,
    //h/v repeat/clamp : float, 0 none,
    //h/v align
    //stretch h/v
    //flip h/v
    //rot 90, 180, 270
    pub use_scaling:bool,
    pub transparency : bool,
    pub alpha_test : Option<Color>,
}

impl Default for UiImage {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            width_scale: 1.0,
            height_scale: 1.0,
            color: Color::WHITE,
            use_scaling: true,
            transparency: false,
            alpha_test: None,
        }
    }
}


#[derive(Component,Reflect,Debug, Default, Clone,Copy,PartialEq,Eq)]
pub enum UiTextVAlign {
    #[default]
    Center,
    Top,
    Bottom,
}

#[derive(Component, Clone, Default, Debug)]
#[require(UiLayoutComputed)]
pub struct UiTextComputed{
    pub calc_size: Vec2, //box size that text sits in including empty space, unlike text_layout.logical_size which is only for text itself
    pub scaling:f32,
    // pub needs_update:bool,
    // pub bounds_none:(bool,bool),

    pub layout_computed_size : Vec2,
    // pub font_errs:bevy::platform::collections::HashSet<Handle<Font>>,
    pub err:bool,
    //pub text_bound:TextBound,


    //
    // pub max_size: Vec2, //what is this for?

    //TextBounds{ width: todo!(), height: todo!() },

    //TextLayoutInfo{ scale_factor: todo!(), glyphs: todo!(), section_rects: todo!(), size: todo!() },


    // //prob don't need
    // //from ui_size, to check if they have changed
    // pub width_used:UiVal,
    // pub height_used:UiVal,
}

// #[derive(Debug,  Clone, Default)]
// pub struct UiTextSection {
//     pub section : TextSection,
//     pub min_len : usize,
// }

// #[derive(Component,Reflect,Debug, Default, Clone,Copy,PartialEq,Eq)]
// pub enum UiTextVAlign2 {
//     #[default]
//     Center,
//     Top,
//     Bottom,
// }

// #[derive(Component, Debug,  Clone,  )]
// #[require(UiLayoutComputed,UiInnerSize,UiTextComputed,TextLayoutInfo,ComputedTextBlock)]
// pub struct UiText(String);

// #[derive(Component, Debug,  Clone,  )]
// #[require(UiLayoutComputed,UiInnerSize,UiTextComputed,TextLayoutInfo,ComputedTextBlock)]
// pub struct UiTextPre(String);

// #[derive(Component, Debug,  Clone,  )]
// pub struct UiTextVAlign2(UiTextVAlign);

#[derive(Component, Clone, Debug, Default, bevy::prelude::Deref, bevy::prelude::DerefMut, Reflect)]
#[reflect(Component,  Debug, Clone)]

#[require(UiLayoutComputed,UiTextComputed,TextLayoutInfo,ComputedTextBlock,FontHinting::Disabled,LineHeight)]
pub struct UiText(pub String);

pub type UiTextReader<'w, 's> = bevy::text::TextReader<'w, 's, UiText>;
// pub type TextMyWriter<'w, 's> = bevy::text::TextWriter<'w, 's, MyText>;

impl UiText {
    /// Makes a new 2d text component.
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

impl bevy::text::TextRoot for UiText {}

impl bevy::text::TextSpanAccess for UiText {
    fn read_span(&self) -> &str {
        self.as_str()
    }
    fn write_span(&mut self) -> &mut String {
        &mut *self
    }
}

impl From<&str> for UiText {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for UiText {
    fn from(value: String) -> Self {
        Self(value)
    }
}

