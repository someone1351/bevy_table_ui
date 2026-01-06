

use std::collections::BTreeSet;

use bevy::{prelude::*, text::TextLayoutInfo};


use bevy_table_ui::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiLayoutPlugin,
            UiDisplayPlugin,
        ))
        .add_systems(Startup, (setup,))
        .add_systems(Update, (

            update_ui_roots,
            text_update_system,
            atlas_display_system,
        ).chain())
        .run();
}

#[derive(Component)]
struct TextMarker;

#[derive(Component)]
struct DisplayAtlasMarker;


pub fn update_ui_roots(
    windows: Query<&Window>,
    mut root_query: Query<&mut UiRoot,>,
) {
    if let Ok(window)=windows.single() {
        for mut ui_root in root_query.iter_mut() {
            ui_root.width=window.width();
            ui_root.height=window.height();
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //init camera
    commands.spawn((
        CameraUi,
    ));

    //root
    let root_entity= commands.spawn((
        UiRoot::default(),
        UiGap::px(20.0,20.0),
        // UiSize::max(),
        // UiColor::default().back(Color::linear_rgb(0.2,0.4,0.4,)),
    )).id();

    //init text
    commands.entity(root_entity).with_child((
        TextMarker,
        UiText::new("aaa"),
        TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 42.0, ..Default::default()},
        TextColor(Color::linear_rgb(1.0, 0.2, 0.2)),
        UiSize::px(90.0, 55.0),
        UiColor::default().back(Color::linear_rgba(0.4,0.4,0.4,0.5)),
        UiFill::max(),
    ));

    //init texture atlas display
    commands.entity(root_entity).with_child((
        DisplayAtlasMarker,
        UiGap::px(20.0,20.0),
        UiColor::default().back(Color::linear_rgba(0.4,0.4,0.2,0.5)),
        UiFill::max(),
    ));
}



fn text_update_system(
    // mut text_marker_query: Query<&mut MyText, With<TextMarkerMarker>>,
    text_marker_query: Query<Entity, With<TextMarker>>,
    mut text_changed:Local<usize>,
    mut commands: Commands,
) {
    if let Ok(entity)=text_marker_query.single() {
        if *text_changed==50 {
            commands.queue(move|world: &mut World|{
                world.entity_mut(entity).get_mut::<UiText>().unwrap().0="aba".into();
            });
        }        if *text_changed==90 {
            commands.queue(move|world: &mut World|{
                world.entity_mut(entity).get_mut::<UiText>().unwrap().0="aca".into();
            });
        }
        if *text_changed<100 {
            *text_changed+=1;
        }
    }
    // // if let Ok(mut text)=text_marker_query.single_mut() {
    // //     if *text_changed==0 {
    // //         text.0="aba".into();
    // //         *text_changed+=1;
    // //     }
    // //     // else if *text_changed==1 {
    // //     //     text.0="aca".into();
    // //     //    *text_changed+=1;
    // //     // }
    // //     // else {
    // //     //     text.0="aca".to_string();
    // //     //     text.0.push_str("ac".repeat(*text_changed).as_str());
    // //     //    *text_changed+=1;
    // //     // }
    // //     // println!("text is {:?}",text.0);
    // // }
}



fn atlas_display_system(
    display_atlas_query: Query< Entity,With<DisplayAtlasMarker>>,
    // display_atlas_children_query: Query< &Children,With<DisplayAtlasMarker>>,
    text_layout_info_query: Query<&TextLayoutInfo>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,

) {
    if let Ok(entity)=display_atlas_query.single() {
        let mut handles=BTreeSet::new();

        //
        for text_layout_info in text_layout_info_query.iter() {
            for text_glyph in text_layout_info.glyphs.iter() {
                if let Some(handle) = images.get_strong_handle(text_glyph.atlas_info.texture.clone()) {
                    handles.insert(handle);
                }
            }
        }

        // println!("h {},c {}",handles.len(),display_atlas_children_query.single().map(|x|x.len()).unwrap_or_default());

        //
        commands.entity(entity).despawn_children();

        //
        commands.entity(entity).with_children(|parent|{
            for handle in handles {
                parent.spawn((
                    UiImage{
                        handle,
                        // width_scale: 0.25, height_scale: 0.25,
                        color: Color::linear_rgba(1.0,1.0,1.0,0.5),
                        // use_scaling: true,
                        transparency: true,
                        ..Default::default()
                    },
                    UiColor::default().back(Color::linear_rgb(0.4,0.4,0.2)),
                ));
            }
        });
    }
}


