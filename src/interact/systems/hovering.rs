
use std::collections::HashMap;

use bevy::ecs::prelude::*;
use bevy::hierarchy::prelude::*;
use bevy::math::Vec2;


use super::super::super::layout::components::UiLayoutComputed;

use super::super::components::*;
use super::super::resources::*;
use super::super::events::*;
// use super::super::utils::*;
// use super::super::values::*;



    // // active_nodes: Res<UiActiveNodes>,
    // root_query: Query<Entity,(Without<Parent>,With<UiLayoutComputed>)>,
    // // computed_query: Query<&UiComputed,With<UiHoverable>>,
    
    // computed_query: Query<&UiLayoutComputed>,
    // hoverable_query: Query<(Entity,&UiHoverable)>,
    // mut cur_hover_entities : Local<HashMap<Entity,Entity>>, //[root_entity]=cur_hover_entity



pub fn update_hover_events(

    parent_query : Query<&Parent,With<UiLayoutComputed>>,
    hoverable_query: Query<(Entity,&UiLayoutComputed,&UiHoverable)>,
    
    mut input_event_reader: EventReader<UiInteractInputEvent>,
    mut ui_event_writer: EventWriter<UiInteractEvent>,

    mut cur_hover_entities : Local<HashMap<(Entity,i32),(Entity,Vec2)>>, //[(root_entity,device)]=cur_hover_entity
) {

    //un hover inactive/disabled/invisible, and cursor no longer inside due to node pos/size change
    //remove inactive root nodes

    cur_hover_entities.retain(|(root_entity,device),(entity,cursor)|{
        if let Ok((_,layout_computed,hoverable)) = hoverable_query.get(*entity) {
            if hoverable.enable && layout_computed.unlocked && layout_computed.clamped_border_rect().contains_point(*cursor) {
                return true;
            }
        }

        ui_event_writer.send(UiInteractEvent{entity:*entity,event_type:UiInteractEventType::HoverEnd{device:*device}}); //what if entity removed? ok to return a dead one?
        false
    });
    
    //
    let mut hover_root_entities: HashMap<Entity, Vec<_>> = HashMap::new(); //[root]=entities

    //
    for (entity,layout_computed,hoverable) in hoverable_query.iter() {
        if !hoverable.enable {
            continue;
        }

        if !layout_computed.unlocked {
            continue;
        }
        

        let root_entity=parent_query.iter_ancestors(entity).last().unwrap_or(entity);
        
        hover_root_entities.entry(root_entity).or_default().push((entity,layout_computed.order,layout_computed.clamped_border_rect()));
    }
    
    //sort hover_root_entities by computed.order
    for (&root_entity, entities) in hover_root_entities.iter_mut() {
        entities.sort_by_key(|x|x.1);
    }

    //
    for ev in input_event_reader.read() {
        let &UiInteractInputEvent::CursorMoveTo{root_entity,device,cursor:Some(cursor)} = ev else {
            continue;
        };

        let Some(entities)=hover_root_entities.get(&root_entity) else {
            continue;
        };

        let cur_hover_entity=cur_hover_entities.get(&(root_entity,device)).cloned().map(|x|x.0);


        for &(entity,_,rect) in entities.iter() {
            let is_inside = !rect.is_zero() && rect.contains_point(cursor);
        
            if is_inside {
                if cur_hover_entity != Some(entity) {
                    if let Some(other_entity) = cur_hover_entity {
                        ui_event_writer.send(UiInteractEvent{entity:other_entity,event_type:UiInteractEventType::HoverEnd{device}});
                    }

                    ui_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::HoverBegin{device}});
                    cur_hover_entities.insert((root_entity,device), (entity,cursor));
                }
                
                break;
            } else if cur_hover_entity == Some(entity) { //not inside
                ui_event_writer.send(UiInteractEvent{entity,event_type:UiInteractEventType::HoverEnd{device}});
                cur_hover_entities.remove(&(root_entity,device)).unwrap();
            }
        }    
    }


}
