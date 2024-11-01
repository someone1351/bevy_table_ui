/*
TODO:
* add min text size, if text len is less than size, add spaces to get same size, or get largest char width, and calc max px width, and use that
*/


use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
// use bevy::hierarchy::prelude::*;

use bevy::math::Vec2;
use bevy::render::texture::Image;
use bevy::sprite::TextureAtlasLayout;
use bevy::text::{BreakLineOn, Font, FontAtlasSets, JustifyText, TextError, TextLayoutInfo, TextPipeline, TextSection, TextSettings, TextStyle, YAxisOrientation};
use bevy::window::Window;

use super::super::super::layout::components::{UiLayoutComputed, UiInnerSize};

use super::super::components::*;
// use super::super::resources::*;
// use super::super::utils::*;
use super::super::values::*;

pub fn update_text_image(
    mut commands: Commands,
    windows: Query<&Window>,

    asset_server: Res<AssetServer>,
    fonts: Res<Assets<Font>>,
    mut textures: ResMut<Assets<Image>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut font_atlas_sets: ResMut<FontAtlasSets>,
    mut text_pipeline: ResMut<TextPipeline>,
    text_settings: Res<TextSettings>,
    // mut font_atlas_warning:ResMut<FontAtlasWarning>,

    mut ui_query: Query<(Entity, 
        &UiLayoutComputed,
        &mut UiInnerSize,
        Option<&mut UiText>,
        Option<&mut TextLayoutInfo>,
        Option<&mut UiTextComputed>,
        Option<&UiImage>,
    )>,
) {
    // let window_size=windows.get_single().and_then(|window|Ok((window.width(),window.height()))).unwrap_or((0.0,0.0));
    let scale_factor = windows.get_single().and_then(|window|Ok(window.scale_factor())).unwrap_or(1.0);
    // println!("scale_factor={scale_factor}");
    //only on visible, updated?
    

    for (entity, 
        &computed, 
        mut inner_size, 
        text, 
        text_layout_info,
        text_max_size,
        image) in ui_query.iter_mut()
    {
        if !computed.enabled {
            // println!("{entity:?} {computed:?}");
            continue;
        }

        //sometimes size was 0, and was generating no glypths with that,
        // and then later size wasn't 0, but counted as already updated,
        // showing no text

        if computed.size.x==0.0 || computed.size.y==0.0 {
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
        if let Some(mut text) = text {
            // println!("==textupdated3");
            let mut fonts_loaded=true;

            // for section in text.sections.iter() {
            //     let handle=&section.section.style.font;

            //     if Some(bevy::asset::LoadState::Loaded) != asset_server.get_load_state(handle) {
            //         fonts_loaded=false;
            //         break;
            //     }
            // }
            
            //
            let handle=&text.font;

            if Some(bevy::asset::LoadState::Loaded) != asset_server.get_load_state(handle) {
                fonts_loaded=false;
                // break;
            }

            //
            if text.update && 
            fonts_loaded {
                // let qq:Vec<&String>=text.sections.iter().map(|x|&x.value).collect();
                
                // println!("{:?} {:?} {:?}",entity,(computed.w,computed.h),qq);
                let bounds = Vec2::new(
                    if computed.size.x<0.0 {f32::INFINITY} else {computed.size.x},
                    if computed.size.y<0.0 {f32::INFINITY} else {computed.size.y}
                );
        
                // let bounds = Vec2::new(f32::INFINITY,f32::INFINITY);

                // if text.sections.len()>0 {
                //     // text.sections[0].value=format!("{:?} ",entity)+&text.sections[0].value;
                // }

                let text_alignment = match text.halign {
                    UiTextHAlign::Center => JustifyText::Center,
                    UiTextHAlign::Left => JustifyText::Left,
                    UiTextHAlign::Right => JustifyText::Right,
                };

                //
                let mut bounds2: Vec2=bounds;
                // let mut text_layout_info2= None;//TextLayoutInfo::default();

                let mut new_text_max_size= text_max_size.as_ref().map(|x|x.max_size.clone()).unwrap_or_default();

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

                    let sections = [TextSection{
                        value: " ".repeat(text.hlen as usize),
                        style: TextStyle{
                            font: text.font.clone(),
                            font_size: text.font_size,
                            color: text.color,
                        },
                    }];

                    if let Ok(new_text_layout_info) = text_pipeline.queue_text(
                        &fonts,
                        &sections,
                        scale_factor,
                        text_alignment,
                        BreakLineOn::NoWrap,
                        Vec2::new(f32::INFINITY, f32::INFINITY),
                        &mut font_atlas_sets,
                        &mut texture_atlases,
                        &mut *textures,
                        text_settings.as_ref(),
                        YAxisOrientation::TopToBottom,
                    ) {
                        if text.hlen!=0 {
                            bounds2.x=new_text_layout_info.logical_size.x;
                        }

                        if text.vlen!=0 {
                            bounds2.y=new_text_layout_info.logical_size.y;
                        }

                        new_text_max_size=new_text_layout_info.logical_size;
                    }
                }

                //

                let sections = [TextSection{
                    value: text.value.clone(),
                    style: TextStyle{
                        font: text.font.clone(),
                        font_size: text.font_size,
                        color: text.color,
                    },
                }];

                // bounds2.x=computed.size.x;

                match text_pipeline.queue_text(
                    &fonts,
                    &sections, //text.sections,
                    scale_factor,
                    text_alignment,
                    BreakLineOn::WordBoundary,
                    bounds2,
                    &mut font_atlas_sets, // &mut *font_atlas_set_storage,
                    &mut texture_atlases,
                    &mut *textures,
                    text_settings.as_ref(),

                    // entity,
                    // &mut font_atlas_warning,
                    // YAxisOrientation::BottomToTop, //ydir
                    YAxisOrientation::TopToBottom, //ydir3
                ) {

                    // Err(e @ TextError::ExceedMaxTextAtlases(_)) => {
                    //     panic!("Fatal error when processing text: {e}.");
                    // }
                    Err(e @ TextError::NoSuchFont) => {
                        // format!("TODOFIX (tableuix::core::systems::update_text_image): Fatal error when processing font: {}.", e);
                        println!("Fatal error when processing font: {}.", e);
                        panic!("Fatal error when processing font: {}.", e);
                    },
                    Err(e @ TextError::FailedToAddGlyph(_)) => {
                        println!("Fatal error when processing text: {}.", e);
                        panic!("Fatal error when processing text: {}.", e);
                    },
                    Ok(new_text_layout_info) => {
                        
                        new_text_max_size.x=new_text_max_size.x.max(new_text_layout_info.logical_size.x);
                        new_text_max_size.y=new_text_max_size.y.max(new_text_layout_info.logical_size.y);

                        if let Some(mut text_layout_info) = text_layout_info {
                            *text_layout_info=new_text_layout_info;
                        } else {
                            commands.entity(entity).insert(new_text_layout_info);
                        }

                    }
                };

                //
                
                inner_size.width = inner_size.width.max(new_text_max_size.x); 
                inner_size.height = inner_size.height.max(new_text_max_size.y); 

                //
                if let Some(mut text_max_size) = text_max_size {
                    text_max_size.max_size=new_text_max_size;
                } else {
                    commands.entity(entity).insert(UiTextComputed{max_size:new_text_max_size});
                }

                // println!("==textupdated");
                //
                text.update=false; 
            } else {
                    
                // println!("==textupdated2");
                // if let Some(text_layout_info) = text_layout_info { //first time it is inserted this will fail
                //     inner_size.width = inner_size.width.max(text_layout_info.logical_size.x); //size
                //     inner_size.height = inner_size.height.max(text_layout_info.logical_size.y); //size
                // }
                
                if let Some(text_max_size) = text_max_size { //first time it is inserted this will fail
                    inner_size.width = inner_size.width.max(text_max_size.max_size.x); //size
                    inner_size.height = inner_size.height.max(text_max_size.max_size.y); //size
                }
            }
            // if let Some(text_layout) = text_pipeline.get_glyphs(&entity) {
            //     custom_size.width = custom_size.width.max(text_layout.size.x);
            //     custom_size.height = custom_size.height.max(text_layout.size.y);
            // }
        }

        // bla_sizes.insert(entity,Vec2::new(custom_size.width,custom_size.height));
    }
}
