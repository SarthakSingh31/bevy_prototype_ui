use bevy::{
    ecs::prelude::*,
    render2::{render_resource::*, renderer::RenderDevice, shader::Shader, texture::BevyDefault},
};

use crate::render::ExtractedContainer;

pub struct UiShaders {
    pub pipeline: RenderPipeline,
    pub view_layout: BindGroupLayout,
}

impl FromWorld for UiShaders {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let shader = Shader::from_wgsl(include_str!("ui.wgsl"));
        let shader_module = render_device.create_shader_module(&shader);

        let view_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    // TODO: fix this when bevy does
                    min_binding_size: BufferSize::new(144),
                },
                count: None,
            }],
            label: Some("ui_view_layout"),
        });

        let pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ui_pipeline"),
            push_constant_ranges: &[],
            bind_group_layouts: &[&view_layout],
        });

        let pipeline = render_device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("ui_pipeline"),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.,
                    clamp: 0.,
                },
            }),
            vertex: VertexState {
                buffers: &[ExtractedContainer::desc()],
                module: &shader_module,
                entry_point: "vertex",
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fragment",
                targets: &[ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::One,
                            operation: BlendOperation::Add,
                        },
                    }),
                    write_mask: ColorWrite::ALL,
                }],
            }),
            layout: Some(&pipeline_layout),
            multisample: MultisampleState::default(),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None, // TODO: change this to back?
                polygon_mode: PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
        });

        UiShaders {
            pipeline,
            view_layout,
        }
    }
}
