/*
TODO:
* add min text size, if text len is less than size, add spaces to get same size, or get largest char width, and calc max px width, and use that

* add option to set the text top_to_bottom or left_to_right

BEVY
* queue_text errors need to report the font
*/



// use bevy::color::Color;
use bevy::ecs::entity::EntityHashSet;
use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
// use bevy::hierarchy::prelude::*;

use bevy::image::{Image, TextureAtlasLayout};
// use bevy::render::texture::Image;
// use bevy::sprite::TextureAtlasLayout;
use bevy::text::{ComputedTextBlock, CosmicFontSystem, Font, FontAtlasSet, FontHinting, Justify, LineBreak, SwashCache, TextBounds, TextError, TextFont, TextLayout, TextLayoutInfo, TextPipeline };
// use bevy::window::Window;



use super::super::super::layout::components::{UiLayoutComputed, UiRoot,UiSize};

use super::super::components::*;
// use super::super::resources::*;
// use super::super::utils::*;
// use super::super::values::*;

pub fn update_text_bounds(

    root_query: Query<&UiRoot,With<UiLayoutComputed>>,


    mut ui_query: Query<(
        Entity,

        &mut UiLayoutComputed,
        // Option<&UiSize>,
        Option<Ref<UiSize>>,
        &mut UiTextComputed,

        // Option<& TextLayout>,
        Option<Ref<TextLayout>>,
        // Option<& TextBounds>,
        Option<Ref<TextBounds>>,


        &mut TextLayoutInfo,
        &mut ComputedTextBlock,
        Ref<FontHinting>,
    ),(
        With<UiText>,
    )>,

    // text_bounds_changed_query:Query<(),(Changed<TextBounds>,With<UiText>)>,
    // text_layout_changed_query:Query<(),(Changed<TextLayout>,With<UiText>)>,
    // ui_size_changed_query:Query<(),(Changed<UiSize>,With<UiText>)>,

    fonts: Res<Assets<Font>>,
    text_font_query: Query<&TextFont>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    // mut font_atlas_sets: ResMut<FontAtlasSets>,
    mut font_atlas_set: ResMut<FontAtlasSet>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut swash_cache: ResMut<SwashCache>,
    mut text_reader: UiTextReader,


    mut reprocess_queue: Local<EntityHashSet>,
    // aaa:Query<Ref<UiText>>,
) {
//    for x in aaa.iter() {
//         if x.is_changed() {
//             println!("b '{}'",x.0);
//         }
//     }

    //
    let reprocess_queue_old=reprocess_queue.clone();
    reprocess_queue.clear();

    for (
        entity,
        mut ui_layout_computed,
        ui_size,

        // mut ui_inner_size,
        mut ui_text_computed,

        text_layout,
        text_bounds,

        mut text_layout_info,
        mut computed_text_block,
        hinting,
    ) in ui_query.iter_mut() {
        // println!("{entity}");
        if ui_text_computed.err {
            continue;
        }
        //
        // text_bounds_query
        // text_layout_query
        // ui_size_queryQuery


        //
        let ui_root=root_query.get(ui_layout_computed.root_entity).unwrap();
        let scale_factor=ui_root.scaling.max(0.0)*ui_root.text_scaling.max(0.0);

        //
        let ui_size2=ui_size.as_deref().cloned().unwrap_or_default();
        let mut text_layout2=text_layout.as_deref().cloned().unwrap_or(TextLayout { justify: Justify::Center, linebreak: LineBreak::NoWrap });
        let mut text_bounds2=text_bounds.as_deref().cloned().unwrap_or_default();

        //scale bounds
        text_bounds2.width=text_bounds2.width.map(|x|x*ui_root.scaling);
        // text_bounds.height=text_bounds.height.map(|x|x*ui_root.scaling);
        text_bounds2.height=None; //don't use ...

        //if text_bounds is user set, limit it to layout_computed size
        text_bounds2.width=text_bounds2.width.map(|x|x.min(ui_layout_computed.size.x));
        // // text_bounds.width=text_bounds.height.map(|x|x.min(ui_layout_computed.size.y));

        //
        if ui_size2.width.is_pos() {
            text_bounds2.width=Some(text_bounds2.width.unwrap_or_default().min(ui_layout_computed.size.x));
        } else {
            text_layout2.linebreak=LineBreak::NoWrap;
        }

        //what if bounds go from some to none, with wrap
        //  store wrap in text computed? what about justify?
        //

        if reprocess_queue_old.contains(&entity)
            || computed_text_block.needs_rerender()
            || ui_size.map(|x|x.is_changed()).unwrap_or_default()
            || text_bounds.map(|x|x.is_changed()).unwrap_or_default()
            || text_layout.map(|x|x.is_changed()).unwrap_or_default()

            || ui_text_computed.scaling!=scale_factor
            || ui_text_computed.layout_computed_size!=ui_layout_computed.size

            // || (text_bounds.width.is_some()
            //     && text_bounds.width.unwrap()>0.0
            //     && text_bounds.width.unwrap()!=ui_text_computed.size.x)

            // || true

        {
            println!("go");
            // println!("is {} {} {}: {:?} {}",
            //     computed_text_block.needs_rerender(),
            //     ui_text_computed.scaling!=scale_factor,
            //     text_bounds.width!= Some(ui_text_computed.bounds.x),
            //     text_bounds.width,ui_text_computed.bounds.x,
            // );
            //

            // println!("T");
            ui_text_computed.scaling=scale_factor;
            ui_text_computed.layout_computed_size=ui_layout_computed.size;

            // ui_text_computed.bounds_none=(text_bounds.width.is_none(),text_bounds.height.is_none());



            //
            // match text_pipeline.queue_text(
            //     &mut text_layout_info,
            //     &fonts,
            //     text_reader.iter(entity),
            //     scale_factor.into(),
            //     &text_layout,
            //     text_bounds,
            //     &mut font_atlas_sets,
            //     &mut texture_atlases,
            //     &mut *textures,
            //     &mut computed_text_block,
            //     &mut font_system,
            //     &mut swash_cache,
            //     // YAxisOrientation::TopToBottom,

            // )
            // let text_spans=[
            //     (entity, 0, "abc", text_font_query.get(entity).unwrap(), Color::WHITE, LineHeight::default())
            // ];
            // let text_spans2=text_reader.iter(entity).collect::<Vec<_>>();
            // println!("text_spans2 {text_spans2:?}");
            match text_pipeline.update_buffer(
                &fonts,
                text_reader.iter(entity),
                // text_spans.into_iter(),
                text_layout2.linebreak,
                text_layout2.justify,
                text_bounds2,
                scale_factor.into(),
                &mut computed_text_block,
                &mut font_system,
                *hinting,
            )
            {
                Err(TextError::NoSuchFont) => {
                    // There was an error processing the text layout.
                    // Add this entity to the queue and reprocess it in the following frame
                    reprocess_queue.insert(entity);
                    // println!("no such font");
                    continue;
                }
                Err(
                    e @ (TextError::FailedToAddGlyph(_)
                    | TextError::FailedToGetGlyphImage(_)
                    | TextError::MissingAtlasLayout
                    | TextError::MissingAtlasTexture
                    | TextError::InconsistentAtlasState),
                ) => {
                    eprintln!("Fatal error when processing text: {e}.");
                    ui_text_computed.err=true;
                    continue;
                }
                Ok(()) => {
                    // println!("ok1");

                }

                // Ok(()) => {

                // }
            };


        }

        // println!("xxx");


        //
        match text_pipeline.update_text_layout_info(
            &mut text_layout_info,
            text_font_query,
            scale_factor as f64,
            &mut font_atlas_set,
            &mut texture_atlases,
            &mut textures,
            &mut computed_text_block,
            &mut font_system,
            &mut swash_cache,
            text_bounds2,
            text_layout2.justify,
        ) {
            Err(TextError::NoSuchFont) => {
                // There was an error processing the text layout.
                // Add this entity to the queue and reprocess it in the following frame.
                reprocess_queue.insert(entity);
                // println!("no such font");
                continue;
            }
            Err(
                e @ (TextError::FailedToAddGlyph(_)
                | TextError::FailedToGetGlyphImage(_)
                | TextError::MissingAtlasLayout
                | TextError::MissingAtlasTexture
                | TextError::InconsistentAtlasState),
            ) => {
                eprintln!("Fatal error when processing text: {e}.");
                ui_text_computed.err=true;
                continue;
            }
            Ok(()) => {
                // println!("ok2");
                // text_layout_info.scale_factor = scale_factor;
                // text_layout_info.size *= scale_factor.recip();

                 // println!("t {:?}",text_layout_info.size);

                    //only needed for width, since bevy handles horizontal text alignment
                    //  so text width text_layout_info.size.x is <= text_bounds.width
                    // let text_bounds=Vec2::new(text_bounds.width.unwrap_or_default(),text_bounds.height.unwrap_or_default());

                    if let Some(text_bounds_width)=text_bounds2.width {
                        ui_text_computed.calc_size.x=text_bounds_width.max(text_layout_info.size.x);
                    } else {
                        ui_text_computed.calc_size.x=text_layout_info.size.x;
                    }

                    if let Some(text_bounds_height)=text_bounds2.height {
                        ui_text_computed.calc_size.y=text_bounds_height.max(text_layout_info.size.y);
                    } else {
                        ui_text_computed.calc_size.y=text_layout_info.size.y;
                    }
                    // ui_text_computed.bounds=text_bounds.max(text_layout_info.size); //only stored to be used below and above
                    // // ui_text_computed.bounds=ui_layout_computed.size.max(text_layout_info.size);
                    // // ui_layout_computed.custom_size=ui_layout_computed.custom_size.max(text_layout_info.size);
                    // //println!("hm");
            }
        }

        // println!("yyy");
        // let w=text_bounds.width.unwrap_or_default().max(ui_text_computed.bounds.x);

        // .unwrap_or(ui_text_computed.bounds.x).max(ui_text_computed.bounds.x);
        // .max(text_bounds.unwrap_or_default())

        //not sure if needed with update_text_layout_info
        ui_layout_computed.custom_size=ui_layout_computed.custom_size.max(ui_text_computed.calc_size);
    }

    //


}

    // if let Ok(mut measure)=text_pipeline.create_text_measure(
    //     entity, &fonts,
    //     text_spans.into_iter(),
    //     text_scale_factor.into(),
    //     &TextLayout {linebreak: LineBreak::NoWrap,..Default::default()},
    //     &mut computed_text_block,
    //     &mut font_system,
    // ) {

    //     let size=measure.compute_size(TextBounds{width:None,height:None}, &mut computed_text_block, &mut font_system);

    //     if text.hlen!=0 {
    //         bound_width=Some(size.x);
    //         new_text_max_size.x=size.x;
    //     }

    //     if text.vlen!=0 {
    //         // bound_height=Some(size.y);
    //         new_text_max_size.y=size.y;
    //     }
    // }