
use std::collections::HashSet;

use bevy::{app::AppExit, asset::Handle, color::Color, ecs::prelude::*, input::{keyboard::{KeyCode, KeyboardInput}, mouse::{MouseButton, MouseButtonInput, MouseScrollUnit, MouseWheel}, ButtonState}, math::Vec2, text::{Font, Justify, LineBreak, TextColor, TextFont, TextLayout}, window::Window};
use bevy_table_ui::{MyText, UiColor, UiCursorable, UiEdge, UiFocusable, UiInteractInputMessage, UiRectVal, UiRoot, UiSize, UiText, UiTextVAlign, UiVal};

use super::affect::{create_affect_attrib, UixAffect, UixAffectState};
use rand::{Rng,rngs::ThreadRng};

pub fn generate_screenshot_path<P>(dir : P, prefix : &str, ext : &str) -> Option<std::path::PathBuf>
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


pub fn update_input(
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


pub fn update_ui_input(
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


pub fn create_ui_box(commands: &mut Commands, rng: &mut ThreadRng, font: Handle<Font>,entity:Entity) {
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

    // let width= create_affect_attrib(
    //     |c:&mut UiSize,v|{c.width=v;},
    //     UiVal::None,
    //     [
    //         (UixAffectState::Focus,UiVal::Px(100.0)),
    //         (UixAffectState::Press(0),UiVal::Px(150.0))
    //     ]
    // );

    let text= create_affect_attrib(
        |c:&mut MyText,v|{c.0=v;},
        "aaa".into(),
        [
            (UixAffectState::Focus,"aba".into()),
            (UixAffectState::Press(0),"aca".into())
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
        // MyText2d(format!("{entity}")),
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
    ))
    // .with_child((
    //     TextSpan(format!("{entity}")),

    //     TextFont{ font: font.clone(), font_size: 15.0, ..Default::default() },
    //     TextColor(Color::linear_rgb(0.0,0.0,1.0)),
    // ))
    ;
}
