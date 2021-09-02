use std::{any::{Any, TypeId, type_name}, collections::HashMap, fmt};

use super::{RunableUiComponent, UiStyle};

#[derive(Debug)]
pub enum UiNodePrimitive {
    Box,
    Text(String),
    Slot,
}

pub enum UiNodeTyped {
    Primitive(UiNodePrimitive),
    Component {
        name: String,
        instance: Box<dyn RunableUiComponent + 'static>,
    }
}

impl fmt::Debug for UiNodeTyped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("UiNodeTyped");

        match self {
            Self::Primitive(primitive) => {
                f.field(primitive);
            },
            Self::Component{ name, .. } => {
                f.field(&name);
            },
        }

        f.finish()
    }
}

impl From<UiNodePrimitive> for UiNodeTyped {
    fn from(primitive: UiNodePrimitive) -> Self {
        Self::Primitive(primitive)
    }
}

impl<T: RunableUiComponent> From<T> for UiNodeTyped {
    fn from(component: T) -> Self {
        Self::Component {
            name: type_name::<T>().to_string(),
            instance: Box::new(component),
        }
    }
}

#[derive(Debug)]
pub struct UiNode {
    styles: Vec<UiStyle>,
    component: UiNodeTyped,
    branch_order: Vec<TypeId>,
    branches: HashMap<TypeId, UiNode>,
    dirty: bool,
}

impl UiNode {
    pub fn branch<T: Into<UiNodeTyped> + Default>(&mut self) -> &mut UiNode {
        todo!()
    }

    pub fn prep_rerender(&mut self) -> &mut Self {
        todo!()
    }
}

