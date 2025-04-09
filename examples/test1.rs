
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::collections::HashSet;

use bevy::app::*;
use bevy::asset::prelude::*;
use bevy::color::Color;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::prelude::*;
use bevy::text::*;
// use bevy::ui::{AlignSelf, JustifySelf, Node};
use bevy::window::*;
use bevy::DefaultPlugins;
use bevy::prelude::{BuildChildren, Camera3d, ChildBuild, KeyCode, PluginGroup, };

use bevy_table_ui as table_ui;
use table_ui::*;

fn main() {
    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {watch_for_changes_override:Some(true), ..Default::default() })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "table ui test".into(),
                        resolution: (800.0, 600.0).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            table_ui::UiLayoutPlugin,
            table_ui::UiInteractPlugin,
            table_ui::UiDisplayPlugin,
            table_ui::UiAffectPlugin,
        ))


        // .add_systems(Startup, ( setup_input, setup_camera, setup_menu, ))
        // .add_systems(PreUpdate, ( update_input, ))
        // .add_systems(Update, ( show_menu, ))



        .add_systems(Startup, (
            setup_fps,
            setup_camera,
            setup_ui,
        ))
        .add_systems(Update, (
            update_input,
            show_fps.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(300))),
        ))
        ;

    app.run();
}

#[derive(Component)]
pub struct MenuUiRoot;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn((
        MenuUiRoot,
        UiLayoutComputed::default(),
        UiColor{back:Color::srgb(0.4,0.8,0.4),..Default::default()},
        // UiSize{
        //     width:UiVal::Scale(1.0),
        //     height:UiVal::Scale(1.0),
        // },
        UiSpan{span:1},
        // UiGap{hgap:UiVal::Px(5.0),vgap:UiVal::Px(5.0)},
    )).with_children(|parent|{

        for i in 0 .. 4 {
            if
            true
            // false
            {
                parent.spawn((
                    UiLayoutComputed::default(),
                    UiFill{ hfill: UiVal::Scale(1.0), vfill: UiVal::None },
                    UiColor{
                        back:Color::srgb(0.1,0.5,1.0),
                        border:Color::srgb(0.8,0.8,0.8),
                        ..Default::default()
                    },
                    UiEdge{ border:UiRectVal { top: UiVal::Scale(-0.3), ..Default::default() }, ..Default::default() },
                    UiText{
                        value:format!("{i}").repeat(i+1),
                        font_size:30.0,
                        color:Color::WHITE,
                        font:asset_server.load("fonts/FiraMono-Medium.ttf"),
                        update:true,
                        ..Default::default()
                    },
                ));
            } else {
                parent.spawn((
                    UiLayoutComputed::default(),
                    UiFill{ hfill: UiVal::Scale(1.0), vfill: UiVal::None },
                    UiColor{ back:Color::srgb(0.5,0.1,1.0), ..Default::default() },
                    UiSize{ width:UiVal::None, height:UiVal::Scale(-1.5), },
                )).with_children(|parent|{
                    parent.spawn((
                        UiLayoutComputed::default(),
                        // UiFill{ hfill: UiVal::Scale(1.0), vfill: UiVal::None },
                        UiColor{ back:Color::srgb(0.1,0.5,1.0), ..Default::default() },
                        UiText{
                            value:format!("{i}").repeat(i+1),
                            font_size:30.0,
                            color:Color::WHITE,
                            font:asset_server.load("fonts/FiraMono-Medium.ttf"),
                            update:true,
                            ..Default::default()
                        },
                    ));
                });
            }

        }


    });

}

fn setup_camera(mut commands: Commands) {
    // commands.spawn(( Camera2dBundle { camera: Camera { ..Default::default() }, ..Default::default() }, ));
    // commands.spawn((Camera3dBundle { camera: Camera { ..Default::default() }, ..Default::default() },));
    commands.spawn((Camera3d::default(),));
}

fn update_input(
    mut key_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut exit: EventWriter<AppExit>,
    // mut screenshot_manager: ResMut<bevy::render::view::screenshot::ScreenshotManager>,
    // main_window: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut last_pressed:Local<HashSet<KeyCode>>,
    mut commands: Commands,


) {
    // let Ok(window_entity) = main_window.get_single() else {return;};

    for ev in key_events.read() {
        if ev.state==bevy::input::ButtonState::Pressed && !last_pressed.contains(&ev.key_code) {
            if ev.key_code==KeyCode::Escape || ev.key_code==KeyCode::F4 {
                exit.send(AppExit::Success);
            } else if ev.key_code==KeyCode::F12 {
                if let Some(path) = generate_screenshot_path("./screenshots","screenshot_","png") {
                    // if screenshot_manager.save_screenshot_to_disk(window_entity, &path).is_err() {
                    //     eprintln!("Failed to take screenshot at {path:?}.");
                    // }
                    commands
                        .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                        .observe(bevy::render::view::screenshot::save_to_disk(path));
                }
            }
        }

        if ev.state==bevy::input::ButtonState::Pressed {
            last_pressed.insert(ev.key_code);
        } else if ev.state==bevy::input::ButtonState::Released {
            last_pressed.remove(&ev.key_code);
        }
    }
}

#[derive(Component)]
struct FpsText;

fn setup_fps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    // commands.spawn((
    //     Text::default(),
    //     TextLayout::new_with_justify(JustifyText::Center),
    //     Node {align_self:AlignSelf::Start,justify_self:JustifySelf::End,..Default::default()},
    // )).with_child((
    //     TextSpan::new(""),
    //     TextColor::from(bevy::color::palettes::css::WHITE),
    //     TextFont {font:font.clone(),font_size: 15.0,..Default::default()},
    //     FpsText
    // ));
}

fn show_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut marker_query: Query< &mut TextSpan,With<FpsText>>,
) {
    if let Ok(mut text)=marker_query.get_single_mut() {
        let v=diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
        let fps = v.and_then(|x|x.value()).map(|x|x.round()).unwrap_or_default();
        let avg = v.and_then(|x|x.average()).unwrap_or_default();
        text.0 =format!("{fps:.0} {avg:.0}");
    }
}




fn generate_screenshot_path<P>(dir : P, prefix : &str, ext : &str) -> Option<std::path::PathBuf>
where
    P: AsRef<std::path::Path>,
{
    let dir=dir.as_ref();
    let name_start=prefix.to_string();
    let name_end=".".to_string()+ext;

    //
    let mut last_num=0;

    //
    if !std::fs::create_dir_all(dir).is_ok() {
        eprintln!("Failed to create screenshot directory {dir:?}.");
        return None;
    }

    let Ok(existing) = std::fs::read_dir(dir) else {
        eprintln!("Failed to read screenshot directory {dir:?}.");
        return None;
    };

    for x in existing.into_iter() {
        let Ok(x)=x else {
            continue;
        };

        let Some(x)=x.file_name().to_str().map(|x|x.to_string()) else {
            continue;
        };

        if !x.starts_with(name_start.as_str()) || !x.ends_with(name_end.as_str()) {
            continue;
        }

        let Ok(x)=x[name_start.len() .. x.len()-name_end.len()].to_string().parse::<u32>() else {
            continue;
        };

        last_num=last_num.max(x);
    }

    //
    Some(dir.join(format!("{name_start}{:04}{name_end}", last_num+1)))
}
