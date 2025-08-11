
use std::collections::HashMap;
use std::collections::HashSet;

use bevy::ecs::prelude::*;
// use bevy::hierarchy::prelude::*;

use super::super::super::layout::components::{UiLayoutComputed,UiRoot};

use super::super::components::*;
// use super::super::resources::*;
use super::super::events::*;
// use super::super::utils::*;
// use super::super::values::*;


pub fn update_select_events(
    // mut active_nodes: ResMut<UiActiveNodes>,
    // root_query: Query<Entity,(Without<Parent>,With<UiLayoutComputed>)>,
    root_query: Query<(Entity,&UiLayoutComputed), With<UiRoot>>,
    parent_query : Query<&ChildOf,With<UiLayoutComputed>>,
    computed_query: Query<&UiLayoutComputed>, //,With<UiPressable>
    mut selectable_query: Query<(Entity,&mut UiSelectable)>,
    mut root_group_selecteds : Local<HashMap<Entity,HashMap<String,Entity>>>, //[root_entity][select_group]=node
    mut root_single_selecteds : Local<HashMap<Entity,HashSet<Entity>>>, //[root_entity][node]
    mut ui_event_writer: EventWriter<UiInteractEvent>,
) {

    //unselect removed entities, selecteds with changed group names, removed/disabled selectable
    for root_entity in root_group_selecteds.keys().cloned().collect::<Vec<_>>() {
        // let root_entity_alive=computed_query.get(root_entity).is_ok();
        let root_entity_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();

        //
        let group_selecteds=root_group_selecteds.get_mut(&root_entity).unwrap();

        for group in group_selecteds.keys().cloned().collect::<Vec<_>>() {
            let &entity = group_selecteds.get(&group).unwrap();

            let entity_alive=computed_query.get(entity).is_ok();

            let selectable = selectable_query.get(entity).map(|x|x.1).ok();
            let selectable_enable=selectable.map(|x|x.enable).unwrap_or_default();
            let selectable_group=selectable.map(|x|x.group.clone());
            let selected=selectable.map(|x|x.selected).unwrap_or_default();

            if !root_entity_alive || !entity_alive || !selectable_enable || selectable_group!=Some(group.clone())
                || !selected
                || selectable_group.map(|x|x.is_empty()).unwrap_or_default()
            {
                ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractEventType::SelectEnd});
                group_selecteds.remove(&group);
            }
        }

        //remove inactive roots
        // let no_groups= root_selecteds.get(&root_entity).map(|x|x.is_empty()).unwrap_or_default();

        if !root_entity_alive //&& !no_groups
        {
            root_group_selecteds.remove(&root_entity);
        }
    }

    //single selecteds
    for root_entity in root_single_selecteds.keys().cloned().collect::<Vec<_>>() {
        // let root_entity_alive=computed_query.get(root_entity).is_ok();
        let root_entity_alive= root_query.get(root_entity).map(|(_,computed)|computed.unlocked).unwrap_or_default();

        //
        let single_selecteds=root_single_selecteds.get_mut(&root_entity).unwrap();

        for entity in single_selecteds.clone() {

            let entity_alive=computed_query.get(entity).is_ok();

            let selectable = selectable_query.get(entity).map(|x|x.1).ok();
            let selectable_enable=selectable.map(|x|x.enable).unwrap_or_default();
            let selectable_group=selectable.map(|x|x.group.clone());
            let selected=selectable.map(|x|x.selected).unwrap_or_default();
            // println!("send {entity:?} {selectable_enable:?} {selectable_group:?}");

            if !root_entity_alive || !entity_alive || !selectable_enable
                || !selected
                || selectable_group.map(|x|!x.is_empty()).unwrap_or_default()
            {
                ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractEventType::SelectEnd});
                single_selecteds.remove(&entity);
            }
        }

        //remove inactive roots

        if !root_entity_alive //&& !no_groups
        {
            root_single_selecteds.remove(&root_entity);
        }
    }

    //
    let mut select_root_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    //get root entities with selectable descendants
    for (entity,selectable) in selectable_query.iter() {
        if !selectable.enable {
            continue;
        }

        let Ok(computed) = computed_query.get(entity) else {
            continue;
        };

        if !computed.unlocked { //necessary? should locked nodes be unable to be selected if locked?
            continue;
        }

        if let Some(root_entity)=parent_query.iter_ancestors(entity).last() {
            select_root_entities.entry(root_entity).or_default().push(entity);
        } else {
            select_root_entities.entry(entity).or_default().push(entity);
        }

    }

    //sort select_root_entities by computed.order
    for (_, entities) in select_root_entities.iter_mut() {
        entities.sort_by(|&a,&b|{
            let computed_a = computed_query.get(a).unwrap();
            let computed_b = computed_query.get(b).unwrap();
            computed_a.order.cmp(&computed_b.order) //.reverse()
        });
    }

    //select

    for (&root_entity, entities) in select_root_entities.iter() {
        let group_selecteds = root_group_selecteds.entry(root_entity).or_default();
        let single_selecteds = root_single_selecteds.entry(root_entity).or_default();

        for &entity in entities.iter() {
            let (_,cur_selectable)=selectable_query.get(entity).unwrap();

            if cur_selectable.selected {
                if cur_selectable.group.is_empty() { //single
                    if !single_selecteds.contains(&entity) {
                        ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractEventType::SelectBegin});
                        single_selecteds.insert(entity);
                    }
                } else { //group
                    if Some(entity)!=group_selecteds.get(&cur_selectable.group).cloned() {
                        if let Some(other_entity)=group_selecteds.get_mut(&cur_selectable.group) { //group has existing selected
                            let (_,mut other_selectable)=selectable_query.get_mut(*other_entity).unwrap();

                            other_selectable.selected=false;
                            ui_event_writer.write(UiInteractEvent{entity:*other_entity,event_type:UiInteractEventType::SelectEnd});

                            //
                            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractEventType::SelectBegin});
                            *other_entity=entity;
                        } else { //first time group has selected
                            ui_event_writer.write(UiInteractEvent{entity,event_type:UiInteractEventType::SelectBegin});
                            group_selecteds.insert(cur_selectable.group.clone(),entity);
                        }
                    }
                }
            }

        }
    }
}