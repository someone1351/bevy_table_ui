
// #![allow(unused_mut)]
// #![allow(unused_variables)]
// #![allow(unreachable_code)]

// use std::sync::Arc;

use bevy::app::*;
use bevy::asset::prelude::*;
use bevy::color::Color;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
// use bevy::ecs::prelude::*;

use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};

use bevy::text::*;
// use bevy::ui::{AlignSelf, JustifySelf, Node};
use bevy::window::*;
use bevy::DefaultPlugins;
use bevy::prelude::{Msaa, PluginGroup };


use bevy_table_ui::CameraUi;
use bevy_table_ui as table_ui;
use rand::rngs::ThreadRng;
// use rand::Rng;
// use mesh::TestRenderComponent;
// use render_core::core_my::CameraMy;
use table_ui::*;


// #[path = "affect/mod.rs"]
// mod affect;
mod common;

use common::*;

fn main() {
    use bevy::ecs::prelude::*;

    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {watch_for_changes_override:Some(true), ..Default::default() })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "table ui".into(),
                        resolution: (800, 600).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            UiLayoutPlugin,
            UiInteractPlugin,
            UiDisplayPlugin,
            UiAffectPlugin,
        ))


        // .add_systems(Startup, ( setup_input, setup_camera, setup_menu, ))
        // .add_systems(PreUpdate, ( update_input, ))
        // .add_systems(Update, ( show_menu, ))



        .add_systems(Startup, (
            setup_fps,
            setup_test,
            setup_camera,
            setup_ui,
        ).chain())
        .add_systems(Update, (
            update_ui_roots,
            update_ui_input.before(UiInteractSystem),
            // update_ui,
            // on_affects,
            update_input,
            show_fps, //.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(100))),
            // on_affects2,
            // test_exit,
        ).chain())
        // .add_systems(Update, (
        // ))
        ;

    app.run();
}






pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut rng: ThreadRng = rand::thread_rng();

    let font: Handle<Font>=asset_server.load("fonts/FiraMono-Medium.ttf");

    let root_entity= commands.spawn((
        UiRoot::default(),
        UiGap{hgap:UiVal::Px(20.0),vgap:UiVal::None},
    )).id();


    // // let entity=commands.entity(root_entity).with_child((
    // //     UiSize::px(90.0, 55.0),
    // // )).id();
    // // create_ui_box(&mut commands, &mut rng, font.clone(),entity);

    // // commands.entity(root_entity).with_child((
    // //     TextFont{ font: font.clone(), font_size: 15.0, ..Default::default() },
    // //     TextColor(Color::linear_rgb(1.0,0.0,0.0)),
    // //     TextLayout{ justify: Justify::Right, linebreak: LineBreak::WordBoundary },
    // //     UiText{valign:UiTextVAlign::Bottom,..Default::default()},
    // //     MyText::new("aaa"),
    // // ));

    // commands.entity(root_entity).with_child((
    //     FpsText,
    //     UiRoot::default(),
    //     // TextMarker,
    //     // Text::new("aaa"),
    //     UiText::new("aaa"),
    //     TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 42.0, ..Default::default()},
    //     TextColor(Color::linear_rgb(1.0, 0.2, 0.2)),
    //     UiSize::px(90.0, 55.0),
    //     UiAlign::top(),
    // ));

    let left_container_entity=commands.spawn((
        UiSpan{ span: 1 },
        UiColor{back:Color::srgb(0.5,0.5,0.5),..Default::default()},
        UiGap{hgap:UiVal::Px(30.0),vgap:UiVal::Px(30.0)},
        UiEdge{ padding: UiRectVal::new_px(30.0), ..Default::default() },
        UiFill{ hfill: UiVal::None, vfill: UiVal::Scale(1.0) }
    )).id();

    let right_container_entity=commands.spawn((
        UiSpan{ span: 3 },
        UiColor{back:Color::srgb(0.5,0.5,0.5),..Default::default()},
        UiGap{hgap:UiVal::Px(30.0),vgap:UiVal::Px(30.0)},
        UiEdge{ padding: UiRectVal::new_px(30.0), ..Default::default() },
    )).id();

    commands.entity(root_entity).add_children(&[left_container_entity,right_container_entity]);

    commands.entity(left_container_entity).with_children(|parent|{
        for _ in 0..2 {
            let entity=parent.spawn(()).id();
            create_ui_box(&mut parent.commands(), &mut rng, font.clone(),entity);

            parent.commands().entity(entity).with_children(|parent|{
                for _ in 0..2 {
                    let entity=parent.spawn(()).id();
                    create_ui_box(&mut parent.commands(), &mut rng, font.clone(),entity);
                }
            });
        }
    });
    commands.entity(right_container_entity).with_children(|parent|{
        for _ in 0..9 {
            let entity=parent.spawn(()).id();
            create_ui_box(&mut parent.commands(), &mut rng, font.clone(),entity);
        }
    });

}

fn setup_camera(mut commands: Commands) {

    commands.spawn((
        CameraUi::default(),
        Msaa::Sample8,

    ));
}


fn setup_test(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let font: Handle<Font>=asset_server.load("fonts/FiraMono-Medium.ttf");

    //

    commands.spawn(( UiRoot::default(), UiSize::max())).with_child((
        UiImage{ handle: asset_server.load("bevy_logo_dark_big.png"), ..Default::default() },
        // UiAlign::px(600.0, 0.0),
        // UiAlign::scale(1.0, 1.0),
        UiAlign::scale(1.0, 0.9),
        UiSize::px(50.0, 50.0),
        UiColor::default().back(Color::linear_rgb(1.0,0.3, 0.1)),
    ));

    commands.spawn(( UiRoot::default(), UiSize::max())).with_child((

        // UiAlign::px(600.0, 0.0),
        // UiAlign::scale(1.0, 1.0),
        UiAlign::scale(0.0, 0.3),
        UiSize::px(50.0, 50.0),
        UiColor::default().back(Color::linear_rgb(1.0,0.3, 0.1)),
    ));

    commands.spawn(( UiRoot::default(), UiSize::max())).with_child((

        // UiAlign::px(600.0, 0.0),
        UiAlign::scale(0.5, 1.0),
        UiSize::px(50.0, 50.0),
        UiColor::default().back(Color::linear_rgb(1.0,0.3, 0.1)),
    ));

    commands.spawn((
        UiRoot::default(),
        // UiSize::max(),
        UiGap::px(20.0, 20.0),
        UiAlign::top(),
    ))
    .with_child((
        TextFont{ font:font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::WHITE),
        UiText("non11111".into()),
        TextBackgroundColor(Color::linear_rgb(1.0,0.3, 0.1)),
        TextLayout::new_with_justify(Justify::Right),
        // UiAlign::top_right(),
        // UiSize::px(-20.0, -20.0),
        // UiSize::px(200.0, 50.0),
        UiColor::default().back(Color::linear_rgb(0.5,0.1, 0.1)),
    ))
    .with_child((
        TextFont{ font:font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::WHITE),
        UiText("neg11111".into()),
        TextBackgroundColor(Color::linear_rgb(1.0,0.3, 0.1)),
        TextLayout::new_with_justify(Justify::Right),
        // UiAlign::top_right(),
        UiSize::px(-20.0, -20.0),
        // UiSize::px(200.0, 50.0),
        UiColor::default().back(Color::linear_rgb(0.3, 0.2, 0.1)),
    ))
    .with_child((
        TextFont{ font:font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::WHITE),
        UiText("pos11111".into()),
        TextBackgroundColor(Color::linear_rgb(1.0,0.3, 0.1)),
        TextLayout::new_with_justify(Justify::Right),
        // UiAlign::top_right(),
        // UiSize::px(-20.0, -20.0),
        UiSize::px(200.0, 50.0),
        UiColor::default().back(Color::linear_rgb(0.5,0.1, 0.1)),
    ))
    .with_child((
        TextFont{ font:font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::WHITE),
        UiText("pos23456".into()),
        TextBackgroundColor(Color::linear_rgb(1.0,0.3, 0.1)),
        TextLayout::new_with_justify(Justify::Right),
        UiSize::px(100.0, 50.0),
        UiColor::default().back(Color::linear_rgb(0.5,0.1, 0.1)),
    ))
    .with_child((
        TextFont{ font:font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::WHITE),
        UiText("posb23456".into()),
        TextBackgroundColor(Color::linear_rgb(1.0,0.3, 0.1)),
        // TextLayout::new_with_justify(Justify::Right),
        TextLayout::new(Justify::Right, LineBreak::AnyCharacter),
        TextBounds::new_horizontal(30.0),
        UiSize::px(150.0, 50.0),
        UiColor::default().back(Color::linear_rgb(0.5,0.1, 0.1)),
    ))
    // .with_child((

    //     UiSize::px(-40.0, -30.0),
    //     TextFont{ font: font.clone(), font_size: 15.0, ..Default::default() },
    //     TextColor(Color::linear_rgb(1.0,0.0,0.0)),
    //     TextLayout{
    //         // justify: Justify::Left,
    //         // justify: Justify::Right,
    //         justify: Justify::Center,
    //         linebreak: LineBreak::WordBoundary,
    //     },

    //     UiText::new("caaaaaaaaaaaaaaaac"),

    //     TextBackgroundColor(Color::linear_rgb(0.2,0.5,0.7)),
    //     UiColor::default().back(Color::linear_rgb(0.3, 0.2, 0.1)),


    // ))
    ;

}

#[derive(Component)]
struct FpsText;


fn setup_fps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let font: Handle<Font>=asset_server.load("fonts/FiraMono-Medium.ttf");


    commands.spawn((
        UiRoot::default(),
        FpsText,
        TextFont{ font:font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::WHITE),
        UiText("aaa".into()),
        TextBackgroundColor(Color::linear_rgb(1.0,0.3, 0.1)),
        TextLayout::new_with_justify(Justify::Right),
        // UiAlign::top_right(),
        // UiSize::px(-20.0, -20.0),
        // UiSize::px(200.0, 50.0),
        // UiColor::default().back(Color::linear_rgb(0.5,0.1, 0.1)),
        UiAlign::top_right(),
    ));

}

// fn test_exit(
//     mut exit: bevy::ecs::message::MessageWriter<AppExit>,
//     mut b:bevy::ecs::system::Local<bool>,
// ) {
//     if !*b {
//         *b=true;
//     } else {
//         exit.write(AppExit::Success);
//     }
// }

fn show_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut marker_query: Query< &mut UiText,With<FpsText>>,
) {
    if let Ok(mut text)=marker_query.single_mut() {
        if let Some(v)=diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|v|v.smoothed()) {
            text.0 =format!("{:.0}",v.round());
        } else {
            text.0="".into();
        }
    }
}




fn update_ui_roots(
    windows: Query<&Window>,
    mut root_query: Query<&mut UiRoot,>,
) {
    if let Ok(window)=windows.single() {
        for mut ui_root in root_query.iter_mut() {
            ui_root.width=window.width();
            ui_root.height=window.height();
            // ui_root.scaling=window.resolution.base_scale_factor();
        }
    }
}