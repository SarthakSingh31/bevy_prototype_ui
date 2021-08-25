use bevy::{
    core_pipeline::ViewDepthTexture,
    render2::{
        camera::{ExtractedCamera, ExtractedCameraNames},
        render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType, SlotValue},
        render_phase::{DrawFunctions, TrackedRenderPass},
        render_resource::RenderPassDescriptor,
        renderer::RenderContext,
        view::ExtractedWindows,
    },
};

use super::{camera, draw_ui_graph};

pub struct TransparentUiPhase;

pub struct UiPass;

impl UiPass {
    pub const NAME: &'static str = "ui_pass";
    pub const IN_VIEW_ENTITY: &'static str = "ui_pass_view_entity";
    pub const IN_RENDER_TARGET: &'static str = "ui_pass_render_target";
    pub const IN_DEPTH: &'static str = "ui_pass_depth";
}

impl Node for UiPass {
    fn input(&self) -> Vec<SlotInfo> {
        vec![
            SlotInfo::new(Self::IN_RENDER_TARGET, SlotType::TextureView),
            SlotInfo::new(Self::IN_DEPTH, SlotType::TextureView),
            SlotInfo::new(Self::IN_VIEW_ENTITY, SlotType::Entity),
        ]
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), NodeRunError> {
        let pass_descriptor = RenderPassDescriptor {
            label: Some("ui_pass"),
            color_attachments: &[],
            depth_stencil_attachment: None,
        };

        let draw_functions = world.get_resource::<DrawFunctions>().unwrap();
        let render_pass = render_context
            .command_encoder
            .begin_render_pass(&pass_descriptor);

        let mut draw_functions = draw_functions.write();

        // Used for debugging?
        let mut tracked_pass = TrackedRenderPass::new(render_pass);

        // TODO

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
        render_context: &mut RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), NodeRunError> {
        let extracted_cameras = world.get_resource::<ExtractedCameraNames>().unwrap();
        let extracted_windows = world.get_resource::<ExtractedWindows>().unwrap();

        if let Some(camera) = extracted_cameras.entities.get(camera::CAMERA_UI) {
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
