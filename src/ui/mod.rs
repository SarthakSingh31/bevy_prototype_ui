use std::any::TypeId;

use bevy::{
    ecs::{
        component::Component,
        prelude::*,
        system::{CommandQueue, SystemParam, SystemParamFetch, SystemState},
    },
    math::prelude::*,
};

mod tree;

pub mod camera;
pub mod component;

#[derive(Debug)]
pub enum UiStyle {}

// This should have things like:
// 1. Children (this includes slot mapping)
// 2. Incoming values from the parent (props)
pub struct UiNodeEnv;

pub struct UiNodeHandle;

pub struct UiNodeTree {
    type_id: TypeId,
}

impl UiNodeTree {
    pub fn get_or_add_branch<T: UiComponent>(&mut self) -> UiNodeHandle {


        todo!()
    }
}

pub trait UiComponent: Component {
    type Param: SystemParam;

    fn init(&mut self, commands: Commands);
    fn update(
        &mut self,
        param: <<Self::Param as SystemParam>::Fetch as SystemParamFetch>::Item,
        env: &mut UiNodeEnv,
    ) -> bool;
    fn render(&self, tree: &mut UiNodeTree);
}

pub struct WrappedUiComponent<U, P>
where
    U: UiComponent<Param = P>,
    P: SystemParam + Send + Sync + 'static,
{
    component: U,
    system_state: Option<SystemState<P>>,
}

pub trait RunableUiComponent: Component {
    fn init(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World, env: &mut UiNodeEnv) -> bool;
    fn render(&self, tree: &mut UiNodeTree);
}

impl<U, P> RunableUiComponent for WrappedUiComponent<U, P>
where
    U: UiComponent<Param = P>,
    P: SystemParam + Send + Sync + 'static,
{
    fn init(&mut self, world: &mut World) {
        self.system_state = Some(SystemState::new(world));

        let mut command_queue = CommandQueue::default();
        let commands = Commands::new(&mut command_queue, &world);

        self.component.init(commands);

        // TODO: Maybe this command queue should come from the outside and apply there? idk
        command_queue.apply(world);
    }

    fn update(&mut self, world: &mut World, env: &mut UiNodeEnv) -> bool {
        let system_state = self.system_state.as_mut().unwrap();

        let rerender = self.component.update(system_state.get_mut(world), env);

        system_state.apply(world);
        rerender
    }

    fn render(&self, tree: &mut UiNodeTree) {
        self.component.render(tree);
    }
}

pub struct BoxedRunableUiComponent {
    pub component: Box<dyn RunableUiComponent>,
}

impl<U, P> From<U> for BoxedRunableUiComponent
where
    U: UiComponent<Param = P>,
    P: SystemParam + Send + Sync + 'static,
{
    fn from(component: U) -> Self {
        BoxedRunableUiComponent {
            component: Box::new(WrappedUiComponent {
                component,
                system_state: None,
            }),
        }
    }
}

pub struct UiVNode {
    pub component: BoxedRunableUiComponent,
    pub children: Vec<UiVNode>,
}

pub struct UiDom {
    pub node: UiVNode,
    pub position: Vec2,
}
