use bevy::{
    image::{
        Image,
        // ImageSampler,
        TextureFormatPixelInfo,
    },
    render::{render_resource::{AddressMode, FilterMode, SamplerDescriptor, TexelCopyBufferLayout, TextureViewDescriptor}, renderer::{RenderDevice, RenderQueue}, texture::GpuImage}
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

    let format_size = image.texture_descriptor.format.pixel_size();

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