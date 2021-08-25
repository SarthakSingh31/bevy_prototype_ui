use bevy::{
    app::prelude::*,
    core_pipeline,
    ecs::prelude::*,
    math::prelude::*,
    render2::{
        camera::ActiveCameras,
        render_graph::{RenderGraph, SlotInfo, SlotType},
        render_phase::DrawFunctions,
        render_resource::{BufferUsage, BufferVec},
        renderer::RenderDevice,
        RenderStage, RenderSubApp,
    },
};

pub mod camera;
pub mod draw;
pub mod node;
pub mod shader;

mod draw_ui_graph {
    pub const NAME: &str = "draw_ui_graph";

    pub mod input {
        pub const VIEW_ENTITY: &str = "ui_view_entity";
        pub const RENDER_TARGET: &str = "ui_render_target";
        pub const DEPTH: &str = "ui_depth";
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct UiVertex {
    transform: Mat4,
    size: Vec2,
    _padding: Vec2,
}

pub struct UiMeta {
    instances: BufferVec<UiVertex>,
}

impl Default for UiMeta {
    fn default() -> Self {
        Self {
            instances: BufferVec::new(BufferUsage::VERTEX),
        }
    }
}

pub fn build_ui_rendering(app: &mut App) {
    // Register UI camera
    app.world
        .get_resource_mut::<ActiveCameras>()
        .unwrap()
        .add(camera::CAMERA_UI);

    // Configure UI render graph
    let render_app = app.sub_app_mut(RenderSubApp).unwrap();
    render_app
        .init_resource::<shader::UiShaders>()
        .init_resource::<UiMeta>()
        .add_system_to_stage(RenderStage::Extract, camera::extract_ui_camera)
        .add_system_to_stage(RenderStage::Prepare, camera::prepare_ui_views)
        .add_system_to_stage(RenderStage::Prepare, prepare_ui_buffer)
        .add_system_to_stage(RenderStage::Queue, test_queue);

    let draw_ui = draw::DrawUi::new(&mut render_app.world);
    render_app
        .world
        .get_resource::<DrawFunctions>()
        .unwrap()
        .write()
        .add(draw_ui);

    // let ui_pass_node = node::UiPass::new(&mut render_app.world);
    let ui_pass_node = node::UiPass;
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

fn extract_ui_nodes() {}

fn prepare_ui_buffer(render_device: Res<RenderDevice>, mut ui_buffer: ResMut<UiMeta>) {
    if ui_buffer.instances.capacity() == 0 {
        ui_buffer.instances.reserve(1, &render_device);

        ui_buffer.instances.push(UiVertex {
            transform: Mat4::IDENTITY,
            size: Vec2::new(100.0, 100.0),
            _padding: Vec2::default(),
        });

        ui_buffer.instances.write_to_staging_buffer(&render_device);
    }
}

fn test_queue() {}
