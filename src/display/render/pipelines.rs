

use bevy::ecs::resource::Resource;
use bevy::ecs::world::{FromWorld, World};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::renderer::RenderDevice;
use bevy::render::view::ViewUniform;


use bevy::render::render_resource::*;


use bevy::image::BevyDefault;

use super::super::render_core::core_my::CORE_2D_DEPTH_FORMAT;

use super::shaders::MY_COLORED_MESH2D_SHADER_HANDLE;


//pipeline


#[derive(Resource,Clone)]
pub struct MyUiPipeline {
    pub view_layout: BindGroupLayout,
    pub image_layout: BindGroupLayout, //added
}

impl FromWorld for MyUiPipeline {
    fn from_world(world: &mut World) -> Self {
        MyUiPipeline {
             view_layout : create_view_layout(world),
             image_layout : create_image_layout(world), //added
        }
    }
}

impl SpecializedRenderPipeline for MyUiPipeline {
    type Key = MyUiPipelineKey;
    // type Key = SpritePipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let vertex_buffer_layout = VertexBufferLayout::from_vertex_formats(
            VertexStepMode::Vertex,
            vec![
                VertexFormat::Float32x3,// position
                VertexFormat::Float32x4,// color
                VertexFormat::Float32x2,// tex //added
            ],
        );

        RenderPipelineDescriptor {
            vertex: VertexState {
                shader: MY_COLORED_MESH2D_SHADER_HANDLE,
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                buffers: vec![vertex_buffer_layout],
            },
            fragment: Some(FragmentState {
                shader: MY_COLORED_MESH2D_SHADER_HANDLE,
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: vec![
                self.view_layout.clone(), // Bind group 0 is the view uniform
                self.image_layout.clone(), //added
            ],
            primitive: PrimitiveState {
                // front_face: FrontFace::Ccw,
                front_face: FrontFace::Cw,
                // cull_mode: Some(Face::Back),
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
            },

            depth_stencil: Some(DepthStencilState {
                format: CORE_2D_DEPTH_FORMAT,
                depth_write_enabled: false,
                depth_compare: CompareFunction::GreaterEqual,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
            }),
            // multisample: MultisampleState::default(),
            multisample: MultisampleState {
                // count: key.msaa_samples(),
                count: key.msaa_samples,
                // count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("my_colored_mesh2d_pipeline".into()),
            push_constant_ranges: Vec::new(),
            zero_initialize_workgroup_memory: false,
        }
    }
}

#[derive(PartialEq,Eq, Hash, Clone)]
pub struct MyUiPipelineKey {
    pub msaa_samples:u32,
}



fn create_view_layout(world: &mut World) -> BindGroupLayout {
    let render_device = world.resource::<RenderDevice>();

    render_device.create_bind_group_layout(
        Some("my_mesh2d_view_layout"),
        &[
            BindGroupLayoutEntry { // View
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(ViewUniform::min_size()),
                },
                count: None,
            },
        ]
    )
}


fn create_image_layout(world: &mut World) -> BindGroupLayout { //added
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
    )
}
