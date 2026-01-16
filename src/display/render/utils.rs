use bevy::render::{render_resource::{BindGroup, BindGroupEntries, PipelineCache}, renderer::RenderDevice, texture::GpuImage};

use crate::display::render::pipelines::MyUiPipeline;


pub fn create_image_bind_group(
    render_device: &RenderDevice,
    mesh2d_pipeline: &MyUiPipeline,

    pipeline_cache: &PipelineCache,
    // image_bind_groups: &mut MyUiImageBindGroups,
    // handle:Option<AssetId<Image>>,
    gpu_image:&GpuImage,
) -> BindGroup {

    // let bind_group=
    // render_device.create_bind_group(
    //     "my_ui_material_bind_group",
    //     &mesh2d_pipeline.image_layout, &[
    //         BindGroupEntry {binding: 0, resource: BindingResource::TextureView(&gpu_image.texture_view),},
    //         BindGroupEntry {binding: 1, resource: BindingResource::Sampler(&gpu_image.sampler),},
    //     ]
    // )

    render_device.create_bind_group(
        "my_ui_material_bind_group",
        &pipeline_cache.get_bind_group_layout(&mesh2d_pipeline.image_layout),
        &BindGroupEntries::sequential((
            &gpu_image.texture_view,
            &gpu_image.sampler,
        )),
    )
    // ;

    // let image_id=handle.clone();//.map(|x|x.id());
    // image_bind_groups.values.insert(image_id, bind_group);

    // bind_group
}
// fn create_image_bind_group2(
//     render_device: &RenderDevice,
//     mesh2d_pipeline: &MyUiPipeline,
//     gpu_images: &RenderAssets<GpuImage>,
//     // image_bind_groups: &mut MyUiImageBindGroups,
//     // handle:Option<AssetId<Image>>,
//     // handle:Option<AssetId<Image>>,
// ) {


//     //
//     // let image_id=handle.clone();//.map(|x|x.id());
//     // // let image_id=test.handle.id();
//     // //
//     // if image_bind_groups.values.contains_key(&image_id) {
//     //     return;
//     // }

//     // let Some(image_id)=image_id else {
//     //     return;
//     // };

//     let Some(gpu_image)=gpu_images.get(image_id) else {
//         return;
//     };

//     create_image_bind_group(render_device,mesh2d_pipeline,image_bind_groups,handle,gpu_image);
// }


use bevy::{
    image::{ Image, TextureFormatPixelInfo, },
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor, TexelCopyBufferLayout, TextureViewDescriptor},
        // renderer::RenderDevice,
        renderer::RenderQueue,
        // texture::GpuImage
    }
};

pub fn create_dummy_image(render_device: &RenderDevice, render_queue:&RenderQueue,
    // default_sampler:&DefaultImageSampler
) -> GpuImage {

    let image = Image::default();
    let texture = render_device.create_texture(&image.texture_descriptor);

    // let sampler = match image.sampler {
    //     ImageSampler::Default => (**default_sampler).clone(),
    //     ImageSampler::Descriptor(ref descriptor) => {
    //         render_device.create_sampler(&descriptor.as_wgpu())
    //     }
    // };

    let sampler = render_device.create_sampler(&SamplerDescriptor {
        min_filter: FilterMode::Nearest,
        mag_filter: FilterMode::Nearest,
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        ..Default::default()
    });

    let format_size = image.texture_descriptor.format.pixel_size().expect("TableUI, dummy image format pixel size err");

    render_queue.write_texture(
        texture.as_image_copy(),
        image.data.as_ref().expect("Image has no data"),
        TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(image.width() * format_size as u32),
            rows_per_image: None,
        },
        image.texture_descriptor.size,
    );

    let texture_view = texture.create_view(&TextureViewDescriptor::default());

    GpuImage {
        texture,
        texture_view,
        texture_format: image.texture_descriptor.format,
        sampler,
        size: image.texture_descriptor.size,
        mip_level_count: image.texture_descriptor.mip_level_count,
        texture_view_format: image.texture_view_descriptor.and_then(|v| v.format),
        had_data: false,
    }
}


/*


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


*/