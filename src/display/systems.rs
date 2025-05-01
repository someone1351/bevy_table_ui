/*
TODO:
* add min text size, if text len is less than size, add spaces to get same size, or get largest char width, and calc max px width, and use that
*/


use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
// use bevy::hierarchy::prelude::*;

use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::Vec2;
// use bevy::render::texture::Image;
// use bevy::sprite::TextureAtlasLayout;
use bevy::text::{ComputedTextBlock, CosmicFontSystem, Font, FontAtlasSets, FontSmoothing, JustifyText, LineBreak, LineHeight, SwashCache, TextBounds, TextError, TextFont, TextLayout, TextLayoutInfo, TextPipeline, YAxisOrientation};
use bevy::window::Window;

use super::super::layout::components::{UiLayoutComputed, UiInnerSize};

use super::components::*;
// use super::super::resources::*;
// use super::super::utils::*;
use super::values::*;

pub fn update_text_image(
    // mut commands: Commands,
    windows: Query<&Window>,

    asset_server: Res<AssetServer>,
    fonts: Res<Assets<Font>>,
    mut textures: ResMut<Assets<Image>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut font_atlas_sets: ResMut<FontAtlasSets>,
    mut text_pipeline: ResMut<TextPipeline>,
    // text_settings: Res<TextSettings>,
    // mut font_atlas_warning:ResMut<FontAtlasWarning>,

    mut ui_query: Query<(Entity,
        &UiLayoutComputed,
        &mut UiInnerSize,
        Option<&mut UiText>,
        Option<&mut TextLayoutInfo>,
        Option<&mut UiTextComputed>,
        Option<&mut ComputedTextBlock>,
        Option<&UiImage>,
    )>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut swash_cache: ResMut<SwashCache>,
) {
    // let window_size=windows.get_single().and_then(|window|Ok((window.width(),window.height()))).unwrap_or((0.0,0.0));
    //todo need to get window for camera?
    let scale_factor = windows.single().and_then(|window|Ok(window.scale_factor() as f64)).unwrap_or(1.0);
    // println!("scale_factor={scale_factor}");
    //only on visible, updated?


    for (entity,
        &layout_computed,
        mut inner_size,
        text,
        text_layout_info,
        text_computed,
        computed_text_block,
        image) in ui_query.iter_mut()
    {
        if !layout_computed.enabled {
            // println!("{entity:?} {computed:?}");
            continue;
        }

        //sometimes size was 0, and was generating no glypths with that,
        // and then later size wasn't 0, but counted as already updated,
        // showing no text

        if layout_computed.size.x==0.0 || layout_computed.size.y==0.0 {
            continue;
        }

        inner_size.width = 0.0;
        inner_size.height = 0.0;


        // let mut custom_size.width : f32 = 0.0;
        // let mut custom_size.height : f32 = 0.0;

        //image here
        if let Some(image) = image {
            if let Some(texture) = textures.get(&image.handle) {
                let size = texture.size().as_vec2();

                //todo keep aspect ratio

                if image.width_scale>0.0 {
                    inner_size.width = inner_size.width.max(image.width_scale*size.x);
                } else if inner_size.width == 0.0 {
                    inner_size.width = inner_size.width.max(size.x);
                }

                if image.height_scale>0.0 {
                    inner_size.height = inner_size.height.max(image.height_scale*size.y);
                } else if inner_size.height == 0.0 {
                    inner_size.height = inner_size.height.max(size.y);
                }
            }
        }

        //text here!
        if let (
            Some(mut text),
            Some(mut text_computed),
            Some(mut text_layout_info),
            Some(mut computed_text_block))
            = (text,text_computed,text_layout_info,computed_text_block )
        {
            let mut fonts_loaded=true;
            let handle=&text.font;

            if let Some(bevy::asset::LoadState::Loaded) = asset_server.get_load_state(handle) { } else {
                fonts_loaded=false;
            }

            //
            if text.update && fonts_loaded {
                let mut bound_width=(layout_computed.size.x>=0.0).then_some(layout_computed.size.x);
                let mut bound_height=(layout_computed.size.y>=0.0).then_some(layout_computed.size.y);

                let mut new_text_max_size= Vec2::ZERO;

                let text_alignment = match text.halign {
                    UiTextHAlign::Center => JustifyText::Center,
                    UiTextHAlign::Left => JustifyText::Left,
                    UiTextHAlign::Right => JustifyText::Right,
                };

                //
                if text.hlen!=0 || text.vlen!=0 {
                    let mut value = if text.hlen!=0 {
                        " ".repeat(text.hlen as usize)
                    } else {
                        " ".to_string()
                    };

                    if text.vlen>1 {
                        value.push_str("\n ".repeat((text.vlen-1) as usize).as_str());
                    }

                    let text_spans=[(entity, 0 /*depth*/, " ", &TextFont{
                        font: text.font.clone(), font_size: text.font_size, font_smoothing: FontSmoothing::None,
                        line_height: LineHeight::RelativeToFont(1.2),
                    },text.color)];

                    let mut temp_text_layout_info = TextLayoutInfo::default();

                    if let Ok(()) = text_pipeline.queue_text(
                        &mut temp_text_layout_info,
                        &fonts,
                        text_spans.into_iter(),
                        scale_factor,
                        &TextLayout {justify: text_alignment,linebreak: LineBreak::NoWrap,},
                        TextBounds{width:None,height:None},
                        &mut font_atlas_sets,
                        &mut texture_atlases,
                        &mut *textures,
                        YAxisOrientation::TopToBottom,
                        &mut computed_text_block,
                        &mut font_system,
                        &mut swash_cache,
                    ) {
                        if text.hlen!=0 {
                            bound_width=Some(temp_text_layout_info.size.x);
                        }

                        if text.vlen!=0 {
                            bound_height=Some(temp_text_layout_info.size.y);
                        }

                        new_text_max_size=temp_text_layout_info.size;
                    }
                }

                //
                let text_spans=[(entity, 0 /*depth*/, text.value.as_str(), &TextFont{
                    font: text.font.clone(), font_size: text.font_size, font_smoothing: FontSmoothing::AntiAliased,
                    line_height: LineHeight::RelativeToFont(1.2),
                },text.color)];


                match text_pipeline.queue_text(
                    &mut text_layout_info,
                    &fonts,
                    text_spans.into_iter(),
                    scale_factor,
                    &TextLayout {justify: text_alignment,linebreak: LineBreak::WordBoundary,},
                    TextBounds{width:bound_width,height:bound_height},
                    &mut font_atlas_sets,
                    &mut texture_atlases,
                    &mut *textures,
                    YAxisOrientation::TopToBottom,
                    &mut computed_text_block,
                    &mut font_system,
                    &mut swash_cache,

                ) {
                    Err(e @ TextError::FailedToGetGlyphImage(_)) => {
                        panic!("Fatal error when processing font: {}.", e);
                    },
                    Err(e @ TextError::NoSuchFont) => {
                        panic!("Fatal error when processing font: {}.", e);
                    },
                    Err(e @ TextError::FailedToAddGlyph(_)) => {
                        panic!("Fatal error when processing text: {}.", e);
                    },
                    Ok(()) => {
                        new_text_max_size.x=new_text_max_size.x.max(text_layout_info.size.x);
                        new_text_max_size.y=new_text_max_size.y.max(text_layout_info.size.y);
                    }
                };

                //
                inner_size.width = inner_size.width.max(new_text_max_size.x);
                inner_size.height = inner_size.height.max(new_text_max_size.y);

                //
                text_computed.max_size=new_text_max_size;
                text_computed.bounds=layout_computed.size.max(new_text_max_size); //layout_computed.size before it's possibly recalculated?

                //
                text.update=false;
            } else { //whats this for again? because inner_size is cleared at top, need to reset it when not updated? what about for image?
                inner_size.width = inner_size.width.max(text_computed.max_size.x); //size
                inner_size.height = inner_size.height.max(text_computed.max_size.y); //size
            }
        }
    }
}
