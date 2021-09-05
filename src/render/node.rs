use bevy::{
    core_pipeline::ViewDepthTexture,
    ecs::prelude::*,
    render2::{
        camera::{ExtractedCamera, ExtractedCameraNames},
        render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType, SlotValue},
        render_phase::{DrawFunctions, RenderPhase, TrackedRenderPass},
        render_resource::{
            LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
            RenderPassDescriptor,
        },
        renderer::RenderContext,
        view::{ExtractedView, ExtractedWindows},
    },
};

use super::{draw_ui_graph, UiMeta, CAMERA_UI};

pub struct UiPassPhase;

pub struct UiPass {
    query: QueryState<&'static RenderPhase<UiPassPhase>, With<ExtractedView>>,
}

impl UiPass {
    pub const NAME: &'static str = "ui_pass";
    pub const IN_VIEW_ENTITY: &'static str = "ui_pass_view_entity";
    pub const IN_RENDER_TARGET: &'static str = "ui_pass_render_target";
    pub const IN_DEPTH: &'static str = "ui_pass_depth";

    pub fn new(render_world: &mut World) -> Self {
        Self {
            query: QueryState::new(render_world),
        }
    }
}

impl Node for UiPass {
    fn input(&self) -> Vec<SlotInfo> {
        vec![
            SlotInfo::new(Self::IN_RENDER_TARGET, SlotType::TextureView),
            SlotInfo::new(Self::IN_DEPTH, SlotType::TextureView),
            SlotInfo::new(Self::IN_VIEW_ENTITY, SlotType::Entity),
        ]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), NodeRunError> {
        let render_texture = graph.get_input_texture(Self::IN_RENDER_TARGET)?;
        let depth_texture = graph.get_input_texture(Self::IN_DEPTH)?;
        let pass_descriptor = RenderPassDescriptor {
            label: Some("ui_pass"),
            color_attachments: &[RenderPassColorAttachment {
                view: render_texture,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: depth_texture,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        };

        let view_entity = graph.get_input_entity(Self::IN_VIEW_ENTITY)?;
        let draw_functions = world.get_resource::<DrawFunctions>().unwrap();

        let uimeta = world.get_resource::<UiMeta>().unwrap();
        uimeta
            .containers
            .write_to_buffer(&mut render_context.command_encoder);

        let phase = self
            .query
            .get_manual(world, view_entity)
            .expect("view entity should exist");

        let render_pass = render_context
            .command_encoder
            .begin_render_pass(&pass_descriptor);

        let mut draw_functions = draw_functions.write();
        let mut tracked_pass = TrackedRenderPass::new(render_pass);
        for drawable in phase.drawn_things.iter() {
            let draw_function = draw_functions.get_mut(drawable.draw_function).unwrap();
            draw_function.draw(
                world,
                &mut tracked_pass,
                view_entity,
                drawable.draw_key,
                drawable.sort_key,
            );
        }

        Ok(())
    }
}

pub struct UiPassDriver;

impl UiPassDriver {
    pub const NAME: &'static str = "ui_pass_driver";
}

impl Node for UiPassDriver {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        _render_context: &mut RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), NodeRunError> {
        let extracted_cameras = world.get_resource::<ExtractedCameraNames>().unwrap();
        let extracted_windows = world.get_resource::<ExtractedWindows>().unwrap();

        if let Some(camera) = extracted_cameras.entities.get(CAMERA_UI) {
            let extracted_camera = world.entity(*camera).get::<ExtractedCamera>().unwrap();
            let depth_texture = world.entity(*camera).get::<ViewDepthTexture>().unwrap();
            let extracted_window = extracted_windows.get(&extracted_camera.window_id).unwrap();
            let swap_chain_texture = extracted_window.swap_chain_frame.as_ref().unwrap().clone();

            graph.run_sub_graph(
                draw_ui_graph::NAME,
                vec![
                    SlotValue::Entity(*camera),
                    SlotValue::TextureView(swap_chain_texture),
                    SlotValue::TextureView(depth_texture.view.clone()),
                ],
            )?;
        }

        Ok(())
    }
}
