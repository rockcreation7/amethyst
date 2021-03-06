use smallvec::SmallVec;

use crate::ecs::*;

#[derive(Debug, Default, Clone)]
/// Contains childrens of this entity.
/// This component is automatically generated by [parent_update_system] based on [Parent] components.
pub struct Children(pub SmallVec<[Entity; 8]>);

impl Children {
    /// Crates [Children] component from slice of entities
    pub fn with(entity: &[Entity]) -> Self {
        Self(SmallVec::from_slice(entity))
    }
}
