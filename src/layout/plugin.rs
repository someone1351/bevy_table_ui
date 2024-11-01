
// use bevy::app::prelude::*;
use bevy::ecs::prelude::*; 

// use super::values::*;
use super::components::*;
use super::systems::*;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// #[derive(SystemSet)]
// pub enum MySystems {
//     SystemA,
//     SystemB,
//     SystemC,
// }

/*
TODO
* move to pre update
*/

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiLayoutSystem;

#[derive(Default)]
pub struct UiLayoutPlugin;

impl bevy::app::Plugin for UiLayoutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app

            .register_type::<UiEdge>() 
            .register_type::<UiGap>() 
            .register_type::<UiExpand>() 
            .register_type::<UiFill>() 
            .register_type::<UiScroll>() 
            .register_type::<UiFloat>() 
            .register_type::<UiHide>() 
            .register_type::<UiSpan>() 
            .register_type::<UiAlign>() 
            .register_type::<UiSize>() 
            .register_type::<UiInnerSize>() 
            .register_type::<UiLayoutComputed>() 
            
            .add_systems(bevy::app::PostUpdate, 
                (
                    ui_init_computeds,
                    ui_calc_computeds,
                ).chain().in_set(UiLayoutSystem) 
            )
            ;
        
    }
}
