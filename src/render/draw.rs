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
        _sort_key: usize,
    ) {
        const VERTICES: u32 = 4;
        let (ui_shaders, ui_meta, views) = self.params.get(world);
        let view_uniform = views.get(view).unwrap();
        let ui_meta = ui_meta.into_inner();
        let ui_shaders = ui_shaders.into_inner();
        pass.set_render_pipeline(&ui_shaders.pipeline);
        pass.set_vertex_buffer(0, ui_meta.container_instances.buffer().unwrap().slice(..));
        pass.set_bind_group(
            0,
            ui_meta.view_bind_group.as_ref().unwrap(),
            &[view_uniform.offset],
        );
        // if let Some(bind_group_key) = ui_meta.texture_bind_group_keys[draw_key] {
        //     pass.set_bind_group(1, &ui_meta.texture_bind_groups[bind_group_key], &[]);
        // } else {
        //     pass.set_bind_group(1, &ui_shaders.dummy_texture_bind_group, &[]);
        // }
        let instance = draw_key as u32;

        pass.draw(0..VERTICES, instance..instance + 1);
    }
}
