// use std::collections::HashSet;

use std::collections::{BTreeSet, HashMap};

use bevy::ecs::prelude::*;

use super::components::*;
use super::super::layout::components::UiLayoutComputed;
use super::super::display::components::{UiColor,UiText}; //,UiImage

use super::super::interact::messages::{UiInteractEvent,UiInteractMessageType};

/*
* what to do about an attribute set by multiple events?
* * have a priority?
* * average them together?
*/

pub fn run_affect_states(
    mut interact_event_reader: MessageReader<UiInteractEvent>,

    mut affect_computed_query: Query<&mut UiAffect,>,
    // mut color_query: Query<(&UiAffectColor, &mut UiColor,)>,
    // mut text_query: Query<(&mut UiText,&UiAffectText)>,
    // mut image_query: Query<(&mut UiImage,&UiAffectImage)>,
) {
    //if pressed and released in a single frame, need affect state to stay set for atleast one frame
    for mut affect_computed in affect_computed_query.iter_mut() {
        let removes=affect_computed.remove_states.clone();
        affect_computed.remove_states.clear();
        affect_computed.states.retain(|x|!removes.contains(x));
    }

    //
    let mut new_states=BTreeSet::<UiAffectState>::new();


    //
    for ev in interact_event_reader.read() {
        let Ok(mut affect_computed)=affect_computed_query.get_mut(ev.entity) else {
            continue;
        };

        match &ev.event_type {
            UiInteractMessageType::FocusBegin { .. } => {
                affect_computed.states.insert(UiAffectState::Focus);
                new_states.insert(UiAffectState::Focus);
            }
            UiInteractMessageType::FocusEnd { .. } => {
                if new_states.contains(&UiAffectState::Focus) {
                    affect_computed.remove_states.insert(UiAffectState::Focus);
                } else {
                    affect_computed.states.remove(&UiAffectState::Focus);
                }
            }
            // UiInteractEventType::DragBegin => {
            //     affect_computed.states.insert(UiAffectState::Drag);
            //     new_states.insert(UiAffectState::Drag);
            // }
            // UiInteractEventType::DragEnd => {
            //     if new_states.contains(&UiAffectState::Drag) {
            //         affect_computed.remove_states.insert(UiAffectState::Drag);
            //     } else {
            //         affect_computed.states.remove(&UiAffectState::Drag);
            //     }
            // }
            UiInteractMessageType::PressBegin => {
                affect_computed.states.insert(UiAffectState::Press);
                new_states.insert(UiAffectState::Press);
            }
            UiInteractMessageType::PressEnd => {
                if new_states.contains(&UiAffectState::Press) {
                    affect_computed.remove_states.insert(UiAffectState::Press);
                } else {
                    affect_computed.states.remove(&UiAffectState::Press);
                }
            }
            UiInteractMessageType::SelectBegin => {
                affect_computed.states.insert(UiAffectState::Select);
                new_states.insert(UiAffectState::Select);
            }
            UiInteractMessageType::SelectEnd => {
                if new_states.contains(&UiAffectState::Select) {
                    affect_computed.remove_states.insert(UiAffectState::Select);
                } else {
                    affect_computed.states.remove(&UiAffectState::Select);
                }
            }
            UiInteractMessageType::HoverBegin{..} => {
                affect_computed.states.insert(UiAffectState::Hover);
                new_states.insert(UiAffectState::Hover);
            }
            UiInteractMessageType::HoverEnd{..} => {
                if new_states.contains(&UiAffectState::Hover) {
                    affect_computed.remove_states.insert(UiAffectState::Hover);
                } else {
                    affect_computed.states.remove(&UiAffectState::Hover);
                }
            }
            UiInteractMessageType::Click => {}
            UiInteractMessageType::DragX{..} => {}
            UiInteractMessageType::DragY{..} => {}
            // UiInteractEventType::DragMove { .. } =>{}
            // _ =>{}
        }
    }

}


fn get_state_val<T:Default+Clone>(cur_states : &BTreeSet<UiAffectState>,state_vals:&HashMap<Option<UiAffectState>,T>) -> Option<T>{
    if !state_vals.is_empty() {
        let keys=state_vals.keys().filter_map(|&x|x).collect::<BTreeSet<_>>();
        let states=cur_states.intersection(&keys).map(|&x|x).collect::<Vec<_>>();

        Some(if states.is_empty() {
            state_vals.get(&None).cloned().unwrap_or_default()
        } else {
            state_vals.get(&Some(*states.last().unwrap())).cloned().unwrap()
        })
    } else {
        None
    }
}

pub fn run_affect_vals(
    // mut color_query: Query<(Entity,&UiAffectComputed,&mut UiAffectColor, &mut UiColor,)>,
    mut affect_query: Query<(Entity,&UiLayoutComputed,&UiAffect,)>, //&UiAffectColor, &mut UiColor,
    mut commands: Commands,
) {
    // for (entity,affect_computed,mut affect_color,mut color) in color_query.iter_mut()
    for (entity,layout_computed,affect,) in affect_query.iter_mut()  //affect_color,mut color
    {
        if !layout_computed.enabled||!layout_computed.visible {
            continue;
        }

        // if !affect_color.updated {
        //     continue;
        // }

        // affect_color.updated=false;

        //back color
        if let Some(x)=get_state_val(&affect.states,&affect.back_color) {
            commands.queue(move|world: &mut World| {
                let mut e=world.entity_mut(entity);
                let mut c=e.entry::<UiColor>().or_default().into_mut();
                c.back=x;
            });
        }

        //border color
        if let Some(x)=get_state_val(&affect.states,&affect.border_color) {
            commands.queue(move|world: &mut World| {
                let mut e=world.entity_mut(entity);
                let mut c=e.entry::<UiColor>().or_default().into_mut();
                c.border=x;
            });
        }

        //font color
        if let Some(x)=get_state_val(&affect.states,&affect.text_color) {
            //don't need to add other components needed for drawing text, since is only colour,
            //  which in itself not enough to draw text
            //  the necessary components for text will be added elsewhere
            // commands.add(move |world: &mut World| {
            //     if let Some(mut c)=world.entity_mut(entity).get_mut::<UiText>() {
            //         c.color=x;
            //     } else {
            //         world.entity_mut(entity).insert(UiText{color:x,..Default::default()});
            //     }
            // });

            commands.queue(move|world: &mut World| {
                let mut e=world.entity_mut(entity);
                let mut c=e.entry::<UiText>().or_default().into_mut();
                c.color=x;
            });
        }

        //padding color
        if let Some(x)=get_state_val(&affect.states,&affect.padding_color) {
            commands.queue(move|world: &mut World| {
                let mut e=world.entity_mut(entity);
                let mut c=e.entry::<UiColor>().or_default().into_mut();
                c.padding=x;
            });
        }

        //margin color
        if let Some(x)=get_state_val(&affect.states,&affect.margin_color) {
            commands.queue(move|world: &mut World| {
                let mut e=world.entity_mut(entity);
                let mut c=e.entry::<UiColor>().or_default().into_mut();
                c.margin=x;
            });
        }

        //cell color
        if let Some(x)=get_state_val(&affect.states,&affect.cell_color) {
            commands.queue(move|world: &mut World| {
                let mut e=world.entity_mut(entity);
                let mut c=e.entry::<UiColor>().or_default().into_mut();
                c.cell=x;
            });
        }
    }
}

