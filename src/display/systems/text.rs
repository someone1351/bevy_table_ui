/*
TODO:
* add min text size, if text len is less than size, add spaces to get same size, or get largest char width, and calc max px width, and use that

* add option to set the text top_to_bottom or left_to_right

BUG
* cosmic text has a bug where if you update the text by adding or modifying a char to one that hasn't been used before, then it will render as an empty spot
** only when you enter another new char that hasn't been used before, will it render the previously added char, but will then not render the newest char

* the problem is i'm updating stuff, then immediate running queue_text which checks for component updated flag?,
** instead need to separate them into dif systems? ie things that update components, then thing that runs queue_text?
NOTE
* cosmic text seems to use same texture for all text


*/



use std::collections::HashMap;

use bevy::color::Color;
use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
// use bevy::hierarchy::prelude::*;

use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::Vec2;
// use bevy::render::texture::Image;
// use bevy::sprite::TextureAtlasLayout;
use bevy::text::{ComputedTextBlock, CosmicFontSystem, Font, FontAtlasSets, FontSmoothing, Justify, LineBreak, LineHeight, SwashCache, TextBounds, TextError, TextFont, TextLayout, TextLayoutInfo, TextPipeline, TextSpan, };
// use bevy::window::Window;


use crate::UiSize;

use super::super::super::layout::components::{UiLayoutComputed, UiInnerSize,UiRoot};

use super::super::components::*;
// use super::super::resources::*;
// use super::super::utils::*;
use super::super::values::*;

pub fn update_text_bounds(

    root_query: Query<&UiRoot,With<UiLayoutComputed>>,


    mut ui_query: Query<(Entity,
        Option<&UiSize>,
        &UiLayoutComputed,
        &mut UiInnerSize,
        Option<&mut UiText>,
        Option<&mut TextLayoutInfo>,
        Option<&mut UiTextComputed>,
        Option<&mut ComputedTextBlock>,
        &TextFont,
        Option<& TextLayout>,
        Option<& TextSpan>,
        &mut TextBounds,
        // Option<& TextColor>,
    )>,
) {

    for (entity,
        layout_size,
        &layout_computed,
        mut inner_size,
        text,
        text_layout_info,
        text_computed,
        computed_text_block,
        text_font,
        text_layout,
        text_span,
        mut text_bounds,
    ) in ui_query.iter_mut() {
        if !layout_computed.enabled {
            continue;
        }

        //
        let ui_layout_size=layout_size.cloned().unwrap_or_default();
        let text_layout=text_layout.cloned().unwrap_or_default();

        if text_layout.linebreak==LineBreak::NoWrap {
            if text_bounds.width.is_some() {
                text_bounds.width=None;
            }
        } else {

            let  bound_width=(layout_computed.size.x>=0.0).then_some(layout_computed.size.x);

            // if bound_width.is_some() {
            if text_bounds.width !=bound_width {
                text_bounds.width=bound_width;
            }
            // }
        }


        // let  bound_height=None;//(layout_computed.size.y>=0.0).then_some(layout_computed.size.y);
    }
}

pub fn update_text(
    asset_server: Res<AssetServer>,
    fonts: Res<Assets<Font>>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut font_atlas_sets: ResMut<FontAtlasSets>,
    mut text_pipeline: ResMut<TextPipeline>,

    mut ui_query: Query<(Entity,
        Option<&UiSize>,
        &UiLayoutComputed,
        &mut UiInnerSize,
        &mut UiText,
        &mut TextLayoutInfo,
        &mut UiTextComputed,
        &mut ComputedTextBlock,
        // &TextFont,
        Option<& TextLayout>,
        // Option<& TextSpan>,
        // Ref<TextSpan>,
        // Ref<MyText2d>,

        // Option<& TextColor>,
    )>,
    root_query: Query<&UiRoot,With<UiLayoutComputed>>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut swash_cache: ResMut<SwashCache>,


    mut text_reader: MyTextReader,

    // mut tests:Local<HashMap<Entity,bool>>,
) {

    // for (x,y) in texture_atlases.iter() {
    //     for t in y.textures {

    //     }

    // }
    // for x in font_atlas_sets. {

    // }

    for (entity,
        layout_size,
        &layout_computed,
        mut inner_size,
        text,
        mut text_layout_info,
        mut text_computed,
        mut computed_text_block,
        // text_font,
        text_layout,
        // text_span,
    ) in ui_query.iter_mut()
    {
        // println!("hmm {}",layout_computed.enabled);
        let layout_size=layout_size.cloned().unwrap_or_default();
        if !layout_computed.enabled {
            // println!("{entity:?} {computed:?}");
            continue;
        }

        //sometimes size was 0, and was generating no glypths with that,
        // and then later size wasn't 0, but counted as already updated,
        // showing no text

        // if layout_computed.size.x==0.0 || layout_computed.size.y==0.0 {
        //     // continue;
        // }

        let root=root_query.get(layout_computed.root_entity).unwrap();
        let scale_factor=root.scaling.max(0.0);
        let text_scale_factor=scale_factor*root.text_scaling.max(0.0);

        inner_size.width = 0.0;
        inner_size.height = 0.0;


        // let mut custom_size.width : f32 = 0.0;
        // let mut custom_size.height : f32 = 0.0;


        //text here!
        // if let (
        //     Some(mut text),
        //     Some(mut text_computed),
        //     Some(mut text_layout_info),
        //     Some(mut computed_text_block))
        //     = (text,text_computed,text_layout_info,computed_text_block )
        {
            // let mut fonts_loaded=true;
            // let handle=&text_font.font;

            // let fonts_loaded=asset_server.get_load_state(handle).map(|x|x.is_loaded()).unwrap_or_default();


            //
            if text_layout_info.scale_factor!=text_scale_factor {

            }

            //text_bounds modified (only need w)
            //font size modified ??
            //TextLayout aka align/break modified

            //

            // if asset_server.get_load_state(handle).map(|x|x.is_loaded()).unwrap_or_default() { //let Some(bevy::asset::LoadState::Loaded) =
            //     true
            // } else {
            //     // fonts_loaded=false;
            //     // println!("noo");
            //     false
            // };

            // let font_size=text_font.font_size;//*scale_factor*10.0;

            //need to check if layout_computed.size has changed if using it for text wrap?
            //  eg what if you change the size from 0 to -50
            //    can store ui_size's width/height in text_computed
            //  eg what about if image of dif size is used, and changes inner size (with ui_size being <0)?
            // let tex_updated= text.update || text_computed.scaling!=text_scale_factor
            //     || text_computed.width_used!=layout_size.width
            //     || text_computed.height_used!=layout_size.height
            //     // ||text_computed.bounds!=
            //     ;

            //update
            text_computed.width_used=layout_size.width;
            text_computed.height_used=layout_size.height;

            // let tex_updated=true;
            //

            // if computed_text_block.needs_rerender() && fonts_loaded {

            // }
            // if computed_text_block.needs_rerender() {
            //     println!("ree");
            // }
            // let was_test=tests.get(&entity).cloned().unwrap_or_default();

            if //tex_updated &&
            // // (
            // computed_text_block.needs_rerender() &&
            // // //||text_span.is_changed()
            // // ||was_test
            // // ) &&
                // fonts_loaded //|| true
                true
            {
                // println!("ree2");

                // if was_test {
                //     tests.remove(&entity);
                // } else {
                //     tests.insert(entity,true);
                // }
                let text_layout=text_layout.cloned().unwrap_or_default();

                let mut bound_width=(layout_computed.size.x>=0.0).then_some(layout_computed.size.x);
                let  bound_height=None;//(layout_computed.size.y>=0.0).then_some(layout_computed.size.y);

                let mut new_text_max_size= Vec2::ZERO;

                // let text_alignment = match text.halign {
                //     UiTextHAlign::Center => Justify::Center,
                //     UiTextHAlign::Left => Justify::Left,
                //     UiTextHAlign::Right => Justify::Right,
                // };

                //calc total widht/height for hlen/vlen
                // if
                //     // false
                //     text.hlen!=0 || text.vlen!=0
                // {
                //     let mut value = if text.hlen!=0 {
                //         " ".repeat(text.hlen as usize)
                //     } else {
                //         " ".to_string()
                //     };

                //     if text.vlen>1 {
                //         value.push_str("\n ".repeat((text.vlen-1) as usize).as_str());
                //     }

                //     let text_spans=[(entity, 0 /*depth*/,
                //         value.as_str(),
                //         &TextFont{
                //             font: text_font.font.clone(), font_size, font_smoothing: FontSmoothing::AntiAliased,
                //             line_height: LineHeight::RelativeToFont(1.2),
                //         },
                //         // text.color,
                //         Color::WHITE,
                //     )];

                //     // let mut temp_text_layout_info = TextLayoutInfo::default();

                //     if let Ok(mut measure)=text_pipeline.create_text_measure(
                //         entity, &fonts,
                //         text_spans.into_iter(),
                //         text_scale_factor.into(),
                //         &TextLayout {linebreak: LineBreak::NoWrap,..Default::default()},
                //         &mut computed_text_block,
                //         &mut font_system,
                //     ) {

                //         let size=measure.compute_size(
                //             TextBounds{width:None,height:None},
                //             &mut computed_text_block,
                //             &mut font_system);


                //         //println!("hmm {size:?}");

                //         if text.hlen!=0 {
                //             bound_width=Some(size.x);
                //             new_text_max_size.x=size.x;
                //         }

                //         if text.vlen!=0 {
                //             // bound_height=Some(size.y);
                //             new_text_max_size.y=size.y;
                //         }
                //     }

                //     // if let Ok(()) = text_pipeline.queue_text(
                //     //     &mut temp_text_layout_info,
                //     //     &fonts,
                //     //     text_spans.into_iter(),
                //     //     text_scale_factor as f64,
                //     //     // 1.0,
                //     //     &TextLayout {justify: text_alignment,linebreak: LineBreak::NoWrap,},
                //     //     TextBounds{width:None,height:None},
                //     //     &mut font_atlas_sets,
                //     //     &mut texture_atlases,
                //     //     &mut *textures,
                //     //     &mut computed_text_block,
                //     //     &mut font_system,
                //     //     &mut swash_cache,
                //     //     // YAxisOrientation::TopToBottom,
                //     // ) {
                //     //     if text.hlen!=0 {
                //     //         bound_width=Some(temp_text_layout_info.size.x);
                //     //         new_text_max_size.x=temp_text_layout_info.size.x;
                //     //     }

                //     //     if text.vlen!=0 {
                //     //         // bound_height=Some(temp_text_layout_info.size.y);
                //     //         new_text_max_size.y=temp_text_layout_info.size.y;
                //     //     }
                //     // }
                // }

                //
                {
                    // let text_spans=[(
                    //     entity, 0 /*depth*/,
                    //     // text.value.as_str(),
                    //     // text_span.map(|x|x.0.as_str()).unwrap_or_default(),
                    //     text_span.0.as_str(),
                    //     text_font,
                    //     // &TextFont{
                    //     //     font: text.font.clone(),
                    //     //     font_size,
                    //     //     font_smoothing: FontSmoothing::AntiAliased,
                    //     //     line_height: LineHeight::RelativeToFont(1.2),
                    //     // },
                    //     // text.color,
                    //     Color::WHITE,
                    // )];



                    // println!("b {bound_width:?} {bound_height:?}");

                    // match text_pipeline.update_buffer(
                    //     &fonts, text_spans.into_iter(),
                    //     LineBreak::WordBoundary,
                    //     text_alignment,
                    //     TextBounds{width:bound_width,height:bound_height},
                    //     text_scale_factor.into(), &mut computed_text_block, &mut font_system
                    // )

                    // {
                    //     Err(e @ TextError::FailedToGetGlyphImage(_)) => {
                    //         panic!("Fatal error when processing font: {}.", e);
                    //     },
                    //     Err(e @ TextError::NoSuchFont) => {
                    //         panic!("Fatal error when processing font: {}.", e);
                    //     },
                    //     Err(e @ TextError::FailedToAddGlyph(_)) => {
                    //         panic!("Fatal error when processing text: {}.", e);
                    //     },
                    //     Ok(()) => {
                    //         // println!("t {:?}",text_layout_info.size);
                    //         new_text_max_size.x=new_text_max_size.x.max(text_layout_info.size.x);
                    //         new_text_max_size.y=new_text_max_size.y.max(text_layout_info.size.y);
                    //     }
                    // };
                    // TextPipeline::update_buffer and then TextPipeline::update_text_layout.
                    match text_pipeline.queue_text(
                        &mut text_layout_info,
                        &fonts,
                        // text_spans.into_iter(),
                        text_reader.iter(entity),
                        text_scale_factor.into(),
                        // 1.0,
                        // &TextLayout {justify: text_alignment,linebreak: LineBreak::WordBoundary,},
                        &text_layout,
                        TextBounds{width:bound_width,height:bound_height},
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
                            new_text_max_size.x=new_text_max_size.x.max(text_layout_info.size.x);
                            new_text_max_size.y=new_text_max_size.y.max(text_layout_info.size.y);
                        }
                    };

                    //

                    //
                    // let x=text_pipeline.update_text_layout();

                    //
                    inner_size.width = inner_size.width.max(new_text_max_size.x);
                    inner_size.height = inner_size.height.max(new_text_max_size.y);

                    //
                    text_computed.max_size=new_text_max_size;
                    text_computed.bounds=layout_computed.size.max(new_text_max_size); //layout_computed.size before it's possibly recalculated?

                    //
                    // text.update=false;
                    text_computed.scaling=text_scale_factor;

                    // println!("text is {:?}",text.value);
                }
                // println!("goo {new_text_max_size:?}");
            } else { //whats this for again? because inner_size is cleared at top, need to reset it when not updated? what about for image?
                inner_size.width = inner_size.width.max(text_computed.max_size.x); //size
                inner_size.height = inner_size.height.max(text_computed.max_size.y); //size
            }
        }
    }
}



pub fn my_text_update_system2(
    fonts: Res<Assets<Font>>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut font_atlas_sets: ResMut<FontAtlasSets>,
    mut text_pipeline: ResMut<TextPipeline>,

    mut ui_query: Query<(Entity,
        &mut TextLayoutInfo,
        &mut ComputedTextBlock,
        Option<& TextLayout>,
        Option<& TextBounds>,

    ),With<MyText>>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut swash_cache: ResMut<SwashCache>,
    mut text_reader: MyTextReader,
) {

    for (entity,
        mut text_layout_info,
        mut computed_text_block,
        text_layout,
        text_bounds,
    ) in ui_query.iter_mut() {


        if
            // computed_text_block.needs_rerender() &&
           true

        {

            let text_layout=text_layout.cloned().unwrap_or_default();
            let text_bounds=text_bounds.cloned().unwrap_or_default();


            //
            match text_pipeline.queue_text(
                &mut text_layout_info,
                &fonts,
                text_reader.iter(entity),
                1.0,
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
                }
            };


        }
    }
}
