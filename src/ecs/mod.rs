use std::{collections::VecDeque, sync::Once};

use bevy::{app::prelude::*, ecs::prelude::*, math::prelude::*};

use crate::ui::{component::Button, BoxedRunableUiComponent, UiDom, UiVNode, UiNodeEnv};

#[derive(Debug, Clone, Hash, PartialEq, Eq, StageLabel)]
pub struct UiStage;

pub struct UiEcsPlugin;

impl Plugin for UiEcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PostUpdate,
            UiStage,
            SystemStage::single(ecs_component_runner.exclusive_system()),
        );
    }
}

static mut VDOM_ROOTS_CACHE: Vec<UiDom> = vec![];
static INIT: Once = Once::new();

// TODO: This is very temporary
fn ecs_component_runner(world: &mut World) {
    // let compute_pool = world
    //     .get_resource_or_insert_with(|| ComputeTaskPool(TaskPool::default()))
    //     .clone();

    let vdom_roots: &mut Vec<UiDom> = unsafe {
        INIT.call_once(|| {
            VDOM_ROOTS_CACHE = vec![UiDom {
                node: UiVNode {
                    component: Button {
                        mouse_position: None,
                        rendered_once: false,
                    }
                    .into(),
                    children: Vec::default(),
                },
                position: Vec2::default(),
            },];
        });
        &mut VDOM_ROOTS_CACHE
    };

    for UiDom { node, position } in vdom_roots {
        let mut components = VecDeque::new();
        components.push_front(node);

        // loop {
        //     if let Some(node) = components.front() {

        //     }
        // }
    }

    // for ui_component in ui_components.iter_mut() {
    //     ui_component.component.init(world);
    // }

    // let ui_components: Vec<_> = ui_components
    //     .iter_mut()
    //     .filter_map(|ui_component| {
    //         if ui_component.component.update(world, &mut UiNodeEnv) {
    //             Some(ui_component)
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // for ui_component in ui_components {
    //     let rendered = ui_component.component.render();
    //     println!("Rendered {:?}", rendered);
    // }
}
