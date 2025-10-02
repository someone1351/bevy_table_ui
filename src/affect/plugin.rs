
use bevy::ecs::prelude::*;
// use bevy::app::PostUpdate;

use super::systems::*;
use super::super::interact;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiAffectSystem;

#[derive(Default)]
pub struct UiAffectPlugin;

impl bevy::app::Plugin for UiAffectPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            // .add_message::<UiInteractEvent>()
            // .init_resource::<UiFocusState>()
          
            .add_systems(bevy::app::PostUpdate, (
                (  
                    run_affect_states,
                    run_affect_vals,
                    // run_affect_text,
                ).chain().in_set(UiAffectSystem).after(interact::plugin::UiInteractSystem)
                
                ,
                
                // .before(super:)
                //update_text_image
            ))
            ;
        
    }
}
