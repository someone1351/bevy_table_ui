use bevy::{asset::Handle, color::Color, ecs::prelude::*, math::Vec2, prelude::Image, reflect::Reflect, text::Font};

use super::values::*;

#[derive(Reflect,Component, Debug, Clone,Copy)]
// pub struct UiColor(pub Color);

pub struct UiColor { 
    pub back : Color,
    pub padding : Color,
    pub border : Color, 
    pub margin : Color, 
    pub cell : Color, 
}

impl Default for UiColor {
    fn default() -> Self {
        Self { back: Color::NONE, padding: Color::NONE, border: Color::NONE, margin: Color::NONE, cell: Color::NONE }
    }
}


#[derive(Reflect,Component, Default, Debug, Clone)]
pub struct UiImage {
    pub handle : Handle<Image>,
    pub keep_aspect_ratio : bool,
    pub width_scale : f32,//Val,
    pub height_scale : f32,//Val,
    pub color : Color,
    //h/v repeat/clamp : float, 0 none,
    //h/v align
}

#[derive(Component, Clone, Default, Debug)]
pub struct UiTextComputed{
    pub max_size: Vec2,
}

// #[derive(Debug,  Clone, Default)]
// pub struct UiTextSection {
//     pub section : TextSection,
//     pub min_len : usize,
// }

#[derive(Component, Debug,  Clone, 
    // Default,
)]
pub struct UiText {
    // pub sections: Vec<UiTextSection>,
    // pub section : TextSection,

    pub value: String,
    pub font: Handle<Font>,
    pub font_size: f32,
    pub color: Color,
    
    pub hlen : u32,
    pub vlen : u32,
    
    // pub alignment: TextAlignment,
    pub halign : UiTextHAlign,
    pub valign : UiTextVAlign,
    
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
            value:String::new(),

            font: Default::default(),
            font_size: 12.0,
            color: Color::WHITE,

            hlen : 0,
            vlen : 0,
            
            halign : Default::default(),
            valign : Default::default(),
            
            update : false,
        }
    }
}
