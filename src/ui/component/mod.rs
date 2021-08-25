use bevy::{
    ecs::{prelude::*, query::QueryIter, system::SystemState},
    math::prelude::*,
    window::prelude::*,
};

use super::{UiComponent, UiNode, UiNodeEnv};

pub struct Button {
    mouse_position: Option<Vec2>,
}

#[derive(Default)]
pub struct Test {}

impl UiComponent for Button {
    type Query = ();
    type FitlerQuery = ();

    fn init(&mut self, commands: Commands) {}
    fn update(
        &mut self,
        query: QueryIter<Self::Query, Self::FitlerQuery>,
        env: &mut UiNodeEnv,
    ) -> bool {
        false
    }
    fn render(&self) -> Option<UiNode> {
        None
    }
}
