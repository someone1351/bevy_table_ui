
use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;

use bevy::image::Image;


use super::super::super::layout::components::{UiLayoutComputed, UiInnerSize,UiRoot};

use super::super::components::*;
// use super::super::resources::*;
// use super::super::utils::*;
// use super::super::values::*;

pub fn update_image(
    textures: Res<Assets<Image>>,

    mut ui_query: Query<(Entity,
        &UiLayoutComputed,
        &mut UiInnerSize,
        Option<&UiImage>,
    )>,
    root_query: Query<&UiRoot,With<UiLayoutComputed>>,
) {
    for (_entity,
        &layout_computed,
        mut inner_size,
        image) in ui_query.iter_mut()
    {
        if !layout_computed.enabled {
            continue;
        }

        //sometimes size was 0, and was generating no glypths with that,
        // and then later size wasn't 0, but counted as already updated,
        // showing no text

        if layout_computed.size.x==0.0 || layout_computed.size.y==0.0 {
            continue;
        }

        let root_entity=root_query.get(layout_computed.root_entity).unwrap();
        let scale_factor=root_entity.scaling.max(0.0);

        inner_size.width = 0.0;
        inner_size.height = 0.0;


        // let mut custom_size.width : f32 = 0.0;
        // let mut custom_size.height : f32 = 0.0;

        //image here
        if let Some(image) = image {
            if let Some(texture) = textures.get(&image.handle) {
                let image_size = texture.size().as_vec2();

                //todo keep aspect ratio
                let scale_factor=if image.use_scaling {scale_factor}else{1.0};
                // if image.width_scale>0.0 {
                    inner_size.width = inner_size.width.max(image.width_scale.max(0.0)*image_size.x*scale_factor);
                // } else if inner_size.width == 0.0 {
                //     inner_size.width = inner_size.width.max(image_size.x*scale_factor);
                // }

                // if image.height_scale>0.0 {
                    inner_size.height = inner_size.height.max(image.height_scale.max(0.0)*image_size.y*scale_factor);
                // } else if inner_size.height == 0.0 {
                //     inner_size.height = inner_size.height.max(image_size.y*scale_factor);
                // }
            }
        }

    } //end for
}

