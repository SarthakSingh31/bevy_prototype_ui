use std::{collections::VecDeque, mem};

use bevy::{
    app::prelude::*,
    core_pipeline,
    ecs::prelude::*,
    math::prelude::*,
    render2::{
        camera::ActiveCameras,
        color,
        render_graph::{RenderGraph, SlotInfo, SlotType},
        render_phase::{sort_phase_system, DrawFunctions, Drawable, RenderPhase},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupEntry, BufferAddress, BufferUsage, BufferVec,
            VertexAttribute, VertexFormat,
        },
        renderer::RenderDevice,
        view::ViewMeta,
        RenderApp, RenderStage,
    },
    transform::prelude::*,
};

use crate::UI_Z_STEP;

use super::dom;

pub mod camera;
pub mod draw;
pub mod node;
pub mod shader;

pub const CAMERA_UI: &str = "camera_ui_2";

mod draw_ui_graph {
    pub const NAME: &str = "draw_ui_graph";

    pub mod input {
        pub const VIEW_ENTITY: &str = "ui_view_entity";
        pub const RENDER_TARGET: &str = "ui_render_target";
        pub const DEPTH: &str = "ui_depth";
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct ExtractedStyles {
    background_color: Vec4,
    size: Vec2,
    margin: Vec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct ExtractedContainer {
    transform: Mat4,
    styles: ExtractedStyles,
}

impl ExtractedContainer {
    pub fn attributes() -> &'static [VertexAttribute] {
        &[
            // transform col 1
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 0,
                shader_location: 0,
            },
            // transform col 2
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: mem::size_of::<[f32; 4]>() as BufferAddress,
                shader_location: 1,
            },
            // transform col 3
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: (mem::size_of::<[f32; 4]>() * 2) as BufferAddress,
                shader_location: 2,
            },
            // transform col 4
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: (mem::size_of::<[f32; 4]>() * 3) as BufferAddress,
                shader_location: 3,
            },
            // background_color
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: (mem::size_of::<[f32; 4]>() * 4) as BufferAddress,
                shader_location: 4,
            },
            // size
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: (mem::size_of::<[f32; 4]>() * 5) as BufferAddress,
                shader_location: 5,
            },
            // margin
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: (mem::size_of::<[f32; 4]>() * 5 + mem::size_of::<[f32; 2]>())
                    as BufferAddress,
                shader_location: 6,
            },
        ]
    }
}

struct ExtractedNodes {
    containers: Vec<ExtractedContainer>,
}

pub struct UiMeta {
    containers: BufferVec<ExtractedContainer>,
    view_bind_group: Option<BindGroup>,
    // TODO: Add a text instances BufferVec
}

impl Default for UiMeta {
    fn default() -> Self {
        Self {
            containers: BufferVec::new(BufferUsage::VERTEX),
            view_bind_group: None,
        }
    }
}

pub fn build_ui_rendering(app: &mut App) {
    // Register UI camera
    app.world
        .get_resource_mut::<ActiveCameras>()
        .unwrap()
        .add(CAMERA_UI);

    // Configure UI render graph
    let render_app = app.sub_app(RenderApp);
    render_app
        .init_resource::<shader::UiShaders>()
        .init_resource::<UiMeta>()
        .add_system_to_stage(RenderStage::Extract, camera::extract_ui_camera)
        .add_system_to_stage(RenderStage::Extract, extract_dom)
        .add_system_to_stage(RenderStage::Prepare, camera::prepare_ui_views)
        .add_system_to_stage(RenderStage::Prepare, prepare_ui_buffer)
        .add_system_to_stage(RenderStage::Queue, queue_ui_nodes)
        .add_system_to_stage(
            RenderStage::PhaseSort,
            sort_phase_system::<node::UiPassPhase>,
        );

    let draw_ui = draw::DrawUi::new(&mut render_app.world);
    render_app
        .world
        .get_resource::<DrawFunctions>()
        .unwrap()
        .write()
        .add(draw_ui);

    let ui_pass_node = node::UiPass::new(&mut render_app.world);
    let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();

    let mut draw_ui_graph = RenderGraph::default();
    draw_ui_graph.add_node(node::UiPass::NAME, ui_pass_node);
    let input_node_id = draw_ui_graph.set_input(vec![
        SlotInfo::new(draw_ui_graph::input::VIEW_ENTITY, SlotType::Entity),
        SlotInfo::new(draw_ui_graph::input::RENDER_TARGET, SlotType::TextureView),
        SlotInfo::new(draw_ui_graph::input::DEPTH, SlotType::TextureView),
    ]);
    draw_ui_graph
        .add_slot_edge(
            input_node_id,
            draw_ui_graph::input::VIEW_ENTITY,
            node::UiPass::NAME,
            node::UiPass::IN_VIEW_ENTITY,
        )
        .unwrap();
    draw_ui_graph
        .add_slot_edge(
            input_node_id,
            draw_ui_graph::input::RENDER_TARGET,
            node::UiPass::NAME,
            node::UiPass::IN_RENDER_TARGET,
        )
        .unwrap();
    draw_ui_graph
        .add_slot_edge(
            input_node_id,
            draw_ui_graph::input::DEPTH,
            node::UiPass::NAME,
            node::UiPass::IN_DEPTH,
        )
        .unwrap();

    graph.add_sub_graph(draw_ui_graph::NAME, draw_ui_graph);

    graph.add_node(node::UiPassDriver::NAME, node::UiPassDriver);
    graph
        .add_node_edge(
            core_pipeline::node::MAIN_PASS_DRIVER,
            node::UiPassDriver::NAME,
        )
        .unwrap();
}

fn extract_dom(mut commands: Commands, dom: Res<dom::Dom>) {
    // if !dom.is_changed() { return; }

    let mut containers = Vec::default();
    let mut layer = VecDeque::from(vec![&dom.root]);
    let mut next_layer = Vec::default();

    while let Some(node) = layer.pop_front() {
        next_layer.extend(&node.children);

        match node.ty {
            dom::NodeType::Container => {
                let background_color: Vec4 =
                    if let dom::style::Background::Color(color) = node.styles.background {
                        color
                    } else {
                        color::Color::YELLOW_GREEN
                    }
                    .as_rgba_linear()
                    .into();

                containers.push(ExtractedContainer {
                    transform: Transform::from_translation(Vec3::new(0., 0., 1.)).compute_matrix(),
                    styles: ExtractedStyles {
                        background_color,
                        size: node.styles.size,
                        margin: node.styles.margin,
                    },
                });
            }
            dom::NodeType::Text(_) => {
                todo!()
            }
        }

        if layer.is_empty() && !next_layer.is_empty() {
            layer = VecDeque::from(next_layer);
            next_layer = Vec::default();
        }
    }

    commands.insert_resource(ExtractedNodes { containers });
}

fn prepare_ui_buffer(
    render_device: Res<RenderDevice>,
    extracted_nodes: Res<ExtractedNodes>,
    mut ui_meta: ResMut<UiMeta>,
) {
    if extracted_nodes.is_changed() {
        ui_meta
            .containers
            .reserve_and_clear(extracted_nodes.containers.len(), &render_device);

        for container in &extracted_nodes.containers {
            // println!("{:?}", container);
            ui_meta.containers.push(container.clone());
        }

        ui_meta.containers.write_to_staging_buffer(&render_device);
    }
}

fn queue_ui_nodes(
    draw_functions: Res<DrawFunctions>,
    render_device: Res<RenderDevice>,
    view_meta: Res<ViewMeta>,
    ui_shaders: Res<shader::UiShaders>,
    mut extracted_nodes: ResMut<ExtractedNodes>,
    mut ui_meta: ResMut<UiMeta>,
    mut views: Query<&mut RenderPhase<node::UiPassPhase>>,
) {
    if view_meta.uniforms.is_empty() {
        return;
    }

    ui_meta.view_bind_group.get_or_insert_with(|| {
        render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: view_meta.uniforms.binding(),
            }],
            label: Some("ui_view_uniforms"),
            layout: &ui_shaders.view_layout,
        })
    });
    let draw_ui_pass_function = draw_functions.read().get_id::<draw::DrawUi>().unwrap();

    for mut ui_pass_phase in views.iter_mut() {
        for (i, container) in extracted_nodes.containers.iter().enumerate() {
            let z = container.transform.transform_point3(Vec3::ZERO).z;
            ui_pass_phase.add(Drawable {
                draw_function: draw_ui_pass_function,
                draw_key: i,
                sort_key: (z / UI_Z_STEP).round() as usize,
            });
        }
    }

    extracted_nodes.containers.clear();
}
