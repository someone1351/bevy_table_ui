
use std::collections::HashSet;

use bevy::app::*;
use bevy::asset::prelude::*;
use bevy::color::Color;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::prelude::*;
use bevy::text::*;
use bevy::ui::{AlignSelf, JustifySelf, Node};
use bevy::window::*;
use bevy::DefaultPlugins;
use bevy::prelude::{BuildChildren, Camera3d, ChildBuild, KeyCode, PluginGroup, Text};

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
                        title: "table ui".into(),
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
    // commands.spawn((
    //     MenuUiRoot,
    //     UiLayoutComputed::default(),
    //     UiColor{back:Color::srgb(0.2,0.4,0.9),..Default::default()},
    //     UiSize{
    //          width:UiVal::Px(200.0),
    //          height:UiVal::Px(500.0),
    //     },
    // ));
    
    commands.spawn((
        MenuUiRoot,
        UiLayoutComputed::default(),
        UiColor{back:Color::srgb(0.2,0.4,0.6),..Default::default()},
        UiSize{
            // width:UiVal::Px(200.0),
            width:UiVal::None,
            // height:UiVal::Px(500.0),
            height:UiVal::None,
        },
        UiSpan{span:1},
        UiGap{hgap:UiVal::Px(30.0),vgap:UiVal::Px(30.0)},
    )).with_children(|parent|{

        //0
        parent.spawn((
            UiLayoutComputed::default(),
            // UiInnerSize::default(),
            UiColor{
                back:Color::BLACK,
                cell:Color::srgb(0.3,0.3,0.3),
                ..Default::default()
            },
            UiImage{
                handle:asset_server.load("bevy_logo_dark_big.png"),
                width_scale:0.5,
                height_scale:0.5,
                ..Default::default()
            },
            UiCongruent { 
                row_width_scale: 0.0, 
                col_height_scale: 1.0, 
            },
        ));

        //1
        parent.spawn((
            UiLayoutComputed::default(),
            UiColor{
                back:Color::srgb(1.0,0.3,0.1),
                cell:Color::srgb(1.0,0.5,0.1),
                ..Default::default()
            },
            UiText{
                value:"Hello".to_string(),
                font_size:30.0,
                color:Color::WHITE,
                font:asset_server.load("fonts/FiraMono-Medium.ttf"),
                halign:UiTextHAlign::Right,
                update:true,..Default::default()
            },
            UiFill{ 
                hfill: UiVal::None,
                // hfill: UiVal::Scale(1.0), 
                // vfill: UiVal::None,
                vfill: UiVal::Scale(1.0), 
            },
            // UiSize{width:UiVal::Px(200.0),height:UiVal::Px(70.0)},
            UiSize{width:UiVal::Px(200.0),height:UiVal::None},
            // UiAlign{halign:UiVal::Scale(0.0),..Default::default()},
        ));
        
        //2
        parent.spawn((
            UiLayoutComputed::default(),
            UiColor{
                back:Color::srgb(1.0,0.3,0.1),
                cell:Color::srgb(1.0,0.5,0.1),
                ..Default::default()
            },
            UiText{
                value:"Hello".to_string(),
                font_size:30.0,
                color:Color::WHITE,
                font:asset_server.load("fonts/FiraMono-Medium.ttf"),
                halign:UiTextHAlign::Left,
                valign:UiTextVAlign::Top,
                update:true,..Default::default()
            },
            UiFill{ 
                // hfill: UiVal::None,
                hfill: UiVal::Scale(1.0), 
                vfill: UiVal::None,
            },
            // UiSize{width:UiVal::Px(200.0),height:UiVal::Px(70.0)},
        ));

        //3
        parent.spawn((
            UiLayoutComputed::default(),
            UiColor{
                back:Color::srgb(1.0,0.3,0.1),
                cell:Color::srgb(1.0,0.5,0.1),
                ..Default::default()
            },
            UiText{
                value:"Hello".to_string(),
                font_size:30.0,
                color:Color::WHITE,
                font:asset_server.load("fonts/FiraMono-Medium.ttf"),
                halign:UiTextHAlign::Center,
                valign:UiTextVAlign::Center,
                update:true,..Default::default()
            },
            UiFill{ 
                // hfill: UiVal::None,
                hfill: UiVal::Scale(1.0), 
                vfill: UiVal::None,
            },
            // UiSize{width:UiVal::Px(200.0),height:UiVal::None},
        ));

        //4
        parent.spawn((
            UiLayoutComputed::default(),
            UiColor{
                back:Color::srgb(1.0,0.3,0.1),
                cell:Color::srgb(1.0,0.5,0.1),
                ..Default::default()
            },
            UiText{
                value:"X".to_string(),
                hlen:3,
                vlen:3,
                font_size:30.0,
                color:Color::WHITE,
                font:asset_server.load("fonts/FiraMono-Medium.ttf"),
                halign:UiTextHAlign::Right,
                // halign:UiTextHAlign::Left,
                valign:UiTextVAlign::Bottom,
                // valign:UiTextVAlign::Top,
                update:true,..Default::default()
            },
            UiFill{ 
                // hfill: UiVal::None,
                // hfill: UiVal::Scale(1.0), 
                // vfill: UiVal::None,
                ..Default::default()
            },
            UiSize{
                // width:UiVal::Px(200.0),
                // height:UiVal::Px(70.0), 
                ..Default::default()
            },
            UiCongruent { 
                // row_width_scale: 0.0, 
                // col_height_scale: 1.0, 
                ..Default::default()
            },
        ));
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
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn((
        Text::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {align_self:AlignSelf::Start,justify_self:JustifySelf::End,..Default::default()},
    )).with_child((
        TextSpan::new(""),
        TextColor::from(bevy::color::palettes::css::WHITE),
        TextFont {font:font.clone(),font_size: 15.0,..Default::default()},
        FpsText
    ));
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
