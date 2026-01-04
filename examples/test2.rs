// mod my_text;

use std::collections::BTreeSet;

use bevy::{platform::collections::HashSet, prelude::*, text::TextLayoutInfo};

// use my_text::*;
use bevy_table_ui::{self as table_ui, CameraUi, UiText, UiAlign, UiColor, UiImage, UiRoot, UiSize, };

mod common;
mod affect;
use common::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // MyTextPlugin,
            table_ui::UiLayoutPlugin,
            table_ui::UiDisplayPlugin,
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


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //init camera
    commands.spawn(CameraUi);

    //init text
    commands.spawn((
        UiRoot::default(),
        // TextMarker,
        // MyText::new("aaa"),
        // UiText::default(),
        // TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 42.0, ..Default::default()},
        // TextColor(Color::linear_rgb(1.0, 0.2, 0.2)),
        // TextLayout{ justify: Justify::Center, ..Default::default() },
        // UiSize::scale(0.5, 0.5),
        UiAlign::top_left(),
        UiColor::default().back(Color::linear_rgb(0.0, 0.0, 0.5))
    )).with_child((
        TextMarker,
        UiText::new("aaa"),
        // UiText::default(),
        TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 42.0, ..Default::default()},
        TextColor(Color::linear_rgb(1.0, 0.2, 0.2)),
        UiSize::px(100.0, 400.0),
    ));

    // commands.spawn((
    //     Text::new("qqq"),
    //     TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 22.0, ..Default::default()},
    //     TextColor(Color::linear_rgb(0.2, 1.0, 0.2)),
    // ));

    //init texture atlas display
    commands.spawn((
        DisplayAtlasMarker,
        UiRoot::default(),
        UiColor::default().back(Color::linear_rgb(0.1, 0.5, 0.2)),
        // UiSize::scale(1.0, 1.0),

    ));
}





fn text_update_system(
    // mut text_marker_query: Query<&mut MyText, With<TextMarker>>,
    mut text_query: Query<&mut UiText, >,
    mut b:Local<usize>,
) {
    for mut text in text_query.iter_mut() {
        if *b==52 {
            text.0="aba".into();
            println!("done0");
        }
        // else if *b==1 {
        //     text.0="aca".into();
        //     println!("done1");
        // }

    }

    if *b<100 {
        *b+=1;
    }

    // if let Ok(mut text)=text_marker_query.single_mut() {
    //     if *text_changed==0 {
    //         text.0="aba".into();
    //         *text_changed+=1;
    //     }
    //     // else if *text_changed==1 {
    //     //     text.0="aca".into();
    //     //    *text_changed+=1;
    //     // }
    //     // else {
    //     //     text.0="aca".to_string();
    //     //     text.0.push_str("ac".repeat(*text_changed).as_str());
    //     //    *text_changed+=1;
    //     // }
    //     // println!("text is {:?}",text.0);
    // }
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
                    UiImage{
                        handle,
                        // width_scale: 0.25, height_scale: 0.25,
                        color: Color::linear_rgba(1.0,1.0,1.0,0.5),
                        // use_scaling: true,
                        transparency: true,
                        ..Default::default()
                    },
                    UiColor::default().back(Color::linear_rgba(0.4,0.4,0.4,0.5)),
                    // UiSize::px(100.0,100.0),
                    // Node {width:Val::Percent(25.0),height:Val::Percent(25.0),..Default::default()},
                    // ImageNode { image: handle, color: Color::linear_rgba(1.0,1.0,1.0,0.5), ..Default::default() },
                    // BackgroundColor(Color::linear_rgba(0.4,0.4,0.4,0.5))
                ));
            }
        });
    }
}


