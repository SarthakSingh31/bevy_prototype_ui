use std::marker::PhantomData;

use bevy::{ecs::{archetype::ArchetypeGeneration, component::Component, prelude::*, query::{FilterFetch, QueryIter, WorldQuery}, system::{CommandQueue, SystemMeta, SystemParam, SystemParamFetch, SystemParamState}, world::WorldId}};

mod component;

#[derive(Debug)]
pub enum UiStyle {}

#[derive(Debug, Default)]
pub struct UiNode {
    pub name: Option<String>,
    pub children: Vec<UiNode>,
    pub styles: Vec<UiStyle>,
}

// This should have things like:
// 1. Children (this includes slot mapping)
// 2. Incoming values from the parent (props)
pub struct UiNodeEnv;

pub trait UiComponent: Component {
    type Param: SystemParam;

    fn init(&mut self, commands: Commands);
    fn update(
        &mut self,
        param: <<Self::Param as SystemParam>::Fetch as SystemParamFetch>::Item,
        env: &mut UiNodeEnv,
    ) -> bool;
    fn render(&self) -> Option<UiNode>;
}

pub struct WrappedUiComponent<U, P>
where
    U: UiComponent<Param = P>,
    P: SystemParam,
{
    component: U,
    meta: Option<SystemMeta>,
    param_state: Option<<P as SystemParam>::Fetch>,
    world_id: Option<WorldId>,
    // TODO: get_mut uses it, idk what the use it. Should look into it becuase I am using set mut.
    archetype_generation: ArchetypeGeneration,
    phantom: PhantomData<P>,
}

pub trait RunableUiComponent {
    fn init(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World, env: &mut UiNodeEnv) -> bool;
    fn render(&self) -> Option<UiNode>;
}

impl<U, P> RunableUiComponent for WrappedUiComponent<U, P>
where
    U: UiComponent<Param = P>,
    P: SystemParam,
{
    fn init(&mut self, world: &mut World) {
        let mut meta = SystemMeta::new::<P>();
        let config = <P::Fetch as SystemParamState>::default_config();
        let param_state = <P::Fetch as SystemParamState>::init(world, &mut meta, config);
        self.meta = Some(meta);
        self.param_state = Some(param_state);
        self.world_id = Some(world.id());

        let mut command_queue = CommandQueue::default();
        let commands = Commands::new(&mut command_queue, &world);

        self.component.init(commands);

        // TODO: Maybe this command queue should come from the outside and apply there? idk
        command_queue.apply(world);
    }

    fn update(&mut self, world: &mut World, env: &mut UiNodeEnv) -> bool {
        let change_tick = world.increment_change_tick();
        let mut param_state = self.param_state.as_mut().unwrap();
        let meta = self.meta.as_ref().unwrap();
        let param = unsafe {
                <P::Fetch as SystemParamFetch>::get_param(
                &mut param_state,
                meta,
                world,
                change_tick,
            )
        };
        let rerender = self.component.update(param, env);
        param_state.apply(world);
        rerender
    }

    fn render(&self) -> Option<UiNode> {
        self.component.render()
    }
}

pub struct BoxedRunableUiComponent {
    pub component: Box<dyn RunableUiComponent>,
}

impl<U, P> From<U> for BoxedRunableUiComponent
where
    U: UiComponent<Param = P>,
    P: SystemParam + 'static,
{
    fn from(component: U) -> Self {
        BoxedRunableUiComponent {
            component: Box::new(WrappedUiComponent {
                component,
                meta: None,
                param_state: None,
                world_id: None,
                archetype_generation: ArchetypeGeneration::initial(),
                phantom: PhantomData::default(),
            }),
        }
    }
}
