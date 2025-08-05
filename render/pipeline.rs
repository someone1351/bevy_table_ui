use bevy::ecs::prelude::*;
use bevy::image::BevyDefault;
use bevy::render::renderer::RenderDevice;
use bevy::render::render_resource::*;
use bevy::render::view::ViewUniform;

#[derive(Resource,Clone)]
pub struct MyUiPipeline {
    pub view_layout: BindGroupLayout,
    pub image_layout: BindGroupLayout,
}

impl FromWorld for MyUiPipeline {
    fn from_world(world: &mut World) -> Self {
        MyUiPipeline {
             view_layout : create_view_layout(world),
             image_layout : create_image_layout(world),
        }
    }
}

impl SpecializedRenderPipeline for MyUiPipeline {
    type Key = MyUiPipelineKey;

    fn specialize(&self, _key: Self::Key) -> RenderPipelineDescriptor {
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
                shader: super::COLORED_MESH2D_SHADER_HANDLE, 
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                buffers: vec![vertex_buffer_layout],
            },
            fragment: Some(FragmentState {
                shader: super::COLORED_MESH2D_SHADER_HANDLE, 
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
                self.image_layout.clone(),
            ],
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw, 
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: PrimitiveTopology::TriangleList, 
                strip_index_format: None,
            },
            depth_stencil:None,
            multisample: MultisampleState {
                count: 1, //key.msaa_samples(), 
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
pub struct MyUiPipelineKey { }

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
                    min_binding_size: Some(ViewUniform::min_size()), // BufferSize::new(ViewUniform::std140_size_static() as u64),
                },
                count: None,
            },
        ]
    )
}


fn create_image_layout(world: &mut World) -> BindGroupLayout {
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
