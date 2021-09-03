use bevy::{
    ecs::{
        prelude::*,
        system::{SystemParam, SystemParamFetch},
    },
    math::prelude::*,
    window::prelude::*,
};

use super::super::{UiComponent, UiNodeEnv, UiNodeTree};

pub struct Button {
    pub mouse_position: Option<Vec2>,
    pub rendered_once: bool,
}

impl UiComponent for Button {
    type Param = (Res<'static, Windows>,);

    fn init(&mut self, commands: Commands) {}
    fn update(
        &mut self,
        param: <<Self::Param as SystemParam>::Fetch as SystemParamFetch>::Item,
        env: &mut UiNodeEnv,
    ) -> bool {
        let test = param;

        if self.rendered_once {
            false
        } else {
            self.rendered_once = true;
            true
        }
    }
    fn render(&self, tree: &mut UiNodeTree) {}
}
