
use bevy::app::*;
use bevy::asset::prelude::*;
use bevy::color::Color;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::prelude::*;
use bevy::text::*;
use bevy::ui::{AlignSelf, JustifySelf, Style};
use bevy::window::*;
use bevy::DefaultPlugins;
use bevy::prelude::{BuildChildren, Camera, Camera3dBundle, KeyCode, PluginGroup, TextBundle};

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
    commands.spawn((
        MenuUiRoot,
        UiLayoutComputed::default(),
        UiColor{back:Color::srgb(0.2,0.4,0.6),..Default::default()},
        UiSize{
            //width:UiVal::Px(500.0),
            width:UiVal::None,
            height:UiVal::Px(500.0),
        },
        UiSpan{span:1},
        UiGap{hgap:UiVal::Px(30.0),vgap:UiVal::Px(30.0)},
    )).with_children(|parent|{
        parent.spawn((
            UiLayoutComputed::default(),
            UiInnerSize::default(),
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
        ));
        parent.spawn((
            UiLayoutComputed::default(),
            UiColor{
                back:Color::srgb(1.0,0.3,0.1),
                cell:Color::srgb(1.0,0.5,0.1),
                ..Default::default()
            },
            UiInnerSize::default(),
            UiTextComputed::default(),
            UiText{
                value:"Hello".to_string(),
                font_size:40.0,
                color:Color::WHITE,
                font:asset_server.load("FiraMono-Medium.ttf"),
                update:true,..Default::default()
            },
            UiFill{ 
                hfill: UiVal::None,
                // hfill: UiVal::Scale(1.0), 
                // vfill: UiVal::None,
                vfill: UiVal::Scale(1.0), 
            },
            UiSize{width:UiVal::Px(200.0),height:UiVal::Px(70.0)},
            UiAlign{halign:UiVal::Scale(0.0),..Default::default()},
        ));
        
        parent.spawn((
            UiLayoutComputed::default(),
            UiColor{
                back:Color::srgb(1.0,0.3,0.1),
                cell:Color::srgb(1.0,0.5,0.1),
                ..Default::default()
            },
            UiInnerSize::default(),
            UiTextComputed::default(),
            UiText{
                value:"Hello".to_string(),
                font_size:40.0,
                color:Color::WHITE,
                font:asset_server.load("FiraMono-Medium.ttf"),
                halign:UiTextHAlign::Left,
                valign:UiTextVAlign::Top,
                update:true,..Default::default()
            },
            UiFill{ 
                // hfill: UiVal::None,
                hfill: UiVal::Scale(1.0), 
                vfill: UiVal::None,
            },
            UiSize{width:UiVal::Px(200.0),height:UiVal::Px(70.0)},
        ));
        parent.spawn((
            UiLayoutComputed::default(),
            UiColor{
                back:Color::srgb(1.0,0.3,0.1),
                cell:Color::srgb(1.0,0.5,0.1),
                ..Default::default()
            },
            UiInnerSize::default(),
            UiTextComputed::default(),
            UiText{
                value:"Hello".to_string(),
                font_size:40.0,
                color:Color::WHITE,
                font:asset_server.load("FiraMono-Medium.ttf"),
                valign:UiTextVAlign::Center,
                update:true,..Default::default()
            },
            UiFill{ 
                // hfill: UiVal::None,
                hfill: UiVal::Scale(1.0), 
                vfill: UiVal::None,
            },
            UiSize{width:UiVal::Px(200.0),height:UiVal::Px(70.0)},
        ));
        parent.spawn((
            UiLayoutComputed::default(),
            UiColor{
                back:Color::srgb(1.0,0.3,0.1),
                cell:Color::srgb(1.0,0.5,0.1),
                ..Default::default()
            },
            UiInnerSize::default(),
            UiTextComputed::default(),
            UiText{
                value:"Hello".to_string(),
                font_size:40.0,
                color:Color::WHITE,
                font:asset_server.load("FiraMono-Medium.ttf"),
                halign:UiTextHAlign::Right,
                valign:UiTextVAlign::Bottom,
                update:true,..Default::default()
            },
            UiFill{ 
                // hfill: UiVal::None,
                hfill: UiVal::Scale(1.0), 
                vfill: UiVal::None,
            },
            UiSize{width:UiVal::Px(200.0),height:UiVal::Px(70.0)},
        ));
    });

}

fn setup_camera(mut commands: Commands) {
    // commands.spawn(( Camera2dBundle { camera: Camera { ..Default::default() }, ..Default::default() }, ));
    commands.spawn((Camera3dBundle { camera: Camera { ..Default::default() }, ..Default::default() },));
}

fn update_input(
    mut key_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut exit: EventWriter<AppExit>,
) {
   
    for ev in key_events.read() {
        if ev.state==bevy::input::ButtonState::Pressed && ev.key_code==KeyCode::Escape {
            exit.send(AppExit::Success); 
        }
    }
}

#[derive(Component)]
struct FpsText;

fn setup_fps(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("FiraMono-Medium.ttf");
    let text_style=TextStyle {font:font.clone(), font_size: 25.0, color: Color::WHITE};
    let text_bundle=TextBundle::from_section("", text_style);
    let fps_style=Style{align_self:AlignSelf::Start,justify_self:JustifySelf::End,..Default::default()};
    commands.spawn(text_bundle.with_text_justify(JustifyText::Left).with_style(fps_style)).insert(FpsText);
}

fn show_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut marker_query: Query<&mut Text, With<FpsText>>
) {
    if let Ok(mut text)=marker_query.get_single_mut() {
        let v=diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
        let fps = v.and_then(|x|x.value()).map(|x|x.round()).unwrap_or_default();
        let avg = v.and_then(|x|x.average()).unwrap_or_default();
        text.sections[0].value =format!("{fps:.0} {avg:.0}");
    }
}