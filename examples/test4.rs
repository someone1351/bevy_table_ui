// mod my_text;

use std::collections::BTreeSet;

use bevy::{platform::collections::HashSet, prelude::*, text::TextLayoutInfo};

// use my_text::*;
use bevy_table_ui::{self as table_ui, CameraUi, UiAlign, UiColor, UiImage, UiRoot, UiSize, UiSpan, UiText };

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
        ).chain())
        .run();
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //init camera
    commands.spawn(CameraUi);

    //init text
    commands.spawn((
        UiRoot::default(),
        UiSpan::new(2),
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

}

