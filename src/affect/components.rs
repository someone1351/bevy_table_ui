
use std::collections::{HashMap, HashSet};

use bevy::{ecs::prelude::*, window::Window};
use crate::UiRoot;

use super::values::*;

#[derive(Component,Default)]
pub struct UiAffectComputed {
    pub states : HashMap<UiAffectState,HashSet<DeviceType>>, //[state][device]
    pub cur_attrib_inds:HashMap<usize,usize>, //[attrib_ind]=cur_ind
}

#[derive(Component,Default)]
// #[require(UiLayoutComputed)]
#[require(UiAffectComputed)]
pub struct UiAffect(pub Vec<UiAffectAttrib>); //[attrib_ind]=
// pub struct UixAffect {
//     // pub attribs : HashMap<Option<UiAffectState>,Vec<(AttribFuncType,Option<i32>)>>, //[state][attrib_ind]=(func,priority)
//     // pub attribs2 : Vec<HashMap<Option<UiAffectState>,(AttribFuncType,Option<i32>)>>, //attrib_ind][state]=(func,priority)
//     // pub attribs : Vec<HashMap<Option<UiAffectState>,(AttribFuncType,i32)>>, //[attrib_ind][state]=(func,priority)
//     pub attribs : Vec<(AttribFuncType,HashMap<UiAffectState,(AttribFuncType,i32)>)>, //[attrib_ind](default_func,[state]=(func,priority))
//     // pub states : HashMap<Option<UiAffectState>,>,
//     // pub states : HashSet<UiAffectState>,
//     pub states : HashMap<UiAffectState,HashSet<DeviceType>>, //[state][device]

//     // pub remove_states : BTreeSet<UiAffectState>,
// }

pub fn ui_root_to_single_window<M:Component>(
    windows: Query<&Window>,
    mut root_query: Query<&mut UiRoot,With<M>>,
) {

    // let window=windows.single();

    // let window_size=window
    //     .and_then(|window|Ok((window.width(),window.height())))
    //     .unwrap_or_default();

    if let Ok(window)=windows.single() {
        let width=window.width();
        let height=window.height();
        let scaling=window.resolution.base_scale_factor();

        println!("Scale is {} {} {}",
            window.scale_factor(),
            window.resolution.scale_factor(),
            window.resolution.base_scale_factor()
        );

        for mut ui_root in root_query.iter_mut() {
            ui_root.width=width;
            ui_root.height=height;
            ui_root.scaling=scaling;
        }

    }
}
