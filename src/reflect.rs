use bevy::{
    prelude::*,
    scene::DynamicEntity,
};

/// Clone-like trait for duplicating Reflect-related types
pub trait CloneReflect {
    /// Clone the value via Reflection.
    #[must_use]
    fn clone_value(&self) -> Self;
}

impl CloneReflect for Vec<Box<dyn Reflect>> {
    fn clone_value(&self) -> Self {
        let mut result = Vec::new();

        for reflect in self {
            result.push(reflect.clone_value());
        }

        result
    }
}

impl CloneReflect for DynamicEntity {
    fn clone_value(&self) -> Self {
        Self {
            entity: self.entity,
            components: self.components.clone_value(),
        }
    }
}

impl CloneReflect for DynamicScene {
    fn clone_value(&self) -> Self {
        let mut entities = Vec::new();

        for entity in &self.entities {
            entities.push(entity.clone_value());
        }

        Self { entities }
    }
}
