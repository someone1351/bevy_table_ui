
// #![allow(unused_mut)]
// #![allow(unused_variables)]
// #![allow(unreachable_code)]

use std::collections::{BTreeSet, HashSet};
// use std::sync::Arc;

use bevy::app::*;
use bevy::asset::prelude::*;
use bevy::color::Color;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
// use bevy::ecs::prelude::*;

use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageReader;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Local, Query, Res, ResMut};
use bevy::image::Image;

use bevy::text::*;
// use bevy::ui::{AlignSelf, JustifySelf, Node};
use bevy::window::*;
use bevy::DefaultPlugins;
use bevy::prelude::{Msaa, PluginGroup };


use bevy_table_ui as table_ui;
use rand::rngs::ThreadRng;
// use rand::Rng;
// use mesh::TestRenderComponent;
// use render_core::core_my::CameraMy;
use table_ui::*;


// #[path = "affect/mod.rs"]
mod affect;
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
        ).chain())
        .add_systems(Update, (
            update_ui_roots,
            update_ui_input.before(UiInteractSystem),
            // update_ui,
            // on_affects,
            update_input,
            show_fps, //.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(100))),
            on_affects2,
        ).chain())
        // .add_systems(Update, (
        // ))
        ;

    app.run();
}



pub fn on_affects2<'a>(
    mut commands: Commands,
    mut interact_event_reader: MessageReader<UiInteractEvent>,
    mut text_query: Query< &mut MyText,>,
) {

    //
    for ev in interact_event_reader.read() {

        match ev.event_type {
            UiInteractMessageType::CursorPressBegin {  .. }
            |UiInteractMessageType::FocusPressBegin { .. } => {
                text_query.get_mut(ev.entity).unwrap().0="aca".into();

            }
            UiInteractMessageType::CursorPressEnd {  ..}
            |UiInteractMessageType::FocusPressEnd { .. } => {

            }

            UiInteractMessageType::FocusBegin { .. } => {
                text_query.get_mut(ev.entity).unwrap().0="aba".into();

            }
            UiInteractMessageType::FocusEnd { .. } => {
                text_query.get_mut(ev.entity).unwrap().0="aaa".into();
            }
            _ => {}
        }
            // commands.queue(move|world:&mut World|func(entity,world,));

    }


}




pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let mut rng: ThreadRng = rand::thread_rng();

    // let font: Handle<Font>=asset_server.load("fonts/FiraMono-Medium.ttf");

    let root_entity= commands.spawn((
        UiRoot::default(),
        UiGap{hgap:UiVal::Px(20.0),vgap:UiVal::None},
    )).id();


    // let entity=commands.entity(root_entity).with_child((
    //     UiSize::px(90.0, 55.0),
    // )).id();
    // create_ui_box(&mut commands, &mut rng, font.clone(),entity);

    // commands.entity(root_entity).with_child((
    //     TextFont{ font: font.clone(), font_size: 15.0, ..Default::default() },
    //     TextColor(Color::linear_rgb(1.0,0.0,0.0)),
    //     TextLayout{ justify: Justify::Right, linebreak: LineBreak::WordBoundary },
    //     UiText{valign:UiTextVAlign::Bottom,..Default::default()},
    //     MyText::new("aaa"),
    // ));

    commands.entity(root_entity).with_child((
        FpsText,
        UiRoot::default(),
        // TextMarker,
        // Text::new("aaa"),
        MyText::new("aaa"),
        UiText::default(),
        TextFont {font: asset_server.load("fonts/FiraMono-Medium.ttf"),font_size: 42.0, ..Default::default()},
        TextColor(Color::linear_rgb(1.0, 0.2, 0.2)),
        UiSize::px(90.0, 55.0),
        UiAlign::top(),
    ));

    // let left_container_entity=commands.spawn((
    //     UiSpan{ span: 1 },
    //     UiColor{back:Color::srgb(0.5,0.5,0.5),..Default::default()},
    //     UiGap{hgap:UiVal::Px(30.0),vgap:UiVal::Px(30.0)},
    //     UiEdge{ padding: UiRectVal::new_px(30.0), ..Default::default() },
    //     UiFill{ hfill: UiVal::None, vfill: UiVal::Scale(1.0) }
    // )).id();

    // let right_container_entity=commands.spawn((
    //     UiSpan{ span: 3 },
    //     UiColor{back:Color::srgb(0.5,0.5,0.5),..Default::default()},
    //     UiGap{hgap:UiVal::Px(30.0),vgap:UiVal::Px(30.0)},
    //     UiEdge{ padding: UiRectVal::new_px(30.0), ..Default::default() },
    // )).id();

    // commands.entity(root_entity).add_children(&[left_container_entity,right_container_entity]);

    // commands.entity(left_container_entity).with_children(|parent|{
    //     for _ in 0..2 {
    //         let entity=parent.spawn(()).id();
    //         create_ui_box(&mut parent.commands(), &mut rng, font.clone(),entity);

    //         parent.commands().entity(entity).with_children(|parent|{
    //             for _ in 0..2 {
    //                 let entity=parent.spawn(()).id();
    //                 create_ui_box(&mut parent.commands(), &mut rng, font.clone(),entity);
    //             }
    //         });
    //     }
    // });
    // commands.entity(right_container_entity).with_children(|parent|{
    //     for _ in 0..9 {
    //         let entity=parent.spawn(()).id();
    //         create_ui_box(&mut parent.commands(), &mut rng, font.clone(),entity);
    //     }
    // });

}

fn setup_camera(mut commands: Commands) {

    commands.spawn((
        CameraUi::default(),
        Msaa::Sample8,

    ));
}



#[derive(Component)]
struct FpsText;


#[derive(Component)]
struct TextAtlasMarker;

fn setup_fps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let font: Handle<Font>=asset_server.load("fonts/FiraMono-Medium.ttf");



    // commands.spawn((
    //     FpsText,
    //     UiRoot::default(),

    //     TextFont{ font: font.clone(), font_size: 15.0, ..Default::default() },
    //     TextColor(Color::WHITE),
    //     UiAlign{ halign: UiVal::Scale(1.0), valign: UiVal::Scale(0.0) },
    //     MyText2d("aaa".into()),
    //     // TextSpan(format!("aaa")),
    //     UiText{..Default::default()},

    // ));



    commands.spawn((
        TextAtlasMarker,
        UiRoot::default(),
        UiSize::scale(1.0, 1.0),
        UiColor{back:Color::linear_rgb(0.2, 0.2, 0.2),..Default::default()},
        // UiSpan{ span: 1 },

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
    // mut marker_query: Query< (Entity,&mut MyText),With<FpsText>>,
    mut text_query: Query< &mut MyText,
        // With<FpsText>
    >,
    mut marker_query2: Query< Entity,With<TextAtlasMarker>>,

    mut commands: Commands,
    mut b:Local<usize>,

    uinode_query: Query<&TextLayoutInfo>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,

) {
    for mut text in text_query.iter_mut() {
        if *b==1 {
            text.0="aba".into();
            println!("done0");
        }
        // else if *b==1 {
        //     text.0="aca".into();
        //     println!("done1");
        // }

    }

    if *b<10 {
        *b+=1;
    }

    if let Ok(root_entity)=marker_query2.single_mut() {
        let mut handles=BTreeSet::new();
        for text_layout_info in uinode_query.iter() {

            for text_glyph in text_layout_info.glyphs.iter() {

                let id=text_glyph.atlas_info.texture.clone();
                // if let Some(handle)=asset_server.get_id_handle(id)
                if let Some(handle) = images.get_strong_handle(id)
                {
                    // println!("handle {handle:?}");
                    handles.insert(handle);
                }

            }
        }

        commands.entity(root_entity).despawn_children();

        for handle in handles {
            commands.entity(root_entity).with_child((
                UiImage{handle,width_scale:0.75,height_scale:0.75, ..Default::default()},
                UiColor::default().back(Color::linear_rgb(0.4, 0.4, 0.4,),),

            ));
        }
    }
    // if let Ok((e,mut text))=marker_query.single_mut() {
    //     let v=diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
    //     let fps = v.and_then(|x|x.value()).map(|x|x.round()).unwrap_or_default();
    //     let avg = v.and_then(|x|x.average()).unwrap_or_default();
    //     // text.0 =format!("{fps:.0} {avg:.0}");


    //     // if !*b {
    //     // text.0="aza".into();
    //     // *b=true;
    //     // } else {

    //     // text.0="aza".into();
    //     // }

    //     // if !*b {

    //     // commands.entity(e).entry::<MyText2d>().or_default().and_modify(|mut c|{
    //     //     c.0="aza".into();
    //     // });
    //     // commands.queue(move|world: &mut bevy::ecs::world::World|{

    //     //     let mut e=world.entity_mut(e);

    //     //     // let mut c=e.entry::<MyText2d>().or_default();

    //     //     // c.
    //     //     // c.get_mut().0="aza".into();
    //     //     e.get_mut::<MyText2d>().unwrap().0="aza".into();
    //     // });
    //     // *b=true;
    //     // }
    //     // let q=commands.entity(e).entry::<MyText2d>().or_default();
    //     // q.

    // }
}



