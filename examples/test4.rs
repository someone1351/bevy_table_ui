// mod my_text;

use std::collections::BTreeSet;

use bevy::{camera::{visibility::RenderLayers, Viewport}, platform::collections::HashSet, prelude::*, text::TextLayoutInfo};

// use my_text::*;
use bevy_table_ui::{self as table_ui, CameraUi, UiAlign, UiColor, UiImage, UiRoot, UiSize, UiSpan, UiText };

mod common;
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
        ).chain())
        .run();
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //init camera
    commands.spawn((
        bevy_table_ui::CameraUi,
        Camera {
            clear_color:ClearColorConfig::Custom(Color::srgba(0.3, 0.5, 0.3,0.5)),
            order: 1,
            viewport: Some(Viewport {
                physical_position: UVec2::new(200, 200),
                physical_size: UVec2::new(1200, 700),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));
    commands.spawn((
        bevy_table_ui::CameraUi,
        Camera {
            clear_color:ClearColorConfig::Custom(Color::srgba(0.6, 0.3, 0.3,0.5)),
            order: 2,
            viewport: Some(Viewport {
                // physical_position: UVec2::new(500, 0),

                physical_position: UVec2::new(400, 0),
                physical_size: UVec2::new(1200, 700),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));

    //init text
    commands.spawn((
        UiRoot::default(),
        UiSpan::new(2),
        RenderLayers::layer(1),
        UiAlign::top_left(),
    )).with_child((
        UiColor::default().back(Color::linear_rgb(1.0, 0.2, 0.2)),
        UiSize::px(100.0, 100.0),
    )).with_child((
        UiColor::default().back(Color::linear_rgb(0.2, 1.0, 0.2)),
        UiSize::px(100.0, 100.0),
    )).with_child((
        UiColor::default().back(Color::linear_rgb(0.2, 0.2, 1.0)),
        UiSize::px(100.0, 100.0),
    )).with_child((
        UiColor::default().back(Color::linear_rgb(1.0, 0.2, 1.0)),
        UiSize::px(100.0, 100.0),
    ));

    //

    // commands.spawn((
    //     Camera2d,
    //     Camera {
    //         clear_color:ClearColorConfig::Custom(Color::srgba(0.9,0.3, 0.3, 1.0)),
    //         order: 3,
    //         viewport: Some(Viewport {
    //             physical_position: UVec2::new(0, 0),
    //             physical_size: UVec2::new(555, 555),
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     },
    //     RenderLayers::layer(2),
    // ));
    // commands.spawn((
    //     Camera2d,
    //     Camera {
    //         clear_color:ClearColorConfig::Custom(Color::srgba(0.3, 0.3, 0.9,1.0)),
    //         order: 4,
    //         viewport: Some(Viewport {
    //             physical_position: UVec2::new(0, 555),
    //             physical_size: UVec2::new(555, 555),
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     },
    //     RenderLayers::layer(3),
    // ));
    // commands.spawn((
    //     Sprite::from_image(asset_server.load("bevy_logo_dark_big.png")),

    //     RenderLayers::layer(2),
    // ));
}

