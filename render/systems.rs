
use std::collections::{HashMap, HashSet};

use bevy::asset::*;
use bevy::color::Color;
// use bevy::ecs::entity::EntityHashSet;
use bevy::ecs::prelude::*;
// use bevy::hierarchy::Parent;
use bevy::image::{BevyDefault, Image, TextureAtlasLayout, TextureFormatPixelInfo};
use bevy::math::{FloatOrd, Mat4, UVec4, Vec2};
use bevy::prelude::{Camera, Camera2d, Camera3d, GlobalTransform};
use bevy::render::sync_world::{RenderEntity, TemporaryRenderEntity};
// use bevy::sprite::TextureAtlasLayout;
use bevy::text::*;
use bevy::window::{Window,PrimaryWindow};

use bevy::render::Extract;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_phase::{DrawFunctions, PhaseItemExtraIndex, ViewSortedRenderPhases};
use bevy::render::renderer::*;
use bevy::render::texture::*;
use bevy::render::view::*;


use super::draw::*;
use super::phase::*;
use super::pipeline::*;
use super::resources::*;
use super::components::*;
use super::camera::*;

use super::super::{components::{UiColor,UiText,UiTextComputed,UiImage},values::{UiTextHAlign,UiTextVAlign}};
use super::super::super::layout::{components::*,values::UiRect};


pub fn dummy_image_setup(
    mut has_ran: Local<bool>,
    mut images: ResMut<Assets<Image>>,
    mut dummy_image: ResMut<DummyImage>,
) {

    if *has_ran { return; }
    *has_ran = true;

    //
    let image = Image::new_fill(
        Extent3d::default(),
        TextureDimension::D2,
        &[255u8; 4],
        TextureFormat::bevy_default(),
        RenderAssetUsages::RENDER_WORLD,
    );

    let handle = images.add(image);
    dummy_image.handle = handle;

    println!("dummy image inited!");
}

pub fn extract_dummy_image_setup(
    mut has_ran: Local<bool>,
    images: Extract<Res<Assets<Image>>>,
    // mut gpu_images : ResMut<RenderAssets<GpuImage>>,

    dummy_image: Extract<Res<DummyImage>>,
    render_device : Res<RenderDevice>,
    render_queue : Res<RenderQueue>,
    // default_sampler : ResMut<DefaultImageSampler>,
    mut dummy_gpu_image : ResMut<DummyGpuImage>,

) {
    //crates/bevy_pbr/src/render/mesh.rs


    if *has_ran { return; }

    let Some(image) = images.get(&dummy_image.handle) else //.unwrap(); //crashed here ...
    {
        return;
    };

    *has_ran = true;

    //
    let texture = render_device.create_texture(&image.texture_descriptor);

    let sampler = render_device.create_sampler(&SamplerDescriptor {
        min_filter: FilterMode::Nearest,
        mag_filter: FilterMode::Nearest,
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        ..Default::default()
    });

    let format_size = image.texture_descriptor.format.pixel_size();

    render_queue.write_texture(
        // ImageCopyTexture {
        //     texture: &texture,
        //     mip_level: 0,
        //     origin: Origin3d::ZERO,
        //     aspect: TextureAspect::All,
        // },
        texture.as_image_copy(),
        // &image.data,
        image.data.as_ref().expect("Image was created without data"),
        // ImageDataLayout {
        //     offset: 0,
        //     bytes_per_row: Some(image.texture_descriptor.size.width * format_size as u32),
        //     rows_per_image: None,
        // },
        TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(image.width() * format_size as u32),
            rows_per_image: None,
        },
        image.texture_descriptor.size,
    );

    let texture_view = texture.create_view(&TextureViewDescriptor::default());

    let gpu_image = GpuImage {
        texture,
        texture_view,
        texture_format: image.texture_descriptor.format,
        sampler,
        // size: bevy::math::UVec2::new(
        //     image.texture_descriptor.size.width,
        //     image.texture_descriptor.size.height,
        // ),
        // size:Extent3d { width: image.texture_descriptor.size.width, height: image.texture_descriptor.size.height, depth_or_array_layers: () },
        size:image.texture_descriptor.size,
        mip_level_count:1, //todo what
    };

    dummy_gpu_image.gpu_image = Some(gpu_image);

    println!("extract dummy image inited!");
}



pub fn extract_default_ui_camera_view(
    mut commands: Commands,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
    query: Extract<Query<(Entity,RenderEntity, &Camera), Or<(With<Camera2d>, With<Camera3d>)>>>,
    // query: Extract<Query<(Entity,&Camera), Or<(With<Camera2d>, With<Camera3d>)>>>,
    // mut live_entities: Local<EntityHashSet>,
    mut live_entities: Local<HashSet<RetainedViewEntity>>,
) {
    /// Extracts all UI elements associated with a camera into the render world.

    const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;
    const UI_CAMERA_FAR: f32 = 1000.0;

    live_entities.clear();

    // let scale = 1.0;//ui_scale.0.recip();
    for (entity, render_entity,camera) in &query {
        // let render_entity=entity;
        // ignore inactive cameras
        if !camera.is_active {
            let mut entity_commands = commands.get_entity(entity).expect("Camera entity wasn't synced.");
            entity_commands.remove::<MyCameraView>();
            continue;
        }

        // if let (Some(logical_size),Some(URect {min: physical_origin,..}), Some(physical_size),) = (
        //     camera.logical_viewport_size(),
        //     camera.physical_viewport_rect(),
        //     camera.physical_viewport_size(),
        // )
        if let Some(physical_viewport_rect) = camera.physical_viewport_rect()
        {
            // use a projection matrix with the origin in the top left instead of the bottom left that comes with OrthographicProjection
            let projection_matrix = Mat4::orthographic_rh(
                0.0,
                // logical_size.x * scale,
                // logical_size.y * scale,
                physical_viewport_rect.width() as f32,
                physical_viewport_rect.height() as f32,
                0.0,
                0.0, UI_CAMERA_FAR,
            );

            /// The ID of the subview associated with a camera on which UI is to be drawn.
            ///
            /// When UI is present, cameras extract to two views: the main 2D/3D one and a
            /// UI one. The main 2D or 3D camera gets subview 0, and the corresponding UI
            /// camera gets this subview, 1.
            const MYUI_CAMERA_SUBVIEW: u32 = 1;
            let retained_view_entity = RetainedViewEntity::new(entity.into(), None, MYUI_CAMERA_SUBVIEW);

            let default_camera_view = commands
                .spawn((ExtractedView {
                    retained_view_entity,
                    clip_from_view: projection_matrix,
                    world_from_view: GlobalTransform::from_xyz(0.0, 0.0, UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,),
                    clip_from_world: None,
                    hdr: camera.hdr,
                    // viewport: UVec4::new( physical_origin.x, physical_origin.y, physical_size.x, physical_size.y, ),
                    viewport: UVec4::from((
                        physical_viewport_rect.min,
                        physical_viewport_rect.size(),
                    )),
                    color_grading: Default::default(),
                },
                TemporaryRenderEntity,
                MyUiViewTarget(render_entity),
            )).id();

            let mut entity_commands = commands.get_entity(render_entity).expect("Camera entity wasn't synced.");
            entity_commands.insert(MyCameraView(default_camera_view));

            transparent_render_phases.insert_or_clear(retained_view_entity); //entity
            live_entities.insert(retained_view_entity); //entity
        }
    }

    transparent_render_phases.retain(|entity, _| live_entities.contains(entity));
}







pub fn extract_uinodes(
    windows: Extract<Query<&Window, With<PrimaryWindow>>>,
    mut commands: Commands,

    textures: Extract<Res<Assets<Image>>>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,

    uinode_query: Extract<Query<(
        Entity,
        &UiLayoutComputed,
        Option<&UiImage>,
        Option<&UiText>,
        Option<&UiTextComputed>,
        Option<&TextLayoutInfo>,
        Option<&ChildOf>,
        Option<&UiFloat>,
        Option<&UiEdge>,
        Option<&UiColor>,
        // Option<&MyTargetCamera>,
    )> >,

    mut extracted_elements : ResMut<MyUiExtractedElements>,

    // camera_query: Extract<Query<(Entity, &Camera)>>,
    default_ui_camera: Extract<MyDefaultUiCamera>,
    mapping: Extract<Query<RenderEntity>>,
) {

    extracted_elements.elements.clear();


    let scale_factor = windows
        .single()
        .map(|window| window.resolution.scale_factor() as f32)
        .unwrap_or(1.0);

    // let window=windows.get_single();
    // // let window_size=window.and_then(|window|Ok((window.width(),window.height()))).unwrap_or((0.0,0.0));

    let _inv_scale_factor = 1. / scale_factor;

    //camera.map(MyTargetCamera::entity).or(default_ui_camera.get())
    // let Some(camera_entity) = default_ui_camera.get()  else { return; };
    let Some(camera_entity) =
        // camera.map(TargetCamera::entity).or(
            default_ui_camera.get()
        // )
         else {return;};

    let Ok(render_camera_entity) = mapping.get(camera_entity) else {
        return;
    };

    let camera_entity=render_camera_entity;

    for (_entity,
        layout_computed,
        image,
        text,
        text_computed,
        text_layout_info,
        _parent,
        _float,
        _edge,
        color,
    ) in uinode_query.iter() {
        if !layout_computed.visible {
            continue;
        }

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

        //
        let padding_color = color.map(|c|c.padding).unwrap_or(Color::NONE);
        let margin_color = color.map(|c|c.margin).unwrap_or(Color::NONE);
        let border_color = color.map(|c|c.border).unwrap_or(Color::NONE);
        let cell_color = color.map(|c|c.cell).unwrap_or(Color::NONE);
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
                bl,br,tl,tr,
                // z,
                color: back_color.clone(),
                bl_uv : Vec2::new(0.0,1.0),
                br_uv : Vec2::new(1.0,1.0),
                tl_uv : Vec2::new(0.0,0.0),
                tr_uv : Vec2::new(1.0,0.0),
                depth,
                image:None,
                entity:commands.spawn((TemporaryRenderEntity,)).id(),
                camera_entity,
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
                                bl:bls[j],br:brs[j],tl:tls[j],tr:trs[j],
                                // z,
                                color : cols[i].clone(),
                                // ..Default::default()

                                bl_uv : Vec2::new(0.0,1.0),
                                br_uv : Vec2::new(1.0,1.0),
                                tl_uv : Vec2::new(0.0,0.0),
                                tr_uv : Vec2::new(1.0,0.0),
                                depth,
                                image:None,
                                entity:commands.spawn((TemporaryRenderEntity,)).id(),
                                camera_entity,
                            });
                        }
                    }
                }
            }
        }

        //image
        if let Some(image) = image {
            if clamped_inner_width > 0.0 && clamped_inner_height > 0.0 {
                let texture = textures.get(&image.handle);
                let size = if let Some(texture)=texture{texture.size().as_vec2()}else{Vec2::ZERO};

                //todo keep aspect ratio

                if image.keep_aspect_ratio && image.width_scale <= 0.0 && image.height_scale > 0.0 {

                }

                if image.keep_aspect_ratio && image.width_scale > 0.0 && image.height_scale <= 0.0 {

                }

                if image.keep_aspect_ratio && image.width_scale <= 0.0 && image.height_scale <= 0.0 {

                }

                let w = if image.width_scale > 0.0 {
                    image.width_scale*size.x
                } else {
                    layout_computed.size.x
                };

                let h = if image.height_scale > 0.0 {
                    image.height_scale*size.y
                } else {
                    layout_computed.size.y
                };

                let dx=clamped_inner_width/w;
                let dy=clamped_inner_height/h;

                let bl=Vec2::new(clamped_inner_rect.left, clamped_inner_rect.bottom);
                let br=Vec2::new(clamped_inner_rect.right, clamped_inner_rect.bottom);
                let tl=Vec2::new(clamped_inner_rect.left, clamped_inner_rect.top);
                let tr=Vec2::new(clamped_inner_rect.right, clamped_inner_rect.top);

                let bl_uv=Vec2::new(0.0, dy);
                let br_uv=Vec2::new(dx, dy);
                let tl_uv=Vec2::new(0.0, 0.0);
                let tr_uv=Vec2::new(dx, 0.0);

                extracted_elements.elements.push(MyUiExtractedElement{
                    bl,br,tl,tr,
                    // z:image_z,
                    color : image.color.clone(),
                    bl_uv,br_uv,tl_uv,tr_uv,
                    depth:image_depth,
                    image:Some(image.handle.clone()),

                    entity:commands.spawn((TemporaryRenderEntity,)).id(),
                    camera_entity,
                });
            }
        }

        //text
        if let (Some(text), Some(text_layout),Some(text_computed) ) = (text, text_layout_info,text_computed,
            // text_pipeline.get_glyphs(&entity)
        ) {
            let glyph_offset=text_computed.bounds-text_layout.size; //only needed for x, since because bevy now handles halign positioning

            for text_glyph in text_layout.glyphs.iter() {
                let color = text.color;
                let atlas = texture_atlases.get(&text_glyph.atlas_info.texture_atlas).unwrap();
                let glyph_index = text_glyph.atlas_info.location.glyph_index as usize;
                let atlas_glyph_rect = atlas.textures[glyph_index].as_rect();

                let glyph_size=atlas_glyph_rect.max-atlas_glyph_rect.min;
                let mut glyph_pos=layout_computed.pos + text_glyph.position - glyph_offset - glyph_size*0.5;

                let atlas_size=atlas.size.as_vec2();

                if text_layout.size.x<=layout_computed.size.x {
                    glyph_pos.x+=match text.halign {
                        UiTextHAlign::Right => layout_computed.size.x-text_layout.size.x,
                        UiTextHAlign::Center => (layout_computed.size.x-text_layout.size.x)*0.5,
                        UiTextHAlign::Left => 0.0
                    };
                }


                if text_layout.size.y<=layout_computed.size.y {
                    glyph_pos.y+=match text.valign {
                        UiTextVAlign::Top => 0.0,
                        UiTextVAlign::Center => (layout_computed.size.y-text_layout.size.y)*0.5,
                        UiTextVAlign::Bottom => layout_computed.size.y-text_layout.size.y
                    }; //ydir
                }

                //
                let glyph_pos2=glyph_pos+glyph_size;

                let glyph_rect=UiRect { left: glyph_pos.x, right: glyph_pos2.x, top: glyph_pos.y, bottom: glyph_pos2.y };

                //something wrong with vertical tex coords
                if clamped_inner_rect.intersects(&glyph_rect) {
                    let dx = (clamped_inner_rect.left-glyph_rect.left).max(0.0);
                    let dx2 = (glyph_rect.right-clamped_inner_rect.right).max(0.0);

                    let dy =  (clamped_inner_rect.top-glyph_rect.top).max(0.0);
                    let dy2 = (glyph_rect.bottom-clamped_inner_rect.bottom).max(0.0);

                    let x=glyph_rect.left.max(clamped_inner_rect.left);
                    let y=glyph_rect.top.max(clamped_inner_rect.top);
                    let x2=glyph_rect.right.min(clamped_inner_rect.right);
                    let y2=glyph_rect.bottom.min(clamped_inner_rect.bottom);

                    let tx = (atlas_glyph_rect.min.x+dx)/atlas_size.x;
                    let tx2 = (atlas_glyph_rect.max.x-dx2)/atlas_size.x;

                    let ty = (atlas_glyph_rect.min.y+dy)/atlas_size.y;
                    let ty2 = (atlas_glyph_rect.max.y-dy2)/atlas_size.y;

                    let tl=Vec2::new(x,y);
                    let tr=Vec2::new(x2,y);
                    let bl=Vec2::new(x,y2);
                    let br=Vec2::new(x2,y2);

                    let tl_uv=Vec2::new(tx, ty);
                    let tr_uv=Vec2::new(tx2,ty);
                    let bl_uv=Vec2::new(tx,  ty2);
                    let br_uv=Vec2::new(tx2,  ty2);

                    let texture=text_glyph.atlas_info.texture.clone();

                    extracted_elements.elements.push(MyUiExtractedElement{
                        bl,br,tl,tr,
                        // z:text_z,
                        bl_uv,br_uv,tl_uv,tr_uv,
                        color : color.clone(),
                        depth:text_depth,
                        image:Some(texture.clone()),
                        entity:commands.spawn((TemporaryRenderEntity,)).id(),
                        camera_entity,
                    });
                }
            }
        }
    }


}

pub fn queue_uinodes(
    transparent_draw_functions: Res<DrawFunctions<MyTransparentUi>>,

    colored_mesh2d_pipeline: Res<MyUiPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<MyUiPipeline>>,
    pipeline_cache: Res<PipelineCache>,

    extracted_elements : Res<MyUiExtractedElements>,
    mut views: Query<(Entity, &ExtractedView)>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
) {

    let draw_colored_mesh2d = transparent_draw_functions.read().get_id::<DrawMesh>().unwrap();
    let pipeline = pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline,MyUiPipelineKey{});

    // Iterate each view (a camera is a view)
    // for (extracted_view, mut transparent_phase) in extracted_views.iter_mut()
    // {
    // transparent_phase.items.reserve(extracted_elements.elements.len());

    for element in extracted_elements.elements.iter() {

        // println!("{} : {}",
        //     element.camera_entity,
        //     views.iter().map(|x|x.0.to_string()).collect::<Vec<_>>().join(", "));

        let Ok((_view_entity, view)) = views.get_mut(element.camera_entity) else {
            continue;
        };
        // println!("sss {view_entity}");

        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity) else {
            continue;
        };

        // println!("ttt");


        transparent_phase.add(MyTransparentUi {
            entity: element.entity, //*entity,
            draw_function: draw_colored_mesh2d,
            pipeline,
            // sort_key: FloatOrd(0.0),//FloatOrd(*depth as f32),
            sort_key: FloatOrd(element.depth as f32),
            // sort_key: FloatOrd(*depth as f32),
            // sort_key: (FloatOrd(*depth as f32),entity.index(),),
            // This material is not batched
            batch_range: 0..1,//0..c,//c..c+(elements.len()*6) as u32,
            // dynamic_offset:None,
            extra_index: PhaseItemExtraIndex::None,
            indexed: false,

        });

    // }
    }
}

pub fn prepare_uinodes(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,

    extracted_elements : Res<MyUiExtractedElements>,
    mut ui_meta: ResMut<MyUiMeta>,

    mesh2d_pipeline: Res<MyUiPipeline>,

    view_uniforms: Res<ViewUniforms>,
    extracted_views: Query<Entity, With<ExtractedView>>,

    mut image_bind_groups: ResMut<MyUiImageBindGroups>,
    image_asset_events: Res<bevy::sprite::SpriteAssetEvents>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    dummy_gpu_image : Res<DummyGpuImage>,
) {


    if let Some(view_binding) = view_uniforms.uniforms.binding() {
        for entity in extracted_views.iter() {
            let view_bind_group = render_device.create_bind_group(
                "my_mesh2d_view_bind_group",&mesh2d_pipeline.view_layout,&[BindGroupEntry {
                    binding: 0,
                    resource: view_binding.clone(),
                }],);

            commands.entity(entity).insert(MyViewBindGroup { value: view_bind_group, });
        }
    }

    for event in &image_asset_events.images {
        match event {
            AssetEvent::Removed { id } | AssetEvent::Modified { id } => {
                image_bind_groups.values.remove(&Some(id.clone()));//.unwrap();
            }
            _ =>{}
        }
    }

    //
    if dummy_gpu_image.gpu_image.is_none() { return; }
    let dummy_gpu_image = dummy_gpu_image.gpu_image.as_ref().unwrap();

    //
    // for (depth,image_handle) in extracted_elements.elements.keys()
    for element in extracted_elements.elements.iter()
    {
        let image_id=element.image.clone().map(|x|x.id());

        //
        if !image_bind_groups.values.contains_key(&image_id) {
            let gpu_image=image_id.map(|image_id|gpu_images.get(image_id)).unwrap_or(Some(dummy_gpu_image));
            let bind_group=gpu_image.map(|gpu_image|render_device.create_bind_group(
                "my_ui_material_bind_group",
                &mesh2d_pipeline.image_layout, &[
                    BindGroupEntry {binding: 0, resource: BindingResource::TextureView(&gpu_image.texture_view),},
                    BindGroupEntry {binding: 1, resource: BindingResource::Sampler(&gpu_image.sampler),},
                ]
            ));

            if let Some(bind_group)=bind_group {
                image_bind_groups.values.insert(image_id, bind_group);
            }
        }
    }

    //
    ui_meta.vertices.clear();

    //
    let mut batches = HashMap::<Entity,MyUiBatch>::new();

    for element in extracted_elements.elements.iter() {
        let z= element.depth as f32; // let z= z*-1.0;

        let mut batch = MyUiBatch {
            image_handle:element.image.clone(), //image_handle.clone(),
            // z,
            range :0..0,
            // range: ui_meta.vertices.len() as u32 ..ui_meta.vertices.len() as u32,
        };

        batch.range.start=ui_meta.vertices.len() as u32;

        // for element in elements.iter()
        {
            // let z=element.z;

            let v_pos = vec![
                [element.bl.x,element.bl.y,z], [element.br.x,element.br.y,z], [element.tl.x,element.tl.y,z],
                [element.tl.x,element.tl.y,z], [element.br.x,element.br.y,z], [element.tr.x,element.tr.y,z],
            ];

            let v_tex = vec![
                element.bl_uv.to_array(), element.br_uv.to_array(), element.tl_uv.to_array(),
                element.tl_uv.to_array(), element.br_uv.to_array(), element.tr_uv.to_array(),
            ];

            //
            for i in 0..6 {
                let c=element.color.to_linear();
                let a=[c.red,c.green,c.blue,c.alpha];
                ui_meta.vertices.push(MyUiVertex {
                    position: v_pos[i],
                    uv: v_tex[i],
                    color : a,//element.color.as_rgba_f32(),
                });
            }

            batch.range.end=ui_meta.vertices.len() as u32;
        }


        batches.insert(element.entity,batch);

        // commands.entity(*entity).insert(batch);
    }


    for (entity, batch) in batches.iter() {
        // println!("g {entity:?} {batch:?}");
        commands.entity(*entity).insert(batch.clone());
    }

    // commands.spawn(()).insert(batch.clone());

    ui_meta.vertices.write_buffer(&render_device, &render_queue);
}
