
// use bevy::ecs::prelude::*;
use std::collections::HashMap;



use bevy::asset::{AssetEvent, AssetId, Assets, };
use bevy::camera::visibility::RenderLayers;
use bevy::color::Color;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::{FloatOrd, Vec2};
// use bevy::platform::collections::HashSet;
use bevy::prelude::{ MessageReader, Msaa};
use bevy::render::render_asset::RenderAssets;
use bevy::render::texture::GpuImage;
use bevy::render::{render_phase::*, Extract};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::sync_world::{MainEntity, TemporaryRenderEntity};
use bevy::render::view::{ExtractedView, ViewUniforms};
use bevy::ecs::system::*;


use bevy::render::render_resource::*;
use bevy::text::{ComputedTextBlock, Justify, TextBackgroundColor, TextColor, TextLayout, TextLayoutInfo};


use super::draws::DrawMesh;
use super::dummy_image::create_dummy_image;
use super::pipelines::*;
use super::components::*;
use super::resources::*;

use super::super::render_core::core_my::TransparentMy;
use super::super::TestRenderComponent;

use super::super::{components::{UiColor,UiText,UiTextComputed,UiImage},values::UiTextVAlign};
use super::super::super::layout::{components::*,values::UiRect};
//systems

fn create_image_bind_group(
    render_device: &RenderDevice,
    mesh2d_pipeline: &MyUiPipeline,
    image_bind_groups: &mut MyUiImageBindGroups,
    handle:Option<AssetId<Image>>,
    gpu_image:&GpuImage,
) {

    let bind_group=render_device.create_bind_group(
        "my_ui_material_bind_group",
        &mesh2d_pipeline.image_layout, &[
            BindGroupEntry {binding: 0, resource: BindingResource::TextureView(&gpu_image.texture_view),},
            BindGroupEntry {binding: 1, resource: BindingResource::Sampler(&gpu_image.sampler),},
        ]
    );

    let image_id=handle.clone();//.map(|x|x.id());
    image_bind_groups.values.insert(image_id, bind_group);
}
fn create_image_bind_group2(
    render_device: &RenderDevice,
    mesh2d_pipeline: &MyUiPipeline,
    gpu_images: &RenderAssets<GpuImage>,
    image_bind_groups: &mut MyUiImageBindGroups,
    handle:Option<AssetId<Image>>,
) {


    //
    let image_id=handle.clone();//.map(|x|x.id());
    // let image_id=test.handle.id();
    //
    if image_bind_groups.values.contains_key(&image_id) {
        return;
    }

    let Some(image_id)=image_id else {
        return;
    };

    let Some(gpu_image)=gpu_images.get(image_id) else {
        return;
    };

    create_image_bind_group(render_device,mesh2d_pipeline,image_bind_groups,handle,gpu_image);
}

pub fn dummy_image_setup(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mesh2d_pipeline: Res<MyUiPipeline>,
    mut image_bind_groups: ResMut<MyUiImageBindGroups>,
    mut init:Local<bool>,
) {

    if *init {
        return;
    }

    *init=true;


    let gpu_image=create_dummy_image(&render_device,&render_queue);
    create_image_bind_group(&render_device,&mesh2d_pipeline,&mut image_bind_groups,None,&gpu_image);

}





pub fn extract_images(
    // mut commands: Commands,
    uinode_query: Extract<Query<(
        Entity,
        &TestRenderComponent,
    )> >,
    mut image_asset_events: Extract<MessageReader<AssetEvent<Image>>>,

    render_device: Res<RenderDevice>,
    mesh2d_pipeline: Res<MyUiPipeline>,
    mut image_bind_groups: ResMut<MyUiImageBindGroups>,
    gpu_images: Res<RenderAssets<GpuImage>>,
) {

    for event in image_asset_events.read()
    {
        match event {
            AssetEvent::Removed { id } | AssetEvent::Modified { id } => {
                image_bind_groups.values.remove(&Some(id.clone()));//.unwrap();
            }
            _ =>{}
        }
    }

    for (_entity, test,  ) in uinode_query.iter() {
        if test.handle.is_some() {
            let handle=test.handle.clone().map(|h|h.id());
            create_image_bind_group2(&render_device,&mesh2d_pipeline,&gpu_images,&mut image_bind_groups,handle);
        }
    }
}


pub fn extract_images2(
    // mut commands: Commands,
    uinode_query: Extract<Query<(
        &UiLayoutComputed,
        Option<&UiImage>,
        Option<&TextLayoutInfo>,
        // Option<&MyTargetCamera>,
    )> >,
    mut image_asset_events: Extract<MessageReader<AssetEvent<Image>>>,

    render_device: Res<RenderDevice>,
    mesh2d_pipeline: Res<MyUiPipeline>,
    mut image_bind_groups: ResMut<MyUiImageBindGroups>,
    gpu_images: Res<RenderAssets<GpuImage>>,
) {

    for event in image_asset_events.read()
    {
        match event {
            AssetEvent::Removed { id } | AssetEvent::Modified { id } => {
                image_bind_groups.values.remove(&Some(id.clone()));//.unwrap();
            }
            _ =>{}
        }
    }


    for (
        layout_computed,
        image,
        text_layout_info,
    ) in uinode_query.iter() {
        if !layout_computed.visible {
            continue;
        }

        //image
        if image.is_some() {
            let handle=image.map(|x|x.handle.clone()).map(|h|h.id());

            create_image_bind_group2(&render_device,&mesh2d_pipeline,&gpu_images,&mut image_bind_groups,handle);
        }

        //text
        if let Some(text_layout) =  text_layout_info
            // text_pipeline.get_glyphs(&entity)
         {
            for text_glyph in text_layout.glyphs.iter() {

                let handle=Some(text_glyph.atlas_info.texture.clone());
                // let handle: Option<bevy::prelude::Handle<Image>>=Some(handle);

                create_image_bind_group2(&render_device,&mesh2d_pipeline,&gpu_images,&mut image_bind_groups,handle);

            }
        }
    }

}

pub fn extract_uinodes(
    mut commands: Commands,
    uinode_query: Extract<Query<(
        Entity,
        &TestRenderComponent,
        Option<&RenderLayers>,
    )> >,
    mut extracted_elements : ResMut<MyUiExtractedElements>,
    // default_ui_camera: Extract<MyDefaultUiCamera>,
    // cameras: Extract<Query<(RenderEntity, &MyCameraView), With<CameraTest>, >>,
    // mapping: Extract<Query<RenderEntity>>,
) {

    extracted_elements.elements.clear();


    // let Some(camera_entity) = default_ui_camera.get() else {return;};

    // let Ok(render_camera_entity) = mapping.get(camera_entity) else { return; };

    // let camera_entity=render_camera_entity;

    for (entity, test, render_layers, ) in uinode_query.iter() {
        let x= test.x;
        let y= test.y;
        let x2= test.x+test.w;
        let y2= test.y+test.h;

        let render_layers=render_layers.cloned().unwrap_or_else(||RenderLayers::layer(0));

        extracted_elements.elements.push(MyUiExtractedElement{
            entity:commands.spawn((TemporaryRenderEntity,)).id(), //is this needed? instead spawn entity later?
            main_entity:entity.into(),
            // camera_entity,
            // x: test.x,
            // y: test.y,
            // x2: test.x+test.w,
            // y2: test.y+test.h,
            color: test.col,
            depth: 0,
            render_layers,
            image: test.handle.clone().map(|h|h.id()),
            bl: Vec2::new(x, y2),
            br: Vec2::new(x2, y2),
            tl: Vec2::new(x, y),
            tr: Vec2::new(x2, y),
            ..Default::default()
        });
    }
}



pub fn extract_uinodes2(
    // windows: Extract<Query<&Window, With<PrimaryWindow>>>,
    mut commands: Commands,

    // textures: Extract<Res<Assets<Image>>>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,

    // camera_query : Extract<Query<(Entity,Option<&RenderLayers>),With<Camera>>>,

    // uinode_render_layer_query: Extract<Query<&RenderLayers, With<UiLayoutComputed>> >,
    root_render_layer_query: Extract<Query<&RenderLayers, (With<UiLayoutComputed>,With<UiRoot>)> >,
    uinode_query: Extract<Query<(
        Entity,
        &UiLayoutComputed,
        Option<&UiImage>,
        Option<&UiText>,
        Option<&UiTextComputed>,

        Option<&TextColor>,
        Option<&TextLayout>,
        Option<&TextLayoutInfo>,
        Option<&ComputedTextBlock>,

        // Option<&ChildOf>,
        // Option<&UiEdge>,
        Option<&UiColor>,
        Option<&UiFloat>,
        // Option<&MyTargetCamera>,
    )> >,

    text_colors: Extract<Query<&TextColor>>,
    text_background_colors_query: Extract<Query<&TextBackgroundColor>>,
    mut extracted_elements : ResMut<MyUiExtractedElements>,

    // camera_query: Extract<Query<(Entity, &Camera)>>,
    // default_ui_camera: Extract<MyDefaultUiCamera>,
    // mapping: Extract<Query<RenderEntity>>,
) {

    // extracted_elements.elements.clear();


    // for (entity, test, render_layers, ) in uinode_query.iter() {

    //     extracted_elements.elements.push(MyUiExtractedElement{
    //         entity:commands.spawn((TemporaryRenderEntity,)).id(), //is this needed? instead spawn entity later?
    //         main_entity:entity.into(),
    //         // camera_entity,
    //         x: test.x,
    //         y: test.y,
    //         x2: test.x+test.w,
    //         y2: test.y+test.h,
    //         color: test.col,
    //         depth: 0,
    //         render_layers: render_layers.cloned(),
    //         image: test.handle.clone(),
    //     });
    // }

    let default_render_layers = RenderLayers::layer(0);

    for (entity,
        layout_computed,
        image,
        text,
        text_computed,
        text_color,
        text_layout,
        text_layout_info,
        computed_text_block,
        // _parent,
        // _edge,
        color,
        float,
    ) in uinode_query.iter() {
        if !layout_computed.visible {
            continue;
        }

        // let mut used_cameras = HashSet::new();
        let node_render_layers=root_render_layer_query.get(layout_computed.root_entity).unwrap_or(&default_render_layers);

        // for (camera_entity,camera_render_layers) in camera_query.iter() {
        //     let camera_render_layers=camera_render_layers.unwrap_or(&default_render_layers);

        //     if camera_render_layers.intersects(node_render_layers) {
        //         used_cameras.insert(camera_entity);
        //     }
        // }

        // if used_cameras.is_empty() {
        //     continue;
        // }


        let depth = layout_computed.order*3;
        let image_depth=depth+1;
        let text_depth=depth+2;

        let clamped_inner_rect = layout_computed.clamped_rect;
        let clamped_padding_rect = layout_computed.clamped_padding_rect();
        let clamped_border_rect = layout_computed.clamped_border_rect();
        let clamped_margin_rect = layout_computed.clamped_margin_rect();
        let clamped_cell_rect = layout_computed.clamped_cell_rect;

        let clamped_inner_width = layout_computed.clamped_rect.width();
        let clamped_inner_height = layout_computed.clamped_rect.height();

        let inner_rect=layout_computed.inner_rect();
        let is_float=float.map(|x|x.float).unwrap_or_default(); //no cell col for floating


        //
        let padding_color = color.map(|c|c.padding).unwrap_or(Color::NONE);
        let margin_color = color.map(|c|c.margin).unwrap_or(Color::NONE);
        let border_color = color.map(|c|c.border).unwrap_or(Color::NONE);

        // let cell_color = if is_float {Color::NONE} else {color.map(|c|c.cell).unwrap_or(Color::NONE)};
        let cell_color=(!is_float).then_some(()).and_then(|_|color.map(|c|c.cell)).unwrap_or(Color::NONE);

        let back_color = color.map(|c|c.back).unwrap_or(Color::NONE);

        //
        let clamped_inner_rect2=if padding_color.to_srgba().alpha==0.0 {clamped_padding_rect}else{clamped_inner_rect};

        //color
        if back_color.to_srgba().alpha!=0.0 && clamped_inner_rect2.width()>0.0 && clamped_inner_rect2.height()>0.0 {
            let bl=Vec2::new(clamped_inner_rect2.left, clamped_inner_rect2.bottom);
            let br=Vec2::new(clamped_inner_rect2.right,  clamped_inner_rect2.bottom);
            let tl=Vec2::new(clamped_inner_rect2.left, clamped_inner_rect2.top);
            let tr=Vec2::new(clamped_inner_rect2.right, clamped_inner_rect2.top);

            extracted_elements.elements.push(MyUiExtractedElement{
                render_layers: node_render_layers.clone(),
                bl,br,tl,tr,
                // z,
                color: back_color.clone(),
                // bl_uv : Vec2::new(0.0,1.0),
                // br_uv : Vec2::new(1.0,1.0),
                // tl_uv : Vec2::new(0.0,0.0),
                // tr_uv : Vec2::new(1.0,0.0),
                depth,
                image:None,
                entity:commands.spawn((TemporaryRenderEntity,)).id(),

                main_entity: entity.into(),
                // camera_entity,

                ..Default::default()
            });

            // println!("entity {_entity:?} {back_color:?} {clamped_inner_rect2:?} ");
        }

        //edges
        {
            let cols = [padding_color,border_color,margin_color,cell_color];
            let sizes=[layout_computed.padding_size,layout_computed.border_size,layout_computed.margin_size,layout_computed.cell_size];
            let rects1 = [clamped_inner_rect,clamped_padding_rect,clamped_border_rect,clamped_margin_rect];
            let rects2 = [clamped_padding_rect,clamped_border_rect,clamped_margin_rect,clamped_cell_rect];

            for i in 0..4 {
                if cols[i].to_srgba().alpha!=0.0 && !sizes[i].is_zero() {

                    let inner_rect=rects1[i];
                    let outer_rect=rects2[i];

                    let sizes=[
                        outer_rect.bottom-outer_rect.top,
                        outer_rect.bottom-outer_rect.top,
                        outer_rect.right-outer_rect.left,
                        outer_rect.right-outer_rect.left,
                    ];

                    let thicknesses = [
                        inner_rect.left-outer_rect.left,
                        outer_rect.right-inner_rect.right,
                        inner_rect.top-outer_rect.top,
                        outer_rect.bottom-inner_rect.bottom,
                    ];

                    let bls=[outer_rect.left_bottom(),inner_rect.right_bottom(),inner_rect.left_top(),outer_rect.left_bottom()];
                    let tls=[outer_rect.left_top(),inner_rect.right_top(),outer_rect.left_top(),inner_rect.left_bottom()];
                    let brs=[inner_rect.left_bottom(),outer_rect.right_bottom(),inner_rect.right_top(),outer_rect.right_bottom()];
                    let trs=[inner_rect.left_top(),outer_rect.right_top(),outer_rect.right_top(),inner_rect.right_bottom()];

                    for j in 0..4 {
                        if sizes[j]>0.0 && thicknesses[j]>0.0 {
                            extracted_elements.elements.push(MyUiExtractedElement{
                                render_layers: node_render_layers.clone(),
                                bl:bls[j],br:brs[j],tl:tls[j],tr:trs[j],
                                // z,
                                color : cols[i].clone(),
                                // ..Default::default()

                                // bl_uv : Vec2::new(0.0,1.0),
                                // br_uv : Vec2::new(1.0,1.0),
                                // tl_uv : Vec2::new(0.0,0.0),
                                // tr_uv : Vec2::new(1.0,0.0),
                                depth,
                                // image:None,
                                entity:commands.spawn((TemporaryRenderEntity,)).id(),
                                // camera_entity,
                                main_entity: entity.into(),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }

        //image
        if let Some(image) = image {
            if clamped_inner_width > 0.0 && clamped_inner_height > 0.0 {
                // let texture = textures.get(&image.handle);
                // let tex_size = if let Some(texture)=texture{texture.size().as_vec2()}else{Vec2::ZERO};

                //todo keep aspect ratio

                // if image.keep_aspect_ratio && image.width_scale <= 0.0 && image.height_scale > 0.0 {

                // }

                // if image.keep_aspect_ratio && image.width_scale > 0.0 && image.height_scale <= 0.0 {

                // }

                // if image.keep_aspect_ratio && image.width_scale <= 0.0 && image.height_scale <= 0.0 {

                // }

                // let w = if image.width_scale > 0.0 {
                //     image.width_scale*tex_size.x
                // } else {
                //     layout_computed.size.x
                // };

                // let h = if image.height_scale > 0.0 {
                //     image.height_scale*tex_size.y
                // } else {
                //     layout_computed.size.y
                // };

                // let w=image.width_scale.max(0.0)*tex_size.x;
                // let h=image.height_scale.max(0.0)*tex_size.y;

                // let dx=clamped_inner_width/w;
                // let dy=clamped_inner_height/h;

                let bl=Vec2::new(clamped_inner_rect.left, clamped_inner_rect.bottom);
                let br=Vec2::new(clamped_inner_rect.right, clamped_inner_rect.bottom);
                let tl=Vec2::new(clamped_inner_rect.left, clamped_inner_rect.top);
                let tr=Vec2::new(clamped_inner_rect.right, clamped_inner_rect.top);

                let inner_width=inner_rect.width();
                let inner_height=inner_rect.height();
                // let tex_x=(clamped_inner_rect.left-inner_rect.left);
                // let tex_x2=inner_rect.right-clamped_inner_rect.right;

                // let tex_y=clamped_inner_rect.top-inner_rect.top;
                // let tex_y2=inner_rect.bottom-clamped_inner_rect.bottom;
                let tex_x=(clamped_inner_rect.left-inner_rect.left)/inner_width;
                let tex_x2=(clamped_inner_rect.right-inner_rect.left)/inner_width;

                let tex_y=(clamped_inner_rect.top-inner_rect.top)/inner_height;
                let tex_y2=(clamped_inner_rect.bottom-inner_rect.top)/inner_height;



                let bl_uv=Vec2::new(tex_x, tex_y2);
                let br_uv=Vec2::new(tex_x2, tex_y2);
                let tl_uv=Vec2::new(tex_x, tex_y);
                let tr_uv=Vec2::new(tex_x2, tex_y);
                // let bl_uv=Vec2::new(0.0, dy);
                // let br_uv=Vec2::new(dx, dy);
                // let tl_uv=Vec2::new(0.0, 0.0);
                // let tr_uv=Vec2::new(dx, 0.0);

                extracted_elements.elements.push(MyUiExtractedElement{
                    render_layers: node_render_layers.clone(),
                    bl,br,tl,tr,
                    // z:image_z,
                    color : image.color.clone(),
                    bl_uv,br_uv,tl_uv,tr_uv,
                    depth:image_depth,
                    image:Some(image.handle.clone().id()),

                    entity:commands.spawn((TemporaryRenderEntity,)).id(),
                    // camera_entity,
                    main_entity: entity.into(),
                    // ..Default::default()
                });
            }
        }

        //text
        if let (
            Some(text),
            Some(text_layout_info),
            Some(text_computed),
            Some(computed_text_block),
        ) = (
            text, text_layout_info,text_computed,computed_text_block
        ) {
            let text_layout=text_layout.cloned().unwrap_or_default();

            // let glyph_offset=text_computed.bounds-text_layout_info.size; //only needed for x, since because bevy now handles halign positioning
            // println!("hmm comp={:?} text={:?}, dif={:?},tcomp={:?}",
            //     layout_computed.size,
            //     text_layout_info.size,
            //     layout_computed.size-text_layout_info.size,
            //     text_computed.bounds
            // );
            // for (
            //     i,
            //     PositionedGlyph {
            //         position,
            //         atlas_info,
            //         span_index,
            //         ..
            //     },
            // ) in text_layout_info.glyphs.iter().enumerate()
            // {

            // }
            // for text_entity in computed_text_block.entities() {
            //     text_entity.
            // }
            // let span_entities=computed_text_block
            //     .entities()
            //     .get(*span_index)
            //     .map(|t| t.entity)
            //     .unwrap_or(Entity::PLACEHOLDER);

            let mut color = Color::WHITE;
            let mut current_span: Option<usize> = None;
            for text_glyph in text_layout_info.glyphs.iter() {
                if Some(text_glyph.span_index)!=current_span {
                    let x=computed_text_block.entities().get(text_glyph.span_index)
                        .map(|t| t.entity)
                        .unwrap_or(Entity::PLACEHOLDER);

                    color = text_colors.get(x)
                        .map(|text_color| Color::from(text_color.0))
                        .unwrap_or_default();
                    current_span = Some(text_glyph.span_index);
                }

                // let color = text.color;
                // let color = text_color.cloned().unwrap_or_default().0;
                // let color = Color::WHITE;
                let atlas = texture_atlases.get(text_glyph.atlas_info.texture_atlas).unwrap(); // .clone() //necessary?
                let glyph_index = text_glyph.atlas_info.location.glyph_index;
                // text_glyph.
                let atlas_glyph_rect = atlas.textures[glyph_index].as_rect();
                let glyph_size=atlas_glyph_rect.size();

                let mut glyph_pos=layout_computed.pos + text_glyph.position - glyph_size*0.5;//  - glyph_offset;

                let atlas_size=atlas.size.as_vec2();

                if text_computed.bounds.x<=layout_computed.size.x
                {
                    glyph_pos.x+=match text_layout.justify{
                        Justify::Left => 0.0,
                        Justify::Center|Justify::Justified => (layout_computed.size.x-text_computed.bounds.x)*0.5,
                        Justify::Right => layout_computed.size.x-text_computed.bounds.x,
                    };



                }


                // if text_computed.bounds.y<=layout_computed.size.y {
                //     glyph_pos.y+=match text.valign {
                //         UiTextVAlign::Top => 0.0,
                //         UiTextVAlign::Center => (layout_computed.size.y-text_computed.bounds.y)*0.5,
                //         UiTextVAlign::Bottom => layout_computed.size.y-text_computed.bounds.y,
                //     }; //ydir
                // }

                if text_layout_info.size.y<=layout_computed.size.y {
                    glyph_pos.y+=match text.valign {
                        UiTextVAlign::Top => 0.0,
                        UiTextVAlign::Center => (layout_computed.size.y-text_layout_info.size.y)*0.5,
                        UiTextVAlign::Bottom => layout_computed.size.y-text_layout_info.size.y,
                    }; //ydir
                }

                //
                let glyph_pos2=glyph_pos+glyph_size;

                let glyph_rect=UiRect { left: glyph_pos.x, right: glyph_pos2.x, top: glyph_pos.y, bottom: glyph_pos2.y };

                //something wrong with vertical tex coords
                // if clamped_inner_rect.intersects(&glyph_rect)
                {
                    //clamped
                    let d1=Vec2::new(clamped_inner_rect.left-glyph_rect.left,clamped_inner_rect.top-glyph_rect.top).max(Vec2::ZERO);
                    let d2=Vec2::new(glyph_rect.right-clamped_inner_rect.right,glyph_rect.bottom-clamped_inner_rect.bottom).max(Vec2::ZERO);
                    let p1=Vec2::new(glyph_rect.left.max(clamped_inner_rect.left),glyph_rect.top.max(clamped_inner_rect.top));
                    let p2=Vec2::new(glyph_rect.right.min(clamped_inner_rect.right),glyph_rect.bottom.min(clamped_inner_rect.bottom));

                    //unclamped
                    // let d1=Vec2::ZERO;
                    // let d2=Vec2::ZERO;
                    // let p1=Vec2::new(glyph_rect.left,glyph_rect.top);
                    // let p2=Vec2::new(glyph_rect.right,glyph_rect.bottom);

                    //
                    let t1=(atlas_glyph_rect.min+d1)/atlas_size;
                    let t2 = (atlas_glyph_rect.max-d2)/atlas_size;

                    let tl=p1;
                    let tr=Vec2::new(p2.x,p1.y);
                    let bl=Vec2::new(p1.x,p2.y);
                    let br=p2;

                    let tl_uv=t1;
                    let tr_uv=Vec2::new(t2.x,t1.y);
                    let bl_uv=Vec2::new(t1.x, t2.y);
                    let br_uv=t2;

                    let texture=text_glyph.atlas_info.texture.clone();

                    extracted_elements.elements.push(MyUiExtractedElement{
                        render_layers: node_render_layers.clone(),
                        bl,br,tl,tr,
                        // z:text_z,
                        bl_uv,br_uv,tl_uv,tr_uv,
                        color : color.clone(),
                        // color:Color::linear_rgb(1.0, 0.0, 0.0),
                        // depth:text_depth*100,
                        depth:text_depth,
                        image:Some(texture.clone()),
                        // image:None,
                        entity:commands.spawn((TemporaryRenderEntity,)).id(),
                        // camera_entity,
                        main_entity: entity.into(),
                    });
                }
            }
        }
    }

    // extracted_elements.max_depth=depth;

}

//MainTransparentPass2dNode
pub fn queue_uinodes(
    // transparent_draw_functions: Res<DrawFunctions<MyTransparentUi>>,
    transparent_draw_functions: Res<DrawFunctions<TransparentMy>>,

    colored_mesh2d_pipeline: Res<MyUiPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<MyUiPipeline>>,
    pipeline_cache: Res<PipelineCache>,

    extracted_elements : Res<MyUiExtractedElements>,
    views: Query<(
        // Entity, &ExtractedView
        &MainEntity,
        &ExtractedView,
        &Msaa,
        Option<&RenderLayers>, //
    )>,

    // // render_camera_query: Query<(Entity, &MyCameraView),  >,

    // mut render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentMy>>,
) {

    let draw_colored_mesh2d = transparent_draw_functions.read().get_id::<DrawMesh>().unwrap();

    // Iterate each view (a camera is a view)

    // let Ok((view_entity, _view)) = views.get_mut(element.camera_entity) else {
    //     continue;
    // };
    // for (view_entity,_view) in views.iter()
    // for (camera_render_entity,_camera_view) in render_camera_query.iter()

    let default_render_layers = RenderLayers::layer(0);

    for (
        //_view_entiy
        _main_entity,
        extracted_view,
        msaa,
        view_render_layers,
    ) in views.iter() {
        let Some(transparent_phase) = render_phases.get_mut(&extracted_view.retained_view_entity) else {
            //skip transparent phases that aren't for my camera
            continue;
        };
        let view_render_layers=view_render_layers.unwrap_or(&default_render_layers);


        // if let Some(render_layers)=render_layers {
        //     for x in render_layers.iter() {

        //     }

        // }

        let pipeline = pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline,MyUiPipelineKey{ msaa_samples: msaa.samples() });

        for element in extracted_elements.elements.iter() {

            // let element_render_layers=element.render_layers;//.unwrap_or(&default_render_layers);

            if element.render_layers.intersection(view_render_layers).iter().count()==0 {
                continue;
            }

            transparent_phase.add(TransparentMy {
                entity: (element.entity,element.main_entity), //what is it used for?
                draw_function: draw_colored_mesh2d,
                pipeline,
                sort_key: FloatOrd(element.depth as f32),
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::None,
            });
            // println!("camera_render_entity1 {:?}",extracted_view.retained_view_entity);
        }

    }
}


pub fn prepare_views(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mesh2d_pipeline: Res<MyUiPipeline>,
    view_uniforms: Res<ViewUniforms>,
    extracted_views: Query<Entity, With<ExtractedView>>,
) {
    if let Some(view_binding) = view_uniforms.uniforms.binding() {
        for view_entity in extracted_views.iter() {
            let view_bind_group = render_device.create_bind_group(
                "my_mesh2d_view_bind_group",&mesh2d_pipeline.view_layout,&[BindGroupEntry {
                    binding: 0,
                    resource: view_binding.clone(),
                }],);

            commands.entity(view_entity).insert(MyViewBindGroup { value: view_bind_group, });
        }
    }
}



pub fn prepare_uinodes(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    extracted_elements : Res<MyUiExtractedElements>,
    mut ui_meta: ResMut<MyUiMeta>,
) {

    let mut max_depth=0;


    for element in extracted_elements.elements.iter() {
        max_depth=max_depth.max(element.depth);
    }

    //
    ui_meta.vertices.clear();

    //
    let mut batches = HashMap::<Entity,MyUiBatch>::new();

    for element in extracted_elements.elements.iter() {
        let mut batch = MyUiBatch { range :0..0, image_handle: element.image.clone() };
        batch.range.start=ui_meta.vertices.len() as u32;

        // let pos = vec![
        //     [element.x,element.y2,0.0], [element.x2,element.y2,0.0], [element.x,element.y,0.0],
        //     [element.x,element.y,0.0], [element.x2,element.y2,0.0],[element.x2,element.y,0.0],
        // ];

        // let tex=vec![
        //     [0.0,1.0],[1.0,1.0],[0.0,0.0],
        //     [0.0,0.0],[1.0,1.0],[1.0,0.0],
        // ];
        let z= ((element.depth+1) as f32)/((max_depth+2) as f32) -1.0; // let z= z*-1.0;
        // let z=z+0.01;
        // println!("z is {z},  d={} f={}", element.depth,(element.depth as f32)/(max_depth as f32));
        let pos = vec![
            [element.bl.x,element.bl.y,z], [element.br.x,element.br.y,z], [element.tl.x,element.tl.y,z],
            [element.tl.x,element.tl.y,z], [element.br.x,element.br.y,z], [element.tr.x,element.tr.y,z],
        ];

        let tex = vec![
            element.bl_uv.to_array(), element.br_uv.to_array(), element.tl_uv.to_array(),
            element.tl_uv.to_array(), element.br_uv.to_array(), element.tr_uv.to_array(),
        ];

        let col=element.color.to_linear();
        for i in 0..6 {
            ui_meta.vertices.push(MyUiVertex {
                position: pos[i],
                color : [col.red,col.green,col.blue,col.alpha],
                uv: tex[i],

            });
        }

        batch.range.end=ui_meta.vertices.len() as u32;
        batches.insert(element.entity,batch);
    }


    for (entity, batch) in batches.iter() {
        commands.entity(*entity).insert(batch.clone());
        // commands.spawn(batch.clone());
    }

    ui_meta.vertices.write_buffer(&render_device, &render_queue);
}


