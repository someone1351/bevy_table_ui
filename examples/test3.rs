// mod my_text;

use std::collections::BTreeSet;

use bevy::{camera::{visibility::RenderLayers, Viewport}, platform::collections::HashSet, prelude::*, text::TextLayoutInfo};
// use bevy_table_ui::{MyText, UiDisplayPlugin, UiLayoutPlugin};

// use my_text::*;

mod affect;
mod common;
use common::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // MyTextPlugin,
            bevy_table_ui::UiLayoutPlugin,
            bevy_table_ui::UiDisplayPlugin,
        ))
        .add_systems(Startup, (setup,))
        .add_systems(Update, (

            update_ui_roots,
            text_update_system,
            atlas_display_system,
            atlas_display_system2,
        ).chain())
        .run();
}

#[derive(Component)]
struct TextMarker;

#[derive(Component)]
struct DisplayAtlasMarker2;

#[derive(Component)]
struct DisplayAtlasMarker;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //init camera
    //putting camera_ui second seems to overwrite screen?
    let clear_color=ClearColorConfig::Custom(Color::srgba(0.3, 0.3, 0.3,0.0));
    commands.spawn((
        Camera2d,
        Camera {
            clear_color,
            order: 0,
            ..Default::default()
        },
        RenderLayers::layer(0),
    ));

    commands.spawn((
        bevy_table_ui::CameraUi,
        Camera {
            clear_color,
            order: 1,
            viewport: Some(Viewport {
                // physical_position: UVec2::new(500, 0),

                physical_position: UVec2::new(300, 0),
                physical_size: UVec2::new(800, 700),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));
    //init text
    let root_entity= commands.spawn((
        bevy_table_ui::UiRoot::default(),
        bevy_table_ui::UiGap{hgap:bevy_table_ui::UiVal::Px(20.0),vgap:bevy_table_ui::UiVal::None},
        bevy_table_ui::UiSize::max(),
        RenderLayers::layer(1),
    )).id();
    // // commands.spawn((
    // commands.entity(root_entity).with_child((
    //     TextMarker,
    //     // bevy_table_ui::UiRoot::default(),
    //     bevy_table_ui::UiSize::px(100.0, 100.0),
    //     // Text::new("aaa"),
    //     bevy_table_ui::MyText::new("aaa"),
    //     bevy_table_ui::UiText::default(),
    //     TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 42.0, ..Default::default()},
    //     TextColor(Color::linear_rgb(1.0, 0.2, 0.2)),
    // ));


    commands.entity(root_entity).with_child((
        TextMarker,
        // bevy_table_ui::UiRoot::default(),
        // TextMarker,
        // Text::new("aaa"),
        bevy_table_ui::UiText::new("aaa"),
        TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 42.0, ..Default::default()},
        TextColor(Color::linear_rgb(1.0, 0.2, 0.2)),
        bevy_table_ui::UiSize::px(90.0, 55.0),
        bevy_table_ui::UiAlign::top(),
    ));

    // commands.spawn((
    //     Text::new("qqq"),
    //     TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 22.0, ..Default::default()},
    //     TextColor(Color::linear_rgb(0.2, 1.0, 0.2)),
    // ));

    //init texture atlas display
    commands.spawn((
        DisplayAtlasMarker2,
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Row, // Stack children vertically
            align_items: AlignItems::Center, //v
            justify_content: JustifyContent::Start, //h
            row_gap: Val::Px(12.0),
            column_gap: Val::Px(12.0),
            ..Default::default()
        },
        RenderLayers::layer(0),
    ));

    commands.spawn((
        DisplayAtlasMarker,
        bevy_table_ui::UiRoot::default(),
        bevy_table_ui::UiColor::default().back(Color::linear_rgb(0.1, 0.5, 0.2)),
        // UiSize::scale(1.0, 1.0),
        bevy_table_ui::UiAlign::right(),
        RenderLayers::layer(1),

    ));
}



fn text_update_system(
    // mut text_marker_query: Query<&mut MyText, With<TextMarkerMarker>>,
    mut text_marker_query: Query<Entity, With<TextMarker>>,
    mut text_changed:Local<usize>,
    mut commands: Commands,
) {
    if let Ok(entity)=text_marker_query.single() {
        if *text_changed==50 {
            commands.queue(move|world: &mut World|{
                world.entity_mut(entity).get_mut::<bevy_table_ui::UiText>().unwrap().0="aba".into();
            });
        }        if *text_changed==90 {
            commands.queue(move|world: &mut World|{
                world.entity_mut(entity).get_mut::<bevy_table_ui::UiText>().unwrap().0="aca".into();
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


fn atlas_display_system2(
    display_atlas_query: Query< Entity,With<DisplayAtlasMarker2>>,
    display_atlas_children_query: Query< &Children,With<DisplayAtlasMarker2>>,
    text_layout_info_query: Query<&TextLayoutInfo>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,

) {
    if let Ok(root_entity)=display_atlas_query.single() {
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
        commands.entity(root_entity).despawn_children();

        //
        commands.entity(root_entity).with_children(|parent|{
            for handle in handles {
                parent.spawn((
                    Node {width:Val::Percent(25.0),height:Val::Percent(25.0),..Default::default()},
                    ImageNode { image: handle, color: Color::linear_rgba(1.0,1.0,1.0,0.5), ..Default::default() },
                    BackgroundColor(Color::linear_rgba(0.4,0.4,0.4,0.5))
                ));
            }
        });
    }
}



fn atlas_display_system(
    display_atlas_query: Query< Entity,With<DisplayAtlasMarker>>,
    display_atlas_children_query: Query< &Children,With<DisplayAtlasMarker>>,
    text_layout_info_query: Query<&TextLayoutInfo>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,

) {
    if let Ok(root_entity)=display_atlas_query.single() {
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
        commands.entity(root_entity).despawn_children();

        //
        commands.entity(root_entity).with_children(|parent|{
            for handle in handles {
                parent.spawn((
                    bevy_table_ui::UiImage{
                        handle,
                        // width_scale: 0.25, height_scale: 0.25,
                        color: Color::linear_rgba(1.0,1.0,1.0,0.5),
                        // use_scaling: true,
                        transparency: true,
                        ..Default::default()
                    },
                    bevy_table_ui::UiColor::default().back(Color::linear_rgba(0.4,0.4,0.4,0.5)),
                    // UiSize::px(100.0,100.0),
                    // Node {width:Val::Percent(25.0),height:Val::Percent(25.0),..Default::default()},
                    // ImageNode { image: handle, color: Color::linear_rgba(1.0,1.0,1.0,0.5), ..Default::default() },
                    // BackgroundColor(Color::linear_rgba(0.4,0.4,0.4,0.5))
                ));
            }
        });
    }
}


