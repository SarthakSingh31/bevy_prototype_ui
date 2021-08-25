use bevy::{ecs::{prelude::*, query::QueryIter, system::{SystemParam, SystemParamFetch, SystemState}}, math::prelude::*, window::prelude::*};

use super::{BoxedRunableUiComponent, UiComponent, UiNode, UiNodeEnv};

pub struct Button {
    mouse_position: Option<Vec2>,
}

#[derive(Default)]
pub struct Test {}

impl UiComponent for Button {
    type Param = (Res<'static, Windows>,);

    fn init(&mut self, commands: Commands) {}
    fn update(
        &mut self,
        param: <<Self::Param as SystemParam>::Fetch as SystemParamFetch>::Item,
        env: &mut UiNodeEnv,
    ) -> bool {
        let test = param;
        false
    }
    fn render(&self) -> Option<UiNode> {
        None
    }
}

fn _test() {
    let button = Button {
        mouse_position: None,
    };
    
    let _boxed: BoxedRunableUiComponent = button.into();
}
