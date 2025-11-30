
use std::collections::HashSet;

use bevy::ecs::prelude::*;
use super::super::layout::components::UiLayoutComputed;

// #[derive(Component,Default,Clone,Debug)]
// #[require(UiLayoutComputed)]
// pub struct UiHoverable {
//     pub enable : bool,
// }

// #[derive(Component,Default,Clone,Debug)]
// #[require(UiLayoutComputed)]
// pub struct UiPressable {
//     pub enable : bool, //disables everything
//     pub hoverable:bool,
//     pub draggable : bool,
//     pub pressable : bool,

//     pub press_onlys:HashSet<i32>, //[button] //todo use bit flags //only accept these buttons
//     pub drag_toggles:HashSet<i32>, //[button] //todo use bit flags

//     // pub draggable:HashMap<i32,bool>, //[button]=toggle
//     // pub pressable:HashSet<i32>, //[button] //todo use bit flags
//     // doesn't temporarily release if cursor is moved off while still pressing
//     // would want this to handle different for dif buttons eg lmb vs rmb?
//     // pub always : bool,

//     // pub physical : bool, //works like a real button, ie stays held down as long as something is pressing it

//     // pub pressed : bool, //set by both user and interact system
//     // pub released : bool, //set by both user and interact system
// }

// #[derive(Component,Default,Clone,Debug)]
// #[require(UiLayoutComputed)]
// pub struct UiDraggable {
//     pub enable : bool,
// }




#[derive(Component,Default,Clone,Debug)]
#[require(UiLayoutComputed)]
pub struct UiCursorable {
    pub hoverable : bool,
    pub draggable : bool,
    pub pressable : bool,
    // pub press_always : bool,
    pub press_onlys:HashSet<i32>, //[button] //todo use bit flags //only accept these buttons

}

// #[derive(Component,Default,Clone,Debug)]
// pub struct UiScrollable {
//     pub henable : bool,
//     pub venable : bool,
// }

//could add devices to selectable, so multiple devices can select different things ...
#[derive(Component,Default,Clone,Debug)]
#[require(UiLayoutComputed)]
pub struct UiSelectable {
    pub enable : bool,
    pub selected : bool, //set by both user and interact system
    pub group : String,
}

#[derive(Component,Clone,Debug,Default)]
pub struct UiFocusableComputed {
    row:Option<usize>,
    col:Option<usize>,
}

#[derive(Component,Clone,Debug,Default)]
#[require(UiLayoutComputed,UiFocusableComputed)]
pub struct UiFocusable {
    pub enable : bool,
    // pub focused : bool, //set by both user and interact system
    pub group : i32,

    // pub texit : bool,
    pub hexit : bool,
    pub vexit : bool,

    pub hwrap : bool,
    pub vwrap : bool,

    // pub hdir_press : bool, //when left/right on start/aready focus(ed) // ?
    // pub vdir_press : bool, //when up/down on start/aready focus(ed) // ?

    // pub move_press:bool, //on focused or focus move failed then press, works even if pressable is false, but if both enabled, both count has being held down
    // pub pressable:bool,
    pub pressable:HashSet<i32>, //[button] //todo use bit flags
    // pub left_pressable:bool,
    // pub right_pressable:bool,
    // pub up_pressable:bool,
    // pub down_pressable:bool,

    pub init:bool, // on focus_enter, focus on this

}

// impl Default for UiFocusable {
//     fn default() -> Self {
//         Self {
//             enable: false,
//             group: 0,
//             // texit: Default::default(),
//             hexit: false,
//             vexit: false,
//             // hwrap: true,
//             // vwrap: true,
//             hwrap: false,
//             vwrap: false,
//         }
//     }
// }
//for press, when first focusing on end node or already focused, it presses it
//what about tabbing?

// #[derive(Default,Clone,Debug)]
// pub enum UiFocusDirEnd {
//     #[default]
//     ExitWrap, //on sub focus, it exits, on root focus it wraps

//     NoWrap,
//     Wrap,
//     Exit,
// }
