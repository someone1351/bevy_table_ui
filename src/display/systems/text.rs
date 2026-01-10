/*
TODO:
* add min text size, if text len is less than size, add spaces to get same size, or get largest char width, and calc max px width, and use that

* add option to set the text top_to_bottom or left_to_right


*/



use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
// use bevy::hierarchy::prelude::*;

use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::Vec2;
// use bevy::render::texture::Image;
// use bevy::sprite::TextureAtlasLayout;
use bevy::text::{ComputedTextBlock, CosmicFontSystem, Font, FontAtlasSets, Justify, LineBreak, SwashCache, TextBounds, TextError, TextLayout, TextLayoutInfo, TextPipeline, };
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
        Option<&UiSize>,
        // &mut UiInnerSize,
        &mut UiTextComputed,

        Option<& TextLayout>,
        Option<& TextBounds>,
        &mut TextLayoutInfo,
        &mut ComputedTextBlock,
    ),(
        With<UiText>,
    )>,

    fonts: Res<Assets<Font>>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut font_atlas_sets: ResMut<FontAtlasSets>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut swash_cache: ResMut<SwashCache>,
    mut text_reader: UiTextReader,
) {

    //
    // let mut entities=ui_query.iter().filter_map(|(entity,ui_layout_computed,..)|{
    //     ui_layout_computed.enabled.then_some(entity)
    // }).collect::<Vec<_>>();

    let mut entities=ui_query.iter().map(|(entity,..)|entity).collect::<Vec<_>>();

    entities.sort_by(|&a_entity,&b_entity|{
        let (_,UiLayoutComputed{  order:a_order,..},..)=ui_query.get(a_entity).unwrap();
        let (_,UiLayoutComputed{  order:b_order,..},..)=ui_query.get(b_entity).unwrap();
        a_order.cmp(b_order)
    });

    //
    for entity in entities {
        let (
            _,
            mut ui_layout_computed,
            ui_size,

            // mut ui_inner_size,
            mut ui_text_computed,

            text_layout,
            text_bounds,

            mut text_layout_info,
            mut computed_text_block,
        )=ui_query.get_mut(entity).unwrap();

        //
        let ui_root=root_query.get(ui_layout_computed.root_entity).unwrap();
        let scale_factor=ui_root.scaling.max(0.0)*ui_root.text_scaling.max(0.0);

        //
        let ui_size=ui_size.cloned().unwrap_or_default();
        let mut text_layout=text_layout.cloned().unwrap_or(TextLayout { justify: Justify::Center, linebreak: LineBreak::NoWrap });
        let mut text_bounds=text_bounds.cloned().unwrap_or_default();

        //scale bounds
        text_bounds.width=text_bounds.width.map(|x|x*ui_root.scaling);
        // text_bounds.height=text_bounds.height.map(|x|x*ui_root.scaling);
        text_bounds.height=None; //don't use ...

        //if text_bounds is user set, limit it to layout_computed size
        // text_bounds.width=text_bounds.width.map(|x|x.min(ui_layout_computed.size.x));
        // // text_bounds.width=text_bounds.height.map(|x|x.min(ui_layout_computed.size.y));

        //
        if ui_size.width.is_none() {
            text_layout.linebreak=LineBreak::NoWrap;
        }

        //
        if text_layout.linebreak==LineBreak::NoWrap {
            text_bounds.width=None;
        } else {
            text_bounds.width=Some(text_bounds.width.unwrap_or_default().max(ui_layout_computed.size.x));
        }

        //
        // if !ui_layout_computed.visible { }

        //

        if
            computed_text_block.needs_rerender()
            || ui_text_computed.scaling!=scale_factor
            // true

        {
            //
            ui_text_computed.scaling=scale_factor;


            //
            match text_pipeline.queue_text(
                &mut text_layout_info,
                &fonts,
                text_reader.iter(entity),
                scale_factor.into(),
                &text_layout,
                text_bounds,
                &mut font_atlas_sets,
                &mut texture_atlases,
                &mut *textures,
                &mut computed_text_block,
                &mut font_system,
                &mut swash_cache,
                // YAxisOrientation::TopToBottom,

            )
            {
                Err(e @ TextError::FailedToGetGlyphImage(_)) => {
                    panic!("Fatal error when processing font: {}.", e);
                },
                Err(e @ TextError::NoSuchFont) => {
                    // panic!("Fatal error when processing font: {}.", e);
                    println!("Fatal error when processing font: {}.", e);
                },
                Err(e @ TextError::FailedToAddGlyph(_)) => {
                    panic!("Fatal error when processing text: {}.", e);
                },
                Ok(()) => {
                    // println!("t {:?}",text_layout_info.size);

                    //only needed for width, since bevy handles horizontal text alignment
                    //  so text width text_layout_info.size.x is <= text_bounds.width
                    let text_bounds=Vec2::new(text_bounds.width.unwrap_or_default(),text_bounds.height.unwrap_or_default());

                    ui_text_computed.bounds=text_bounds.max(text_layout_info.size);
                    // ui_text_computed.bounds=ui_layout_computed.size.max(text_layout_info.size);
                    // ui_layout_computed.custom_size=ui_layout_computed.custom_size.max(text_layout_info.size);
                    //println!("hm");
                }
            };


        }

        // let w=text_bounds.width.unwrap_or_default().max(ui_text_computed.bounds.x);

        // .unwrap_or(ui_text_computed.bounds.x).max(ui_text_computed.bounds.x);
        // .max(text_bounds.unwrap_or_default())
        ui_layout_computed.custom_size=ui_layout_computed.custom_size.max(ui_text_computed.bounds);
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