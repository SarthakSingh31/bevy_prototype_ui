#![feature(generic_associated_types)]

use bevy::{app::prelude::*, ecs::prelude::*, math::prelude::*, transform::prelude::*};

mod render;
mod ui;

pub struct PrototypeUiPlugin;

impl Plugin for PrototypeUiPlugin {
    fn build(&self, app: &mut App) {
        render::build_ui_rendering(app);
    }
}

#[derive(Debug, Default)]
pub struct UiNode {
    pub size: Vec2,
}

#[derive(Default, Bundle)]
pub struct UiBundle {
    pub node: UiNode,
    pub transform: GlobalTransform,
}
