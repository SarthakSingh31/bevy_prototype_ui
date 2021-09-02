# Feature Name: `min_reactive_ui`

## Summary

This RFC aims to propose a reactive UI interface inspired by Vue/React for bevy. This RFC will try to show that this pattern can integrate well with the Bevy ECS and provide an efficient and ergonomic api for making ui components.

## Motivation

Web Development is one of the most UI intensive field in programming and I believe that borrowing some of the api design from the most popular frameworks in webdev (Vue/React) will help Bevy make a good UI framework.

Vue/React draw a good balance between retained and immediate mode rendering by only rerendering when the internal data of a `Component` changes.

## API

### UI Component

// TODO: add description

```rust
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
```

### UiNodeEnv

// TODO: refine the struct and add description

```rust
pub struct UiNodeEnv {
    pub parent_handle: UiHandle,
    pub children: Vec<Box<dyn UiComponent>> / Vec<props_somehow>,
    pub props: ?
}
```

### UiNode

// TODO: refine the struct and add description

```rust
pub struct UiNode {
    pub children: Vec<UiNode>,
    pub styles: Vec<UiStyles>,
    pub primitive: UiNodePrimitive,
}
```

### UiNodePrimitive

// TODO: refine the struct and add description

```rust
pub enum UiNodePrimitive {
    Div,
    Text,
    Slot(Option<String>),
    // More ?
}
```

## Implementation


## Things I need to figure out

1. How to pass events to any/specific `UiComponenet`?
2. How to pass events to a sibling `UiComponenet`?
2. Should there be global events?
3. How to pass props? (I can put it in the `UiNodeEnv` but I am not 100% on it)
4. How to not allow getting a `dyn UiComponent` in the `UiComponent::Param`?


## What should it actually feel like?

```rust



```