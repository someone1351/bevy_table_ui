
use bevy::ecs::prelude::*;
use super::super::layout::components::UiLayoutComputed;

#[derive(Component,Default,Clone,Debug)]
#[require(UiLayoutComputed)]
pub struct UiHoverable {
    pub enable : bool,
}

#[derive(Component,Default,Clone,Debug)]
#[require(UiLayoutComputed)]
pub struct UiPressable {
    pub enable : bool,
    pub always : bool, // doesn't temporarily release if cursor is moved off while still pressing
    pub physical : bool, //works like a real button, ie stays held down as long as something is pressing it

    // pub pressed : bool, //set by both user and interact system
    // pub released : bool, //set by both user and interact system
}

#[derive(Component,Default,Clone,Debug)]
#[require(UiLayoutComputed)]
pub struct UiDraggable {
    pub enable : bool,
}

// #[derive(Component,Default,Clone,Debug)]
// pub struct UiScrollable {
//     pub henable : bool,
//     pub venable : bool,
// }

#[derive(Component,Default,Clone,Debug)]
#[require(UiLayoutComputed)]
pub struct UiSelectable {
    pub enable : bool,
    pub selected : bool, //set by both user and interact system
    pub group : String,
}

#[derive(Component,Default,Clone,Debug)]
#[require(UiLayoutComputed)]
pub struct UiFocusable {
    pub enable : bool,
    pub focused : bool, //set by both user and interact system
    pub group : i32,

    pub tab_exit : bool,
    pub hdir_exit : bool,
    pub vdir_exit : bool,

    pub hdir_wrap : bool,
    pub vdir_wrap : bool,

    pub hdir_press : bool, //when left/right on start/aready focus(ed) // ?
    pub vdir_press : bool, //when up/down on start/aready focus(ed) // ?

}

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
