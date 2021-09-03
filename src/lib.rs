#![feature(generic_associated_types)]

use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render2::color, transform::prelude::*,
};

mod dom;
mod ecs;
mod render;
pub mod ui;

pub const UI_Z_STEP: f32 = 0.001;

pub struct PrototypeUiPlugin;

impl Plugin for PrototypeUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(dom::Dom {
            root: dom::Node {
                ty: dom::NodeType::Container,
                styles: dom::style::Styles {
                    size: Vec2::new(1.0, 1.0),
                    margin: Vec2::ZERO,
                    background: dom::style::Background::Color(color::Color::GREEN),
                },
                children: vec![],
            },
        });

        render::build_ui_rendering(app);

        app.add_plugin(ecs::UiEcsPlugin);
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
