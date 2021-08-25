use std::marker::PhantomData;

use bevy::{
    ecs::{
        component::Component,
        prelude::*,
        query::{FilterFetch, QueryIter, WorldQuery},
    },
};

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

pub trait UiComponent: Component
where
    <Self::FitlerQuery as WorldQuery>::Fetch: FilterFetch,
{
    type Query: WorldQuery;
    type FitlerQuery: WorldQuery;

    fn init(&mut self, commands: Commands);
    fn update(
        &mut self,
        query: QueryIter<Self::Query, Self::FitlerQuery>,
        env: &mut UiNodeEnv,
    ) -> bool;
    fn render(&self) -> Option<UiNode>;
}

pub struct WrappedUiComponent<U, Q, F>
where
    U: UiComponent<Query = Q, FitlerQuery = F>,
    Q: WorldQuery,
    F: WorldQuery,
    F::Fetch: FilterFetch,
{
    component: U,
    phantom: PhantomData<(Q, F)>,
}

pub trait RunableUiComponent {
    fn init(&mut self, commands: Commands);
    fn update(&mut self, world: &mut World, env: &mut UiNodeEnv) -> bool;
    fn render(&self) -> Option<UiNode>;
}

impl<U, Q, F> RunableUiComponent for WrappedUiComponent<U, Q, F>
where
    U: UiComponent<Query = Q, FitlerQuery = F>,
    Q: WorldQuery,
    F: WorldQuery,
    F::Fetch: FilterFetch,
{
    fn init(&mut self, commands: Commands) {
        self.component.init(commands);
    }

    fn update(&mut self, world: &mut World, env: &mut UiNodeEnv) -> bool {
        let mut query_state = QueryState::new(world);
        self.component.update(query_state.iter_mut(world), env)
    }

    fn render(&self) -> Option<UiNode> {
        self.component.render()
    }
}

pub struct BoxedRunableUiComponentVec {
    components: Vec<Box<dyn RunableUiComponent>>,
}

impl<U, Q, F> From<U> for WrappedUiComponent<U, Q, F>
where
    U: UiComponent<Query = Q, FitlerQuery = F>,
    Q: WorldQuery,
    F: WorldQuery,
    F::Fetch: FilterFetch,
{
    fn from(component: U) -> Self {
        Self {
            component,
            phantom: PhantomData::default(),
        }
    }
}
