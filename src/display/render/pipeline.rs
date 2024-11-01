use bevy::ecs::prelude::*;
use bevy::math::Vec2;
use bevy::render::texture::{GpuImage,Image,BevyDefault,TextureFormatPixelInfo};
use bevy::render::renderer::{RenderDevice,RenderQueue};
use bevy::render::render_resource::*;
use bevy::render::view::ViewUniform;
// use bevy::render::render_resource::std140::AsStd140;

#[derive(Resource,Clone)]
pub struct MyUiPipeline {
    pub view_layout: BindGroupLayout,
    pub image_layout: BindGroupLayout,
    // pub dummy_white_gpu_image: GpuImage,//used in place of optional textures
   
}

impl FromWorld for MyUiPipeline {
    fn from_world(world: &mut World) -> Self {
        MyUiPipeline {
             view_layout : create_view_layout(world),
             image_layout : create_image_layout(world),
            //  dummy_white_gpu_image : create_dummy_image(world),
        }
    }
}


/// A marker component for colored 2d meshes, We implement `SpecializedPipeline` to customize the default rendering from `Mesh2dPipeline`
impl SpecializedRenderPipeline for MyUiPipeline {
    type Key = MyUiPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        // Customize how to store the meshes' vertex attributes in the vertex buffer
        // Our meshes only have position and color
        // Position (GOTCHA! Vertex_Position isn't first in the buffer due to how Mesh sorts attributes (alphabetically))
        // This is the sum of the size of position and color attributes (12 + 16 = 28)


        let vertex_buffer_layout = VertexBufferLayout::from_vertex_formats(
            VertexStepMode::Vertex,
            vec![                
                VertexFormat::Float32x3,// position                
                VertexFormat::Float32x4,// color                
                VertexFormat::Float32x2,// uv
            ],
        );

        //
        // let vertex_buffer_layout = VertexBufferLayout { // Use our custom vertex buffer
        //     array_stride: 4*3 + 4*4 + 4*2,
        //     step_mode: VertexStepMode::Vertex,
        //     attributes: vec![
        //         VertexAttribute { format: VertexFormat::Float32x3, offset: 0, shader_location: 0, }, // Position 
        //         VertexAttribute { format: VertexFormat::Float32x4, offset: 12, shader_location: 1,}, // Color
        //         VertexAttribute { format: VertexFormat::Float32x2, offset: 28, shader_location: 2,}, // UV
        //     ],
        // };

        RenderPipelineDescriptor {
            vertex: VertexState {
                shader: super::COLORED_MESH2D_SHADER_HANDLE, // Use our custom shader //.typed::<Shader>()
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                buffers: vec![vertex_buffer_layout],
            },
            fragment: Some(FragmentState {
                shader: super::COLORED_MESH2D_SHADER_HANDLE, // Use our custom shader //.typed::<Shader>()
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: vec![ // Use the two standard uniforms for 2d meshes
                self.view_layout.clone(), // Bind group 0 is the view uniform
                self.image_layout.clone(),
            ],
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw, //todo fix vertex order instead of using cw //FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: PrimitiveTopology::TriangleList, //key.primitive_topology(),
                strip_index_format: None,
            },
            depth_stencil:None,
            // depth_stencil: Some(DepthStencilState { format: TextureFormat::Depth24PlusStencil8, depth_write_enabled: true, depth_compare: CompareFunction::GreaterEqual, stencil: StencilState::default(), bias: DepthBiasState::default() }),
            multisample: MultisampleState {
                count: 1, //key.msaa_samples(), //
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("my_colored_mesh2d_pipeline".into()),
            push_constant_ranges: Vec::new(),
        }
    }
}

#[derive(PartialEq,Eq, Hash, Clone)]
pub struct MyUiPipelineKey {

}
// bitflags::bitflags! {
//     #[repr(transparent)]
//     // MSAA uses the highest 6 bits for the MSAA sample count - 1 to support up to 64x MSAA. // FIXME: make normals optional?
//     pub struct MyUiPipelineKey: u32 {
//         const NONE = 0;
//         const VERTEX_TANGENTS = (1 << 0);
//         const MSAA_RESERVED_BITS = MyUiPipelineKey::MSAA_MASK_BITS << MyUiPipelineKey::MSAA_SHIFT_BITS;
//         const PRIMITIVE_TOPOLOGY_RESERVED_BITS = MyUiPipelineKey::PRIMITIVE_TOPOLOGY_MASK_BITS << MyUiPipelineKey::PRIMITIVE_TOPOLOGY_SHIFT_BITS;
//     }
// }

// impl MyUiPipelineKey {
//     const MSAA_MASK_BITS: u32 = 0b111111;
//     const MSAA_SHIFT_BITS: u32 = 32 - 6;
//     const PRIMITIVE_TOPOLOGY_MASK_BITS: u32 = 0b111;
//     const PRIMITIVE_TOPOLOGY_SHIFT_BITS: u32 = Self::MSAA_SHIFT_BITS - 3;

//     pub fn from_msaa_samples(msaa_samples: u32) -> Self {
//         let msaa_bits = ((msaa_samples - 1) & Self::MSAA_MASK_BITS) << Self::MSAA_SHIFT_BITS;
//         MyUiPipelineKey::from_bits(msaa_bits).unwrap()
//     }

//     pub fn msaa_samples(&self) -> u32 {
//         ((self.bits >> Self::MSAA_SHIFT_BITS) & Self::MSAA_MASK_BITS) + 1
//     }

//     pub fn from_primitive_topology(primitive_topology: PrimitiveTopology) -> Self {
//         let primitive_topology_bits = ((primitive_topology as u32)
//             & Self::PRIMITIVE_TOPOLOGY_MASK_BITS)
//             << Self::PRIMITIVE_TOPOLOGY_SHIFT_BITS;

//         MyUiPipelineKey::from_bits(primitive_topology_bits).unwrap()
//     }

//     pub fn primitive_topology(&self) -> PrimitiveTopology {
//         let primitive_topology_bits =
//             (self.bits >> Self::PRIMITIVE_TOPOLOGY_SHIFT_BITS) & Self::PRIMITIVE_TOPOLOGY_MASK_BITS;

//         match primitive_topology_bits {
//             x if x == PrimitiveTopology::PointList as u32 => PrimitiveTopology::PointList,
//             x if x == PrimitiveTopology::LineList as u32 => PrimitiveTopology::LineList,
//             x if x == PrimitiveTopology::LineStrip as u32 => PrimitiveTopology::LineStrip,
//             x if x == PrimitiveTopology::TriangleList as u32 => PrimitiveTopology::TriangleList,
//             x if x == PrimitiveTopology::TriangleStrip as u32 => PrimitiveTopology::TriangleStrip,
//             _ => PrimitiveTopology::default(),
//         }
//     }
// }

fn create_view_layout(world: &mut World) -> BindGroupLayout {
    // let render_device = world.get_resource::<RenderDevice>().unwrap();

    let render_device = world.resource::<RenderDevice>();

    render_device.create_bind_group_layout(
        Some("my_mesh2d_view_layout"), //?
        &[
            BindGroupLayoutEntry { // View
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(ViewUniform::min_size()), // BufferSize::new(ViewUniform::std140_size_static() as u64),
                },
                count: None,
            },
        ]
    //     &BindGroupLayoutDescriptor {
    //     entries: &[
    //         BindGroupLayoutEntry { // View
    //             binding: 0,
    //             visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
    //             ty: BindingType::Buffer {
    //                 ty: BufferBindingType::Uniform,
    //                 has_dynamic_offset: true,
    //                 min_binding_size: Some(ViewUniform::min_size()), // BufferSize::new(ViewUniform::std140_size_static() as u64),
    //             },
    //             count: None,
    //         },
    //     ],
    //     label: Some("my_mesh2d_view_layout"),
    // }
)
}

// fn create_model_layout(world: &mut World) -> BindGroupLayout {
//     let render_device = world.get_resource::<RenderDevice>().unwrap();

//     render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//         entries: &[BindGroupLayoutEntry {
//             binding: 0,
//             visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
//             ty: BindingType::Buffer {
//                 ty: BufferBindingType::Uniform,
//                 has_dynamic_offset: true,
//                 min_binding_size: BufferSize::new(super::MyModelUniform::std140_size_static() as u64),
//             },
//             count: None,
//         }],
//         label: Some("mesh2d_layout"),
//     })
// }


fn create_image_layout(world: &mut World) -> BindGroupLayout {
    // let render_device = world.get_resource::<RenderDevice>().unwrap();

    let render_device = world.resource::<RenderDevice>();

    render_device.create_bind_group_layout(
        Some("my_ui_image_layout"),
        &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
        ],
    //     &BindGroupLayoutDescriptor {
    //     entries: &[
    //         BindGroupLayoutEntry {
    //             binding: 0,
    //             visibility: ShaderStages::FRAGMENT,
    //             ty: BindingType::Texture {
    //                 multisampled: false,
    //                 sample_type: TextureSampleType::Float { filterable: true },
    //                 view_dimension: TextureViewDimension::D2,
    //             },
    //             count: None,
    //         },
    //         BindGroupLayoutEntry {
    //             binding: 1,
    //             visibility: ShaderStages::FRAGMENT,
    //             ty: BindingType::Sampler(SamplerBindingType::Filtering),
    //             count: None,
    //         },
    //     ],
    //     label: Some("my_ui_image_layout"),
    // }
    )
}
