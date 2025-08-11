
use std::collections::{BTreeSet, HashMap};

use bevy::{color::Color, ecs::prelude::*, };

use super::super::layout::components::UiLayoutComputed;


#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone,PartialOrd, Ord)]
pub enum UiAffectState {
    // None,
    Select,
    Hover,
    Focus,
    Drag,
    Press,
}

// impl std::str::FromStr for UiAffectState {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "select" => Ok(Self::Select),
//             "hover" => Ok(Self::Hover),
//             "focus" => Ok(Self::Focus),
//             "drag" => Ok(Self::Drag),
//             "press" => Ok(Self::Press),
//             _ => Err(())
//         }
//     }
// }
// impl UiAffectState {
//     pub fn priority(&self) -> i32 {
//         match self {
//             Self::Select=>0,
//             Self::Hover=>1,
//             Self::Focus=>2,
//             Self::Drag=>3,
//             Self::Press=>4,
//         }
//     }
// }

#[derive(Component,Default,Debug)]
#[require(UiLayoutComputed)]
pub struct UiAffect {
    pub states : BTreeSet<UiAffectState>,
    pub remove_states : BTreeSet<UiAffectState>,
    pub back_color : HashMap<Option<UiAffectState>,Color>,
    pub border_color : HashMap<Option<UiAffectState>,Color>,
    pub text_color: HashMap<Option<UiAffectState>,Color>,

    pub padding_color : HashMap<Option<UiAffectState>,Color>,
    pub margin_color : HashMap<Option<UiAffectState>,Color>,
    pub cell_color : HashMap<Option<UiAffectState>,Color>,
}

// #[derive(Component,Default,Debug)]
// pub struct UiAffectColor {
//     pub back : HashMap<Option<UiAffectState>,Color>,
//     // pub padding : HashMap<Option<UiAffectState>,Color>,
//     pub border : HashMap<Option<UiAffectState>,Color>,
//     // pub margin : HashMap<Option<UiAffectState>,Color>,
//     // pub cell : HashMap<Option<UiAffectState>,Color>,
//     // // pub updated:bool,
// }

// #[derive(Component,Default,Debug)]
// pub struct UiAffectText {
//     pub font_size: HashMap<Option<UiAffectState>,f32>,
//     pub color: HashMap<Option<UiAffectState>,Color>,
//     // pub updated:bool,
// }

// // #[derive(Component,Default,Debug)]
// // pub struct UiAffectImage {
// //     pub color: HashMap<Option<UiAffectState>,Color>,
// //     // pub updated:bool,
// // }
