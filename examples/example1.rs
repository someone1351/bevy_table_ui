
// #![allow(unused_mut)]
// #![allow(unused_variables)]
// #![allow(unreachable_code)]

use std::collections::HashSet;
// use std::sync::Arc;

use bevy::app::*;
use bevy::asset::prelude::*;
use bevy::color::Color;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
// use bevy::ecs::prelude::*;

use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Local, Query, Res};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseScrollUnit, MouseWheel};
use bevy::input::ButtonState;
use bevy::math::Vec2;
use bevy::text::*;
// use bevy::ui::{AlignSelf, JustifySelf, Node};
use bevy::window::*;
use bevy::DefaultPlugins;
use bevy::prelude::{KeyCode, Msaa, PluginGroup };


use bevy_table_ui as table_ui;
use rand::{Rng,rngs::ThreadRng};
// use rand::Rng;
// use mesh::TestRenderComponent;
// use render_core::core_my::CameraMy;
use table_ui::*;

use crate::affect::{create_affect_attrib, UixAffect, UixAffectState};

// #[path = "affect/mod.rs"]
mod affect;

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
            table_ui::UiLayoutPlugin,
            table_ui::UiInteractPlugin,
            table_ui::UiDisplayPlugin,
            // table_ui::UiAffectPlugin,
            affect::UixAffectPlugin,
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
            update_ui_roots,
            update_ui_input.before(UiInteractSystem),
            // update_ui,
            // on_affects,
        ).chain())
        .add_systems(Update, (
            update_input,
            show_fps.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(300))),
        ))
        ;

    app.run();
}

pub fn update_ui_roots(
    windows: Query<&Window>,
    mut root_query: Query<&mut UiRoot,>,
) {

    let window_size=windows.single()
        .and_then(|window|Ok((window.width(),window.height())))
        .unwrap_or_default();

    for mut x in root_query.iter_mut() {
        x.width=window_size.0;
        x.height=window_size.1;
    }
}



#[derive(Component)]
pub struct MenuUiRoot;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut rng: ThreadRng = rand::thread_rng();

    let font: Handle<Font>=asset_server.load("fonts/FiraMono-Medium.ttf");

    let root_entity= commands.spawn((
        MenuUiRoot,
        UiRoot::default(),
        UiGap{hgap:UiVal::Px(20.0),vgap:UiVal::None},
    )).id();

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

fn create_ui_box(commands: &mut Commands, rng: &mut ThreadRng, font: Handle<Font>,entity:Entity) {
    let border_col= create_affect_attrib(|c:&mut UiColor,v|c.border=v,Color::linear_rgb(0.5,0.5,0.5),[
        (UixAffectState::Focus,Color::linear_rgb(0.8,0.6,0.3)),
        (UixAffectState::Press(0),Color::linear_rgb(1.0,0.8,0.1))
    ]);

    // let text= create_affect_attrib(
    //     |c:&mut UiText,v|{c.value=v;c.update=true;},
    //     "abc".into(),
    //     [
    //         (UixAffectState::Focus,"bbb".into()),
    //         (UixAffectState::Press(0),"ccc".into())
    //     ]
    // );

    let width= create_affect_attrib(
        |c:&mut UiSize,v|{c.width=v;},
        UiVal::None,
        [
            (UixAffectState::Focus,UiVal::Px(100.0)),
            (UixAffectState::Press(0),UiVal::Px(150.0))
        ]
    );
    let text= create_affect_attrib(
        |c:&mut MyText2d,v|{c.0=v;},
        "abc".into(),
        [
            (UixAffectState::Focus,"byb".into()),
            (UixAffectState::Press(0),"czc".into())
        ]
    );
    let c=[rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>()];
    let col=Color::srgb_from_array(c.map(|c|c*0.8));
    let col2=Color::srgb_from_array(c.map(|c|c));
    let back_col= create_affect_attrib(|c:&mut UiColor,v|c.back=v,col,[
        (UixAffectState::Hover,col2)
    ]);

    commands.entity(entity).insert((
        UixAffect(vec![
            back_col,
            border_col,
            text,
            // width,
        ]),
        UiSize{ width:UiVal::Px(-20.0), height:UiVal::Px(-30.0), },
        UiFocusable{
            enable: true,
            hexit:false,vexit:true,
            hwrap:true,vwrap:true,
            pressable:true,
            // press_onlys:[0].into(),
            ..Default::default()
        },
        // UiHoverable{ enable: true },
        UiCursorable{
            // enable: true,
            pressable:true,
            hoverable:true,
            // draggable:true,
            scrollable:true,
            //press_onlys:[0].into(),
            ..Default::default()
        },
        // UiHoverable{ enable: true },
        // UiDraggable{ enable: true },
        UiEdge{
            border: UiRectVal::new_scalar(UiVal::Px(5.0)),
            margin: UiRectVal::new_scalar(UiVal::Px(5.0)),
            ..Default::default() },

        TextFont{ font: font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::linear_rgb(1.0,0.0,0.0)),
        // TextColor(Color::linear_rgb(1.0,0.0,0.0)),
        TextLayout{ justify: Justify::Right, linebreak: LineBreak::WordBoundary },
        // TextLayoutInfo{ scale_factor: todo!(), glyphs: todo!(), section_rects: todo!(), size: todo!() },
        // TextBounds{ width: todo!(), height: todo!() },
        MyText2d(format!("{entity}")),
        UiText{
            // value:format!("{entity}"),
            // font_size: 15.0,
            // halign:UiTextHAlign::Left,
            // valign:UiTextVAlign::Top,
            // halign:UiTextHAlign::Right,
            valign:UiTextVAlign::Bottom,
            // font: font.clone(),
            // color: Color::linear_rgb(1.0,1.0,1.0),
            ..Default::default()
        },
    )).with_child((
        TextSpan(format!("{entity}")),

        TextFont{ font: font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::linear_rgb(0.0,0.0,1.0)),
    ));
}

fn setup_camera(mut commands: Commands) {
    // commands.spawn(( Camera2dBundle { camera: Camera { ..Default::default() }, ..Default::default() }, ));
    // commands.spawn((Camera3dBundle { camera: Camera { ..Default::default() }, ..Default::default() },));
    // commands.spawn((Camera3d::default(),));
    // commands.spawn((CameraMy::default(),));
    commands.spawn((
        CameraUi::default(),
        Msaa::Sample8,
        // Msaa::Off,
        // Projection::Orthographic(OrthographicProjection::default_2d()),

        // Camera {
        //     // target: image_handle.clone().into(),
        //     clear_color: Color::WHITE.into(),
        //     order: 0,
        //     // clear_color: ClearColorConfig::Custom(Color::srgb(0.2, 0.1, 0.5)),
        //     viewport: Some(Viewport {
        //         physical_position: UVec2::new(0, 0),
        //         physical_size: UVec2::new(500, 500),
        //         ..Default::default()
        //     }),
        //     ..Default::default()
        // },
        // RenderLayers::layer(0),
        // Transform::from_xyz( 0.0, 0.0, 999.0, ),
    ));

    // commands.spawn((
    //     TestRenderComponent{
    //         col: Color::srgb(1.0,0.0,0.0),
    //         // col: Color::srgb(0.0,0.0,1.0),
    //         // col:Color::WHITE.into(),
    //         x: 0.0, y: 0.0, w: 50.0, h: 50.0,
    //         // handle:Some(asset_server.load("bevy_logo_dark_big.png")),
    //         handle:None,
    //     },
    //     // // RenderLayers::layer(1),
    //     // RenderLayers::from_layers(&[0]),
    //     // Transform::from_xyz( 0.0, 0.0, 0.0, ),
    // ));
    // commands.spawn((
    //     TestRenderComponent{
    //         col: Color::srgb(0.0,1.0,0.0),
    //         x: 50.0, y: 50.0, w: 50.0, h: 50.0,
    //         handle:None,
    //     },
    // ));
    // commands.spawn((
    //     TestRenderComponent{
    //         col: Color::srgb(0.0,0.0,1.0),
    //         x: 100.0, y: 100.0, w: 50.0, h: 50.0,
    //         handle:None,
    //     },
    // ));
}

fn update_ui_input(
    mut windows: Query<&mut Window>,
    mut prev_cursor : Local<Option<Vec2>>,
    mut ui_interact_input_event_writer: MessageWriter<UiInteractInputMessage>,
    ui_root_query : Query<Entity,With<UiRoot>>,

    mut key_events: MessageReader<KeyboardInput>,
    mut mouse_button_events : MessageReader<MouseButtonInput>,
    mut key_lasts : Local<HashSet<KeyCode>>,
    mut mouse_scroll_events: MessageReader<MouseWheel>,
){

    let Ok(window) = windows.single_mut() else {return;};
    let mouse_cursor = window.cursor_position();//.unwrap_or(Vec2::ZERO);

    //

    let device=0;
    let group=0;
    let device2=1;

    //
    for ev in key_events.read() {
        match ev.state {
            ButtonState::Pressed if !key_lasts.contains(&ev.key_code) => {
                key_lasts.insert(ev.key_code);

                for root_entity in ui_root_query.iter() {
                    match ev.key_code {

                        // KeyCode::Tab => {
                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::FocusNext { root_entity, group, device });
                        // }
                        // KeyCode::Backquote => {
                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPrev { root_entity, group, device });
                        // }
                        KeyCode::KeyW => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusUp { root_entity, group, device });
                        }
                        KeyCode::KeyS => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusDown { root_entity, group, device });
                        }
                        KeyCode::KeyA => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusLeft { root_entity, group, device });
                        }
                        KeyCode::KeyD => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusRight { root_entity, group, device });
                        }

                        KeyCode::ArrowUp => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusUp { root_entity, group, device:device2 });
                        }
                        KeyCode::ArrowDown => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusDown { root_entity, group, device:device2 });
                        }
                        KeyCode::ArrowLeft => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusLeft { root_entity, group, device:device2 });
                        }
                        KeyCode::ArrowRight => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusRight { root_entity, group, device:device2 });
                        }

                        KeyCode::Tab|KeyCode::KeyE => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusNext { root_entity, group, device });
                        }
                        KeyCode::Backquote|KeyCode::KeyQ => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPrev { root_entity, group, device });
                        }

                        KeyCode::BracketRight => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusNext { root_entity, group, device:device2});
                        }
                        KeyCode::BracketLeft => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPrev { root_entity, group, device:device2 });
                        }
                        KeyCode::Space => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressBegin{root_entity, group, device,button:0, });
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusEnter {root_entity, group, device, });
                        }
                        KeyCode::Enter => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressBegin{root_entity, group, device: device2,button:0, });
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusEnter {root_entity, group, device: device2, });
                        }
                        KeyCode::Escape => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressCancel{root_entity, device,button:0, });
                        }
                        KeyCode::Backspace => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressCancel{root_entity, device:device2,button:0, });
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusExit {root_entity, group, device, });
                        }
                        _ => {}
                    }
                }
            }
            ButtonState::Released => {
                key_lasts.remove(&ev.key_code);

                for root_entity in ui_root_query.iter() {
                    match ev.key_code {
                        KeyCode::Space => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressEnd{root_entity, device, button: 0 });
                        }
                        KeyCode::Enter => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressEnd{root_entity, device:device2, button: 0 });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    //
    for ev in mouse_button_events.read() {
        match ev.state {
            ButtonState::Pressed => {
                for root_entity in ui_root_query.iter() {
                    match ev.button {
                        MouseButton::Left => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressBegin{root_entity, device, button: 0 });

                            // ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 1 });
                            // ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 2 });
                        }
                        MouseButton::Right => {
                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressBegin{root_entity, device, button: 2 });

                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 0 });
                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 1 });
                        }
                        // MouseButton::Middle => {
                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressBegin{root_entity, device, button: 1 });

                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 0 });
                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 2 });
                        // }
                        MouseButton::Forward => {
                        }
                        MouseButton::Back => {
                        }
                        _ => {}
                    }
                }
            }
            ButtonState::Released => {
                for root_entity in ui_root_query.iter() {
                    match ev.button {
                        MouseButton::Left => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressEnd {root_entity, device, button: 0 });
                        }
                        // MouseButton::Right => {
                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressEnd {root_entity, device, button: 2 });
                        // }
                        // MouseButton::Middle => {
                        //     ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressEnd {root_entity, device, button: 1 });
                        // }
                        _ => {}
                    }
                }
            }
        }
    }

    //
    for ev in mouse_scroll_events.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                // println!("scroll line {} {}",ev.x,ev.y);
                if ev.x!=0.0 {
                    for root_entity in ui_root_query.iter() {
                        ui_interact_input_event_writer.write(UiInteractInputMessage::CursorScroll { root_entity, device: 0, axis: 1, scroll: ev.x });
                    }
                }
                if ev.y!=0.0 {
                    for root_entity in ui_root_query.iter() {
                        ui_interact_input_event_writer.write(UiInteractInputMessage::CursorScroll { root_entity, device: 0, axis: 0, scroll: ev.y });
                    }
                }
            }
            MouseScrollUnit::Pixel => {
                // println!("scroll Pixel {} {}",ev.x,ev.y);
            }
        }
    }

    for root_entity in ui_root_query.iter() {
        if *prev_cursor!=mouse_cursor {
            let player=0;
            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorMoveTo{root_entity,device: player,cursor:mouse_cursor});
        }
    }

    *prev_cursor=mouse_cursor;

}


fn update_input(
    mut key_events: MessageReader<bevy::input::keyboard::KeyboardInput>,
    mut exit: MessageWriter<AppExit>,
    // mut screenshot_manager: ResMut<bevy::render::view::screenshot::ScreenshotManager>,
    // main_window: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut last_pressed:Local<HashSet<KeyCode>>,
    mut commands: Commands,

    mut root_query: Query<&mut UiRoot,>,
) {
    // let Ok(window_entity) = main_window.get_single() else {return;};


    for ev in key_events.read() {
        if ev.state==bevy::input::ButtonState::Pressed && !last_pressed.contains(&ev.key_code) {
            match ev.key_code {
                KeyCode::Escape | KeyCode::F4 => {
                    exit.write(AppExit::Success);

                }
                KeyCode::F12 => {
                    if let Some(path) = generate_screenshot_path("./screenshots","screenshot_","png") {
                        // if screenshot_manager.save_screenshot_to_disk(window_entity, &path).is_err() {
                        //     eprintln!("Failed to take screenshot at {path:?}.");
                        // }
                        commands
                            .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                            .observe(bevy::render::view::screenshot::save_to_disk(path));
                    }
                }
                KeyCode::Equal => {
                    // println!("plus");

                    for mut x in root_query.iter_mut() {
                        x.scaling+=0.25;
                    }
                }
                KeyCode::Minus => {
                    // println!("minus");
                    for mut x in root_query.iter_mut() {
                        x.scaling-=0.25;
                        x.scaling=x.scaling.max(0.0);
                    }
                }
                // KeyCode::ArrowLeft => {
                //     for mut x in root_query.iter_mut() {
                //         x.x-=100.0;
                //     }
                // }
                // KeyCode::ArrowRight => {
                //     for mut x in root_query.iter_mut() {
                //         x.x+=100.0;
                //     }
                // }
                // KeyCode::ArrowUp => {
                //     for mut x in root_query.iter_mut() {
                //         x.y-=100.0;
                //     }
                // }
                // KeyCode::ArrowDown => {
                //     for mut x in root_query.iter_mut() {
                //         x.y+=100.0;
                //     }
                // }

                _ => {

                    // println!("key {ev:?}");
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

    let font: Handle<Font>=asset_server.load("fonts/FiraMono-Medium.ttf");


    commands.spawn((
        UiRoot::default(),

        TextFont{ font: font.clone(), font_size: 15.0, ..Default::default() },
        TextColor(Color::WHITE),
        UiAlign{ halign: UiVal::Scale(1.0), valign: UiVal::Scale(0.0) },
        MyText2d("aaa".into()),
        // TextSpan(format!("aaa")),
        UiText{..Default::default()},

    ));


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
    if let Ok(mut text)=marker_query.single_mut() {
        let v=diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
        let fps = v.and_then(|x|x.value()).map(|x|x.round()).unwrap_or_default();
        let avg = v.and_then(|x|x.average()).unwrap_or_default();
        // text.0 =format!("{fps:.0} {avg:.0}");
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
