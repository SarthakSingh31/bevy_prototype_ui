use bevy::ecs::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render2::render_phase::{Draw, TrackedRenderPass};
use bevy::render2::view::ViewUniformOffset;

use super::shader;

type DrawUiQuery<'s, 'w> = (
    Res<'w, shader::UiShaders>,
    Res<'w, super::UiMeta>,
    Query<'w, 's, &'w ViewUniformOffset>,
);
pub struct DrawUi {
    params: SystemState<DrawUiQuery<'static, 'static>>,
}

impl DrawUi {
    pub fn new(world: &mut World) -> Self {
        Self {
            params: SystemState::new(world),
        }
    }
}

impl Draw for DrawUi {
    fn draw<'w, 's>(
        &'s mut self,
        world: &'w World,
        pass: &mut TrackedRenderPass<'w>,
        view: Entity,
        draw_key: usize,
        sort_key: usize,
    ) {
        println!("trying to draw some ui");
        // const VERTICES: u32 = 5;
        // let (ui_shaders, ui_buffer, views) = self.params.get(world);
        // let view_uniform = views.get(view).unwrap();
        // pass.set_render_pipeline(&ui_shaders.into_inner().pipeline);
        // pass.set_vertex_buffer(0, ui_buffer.instances.buffer().unwrap().slice(..));
        // TODO
    }
}
