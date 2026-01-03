use bevy::{asset::Handle, color::Color, ecs::prelude::*, math::Vec2, prelude::Image, reflect::Reflect, text::{ComputedTextBlock, Font, TextLayoutInfo}};


use crate::UiVal;

use super::super::layout::components::{UiInnerSize,UiLayoutComputed};
// use bevy::prelude::*;
use super::values::*;

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
#[require(UiInnerSize,UiLayoutComputed)]


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

#[derive(Component, Clone, Default, Debug)]
#[require(UiLayoutComputed)]
pub struct UiTextComputed{
    pub max_size: Vec2, //what is this for?

    //TextBounds{ width: todo!(), height: todo!() },
    pub bounds: Vec2, //box size that text sits in including empty space, unlike text_layout.logical_size which is only for text itself

    //TextLayoutInfo{ scale_factor: todo!(), glyphs: todo!(), section_rects: todo!(), size: todo!() },
    pub scaling:f32,


    //prob don't need
    //from ui_size, to check if they have changed
    pub width_used:UiVal,
    pub height_used:UiVal,
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

#[require(UiLayoutComputed,UiInnerSize,UiTextComputed,TextLayoutInfo,ComputedTextBlock)]
pub struct MyText(pub String);

pub type MyTextReader<'w, 's> = bevy::text::TextReader<'w, 's, MyText>;
// pub type TextMyWriter<'w, 's> = bevy::text::TextWriter<'w, 's, MyText>;

impl MyText {
    /// Makes a new 2d text component.
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

impl bevy::text::TextRoot for MyText {}

impl bevy::text::TextSpanAccess for MyText {
    fn read_span(&self) -> &str {
        self.as_str()
    }
    fn write_span(&mut self) -> &mut String {
        &mut *self
    }
}

impl From<&str> for MyText {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for MyText {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug,  Clone,  )]
#[require(UiLayoutComputed,UiInnerSize,UiTextComputed,TextLayoutInfo,ComputedTextBlock)]

pub struct UiText {
    // text:String,
    // pre:String,
    // pub sections: Vec<UiTextSection>,
    // pub section : TextSection,

    //TextFont
    // pub font: Handle<Font>,
    // pub font_size: f32,

    //TextLayout
    // pub halign : UiTextHAlign,

    //TextColor
    // pub color: Color,

    //TextSpan
    // pub value: String,


    pub hlen : u32, //calcs boundary
    pub vlen : u32, //calcs boundary
    pub valign : UiTextVAlign, //only used in display code

    //


    // pub alignment: TextAlignment,

    pub update : bool,
}

impl Default for UiText {
    fn default() -> Self {
        // let sections = vec![TextSection {
        //     value : String::new(),
        //     style : TextStyle {
        //         font : Font::,
        //         font_size:16.0,
        //         color : Color::GRAY,
        //     }
        // }];

        Self {
            // sections : vec![Default::default()],
            // value:String::new(),

            // font: Default::default(),
            // font_size: 12.0,
            // color: Color::WHITE,

            hlen : 0,
            vlen : 0,

            // halign : Default::default(),
            valign : Default::default(),

            update : true,
        }
    }
}
