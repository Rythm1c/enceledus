use super::model::CPUModel;
use super::skeleton::CPUSkeleton;

/// Composite: Geometry + Animation for a single model
/// Use this when loading a model that has animations
#[derive(Clone)]
pub struct AnimatedModel {
    pub name: Option<String>,
    pub geometry: CPUModel,
    pub skeleton: CPUSkeleton,
}

impl AnimatedModel {
    pub fn new(geometry: CPUModel, skeleton: CPUSkeleton) -> Self {
        Self {
            name: None,
            geometry,
            skeleton,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.geometry.meshes.is_empty() && self.skeleton.has_skeleton()
    }

    /// Check if this model has skinned meshes
    pub fn has_skinning(&self) -> bool {
        self.geometry
            .meshes
            .iter()
            .any(|mesh| mesh.attributes.has_skinning)
    }
}

/// Use this when loading a model without animation
#[derive(Clone)]
pub struct StaticModel {
    pub name: Option<String>,
    pub geometry: CPUModel,
}

impl StaticModel {
    pub fn new(geometry: CPUModel) -> Self {
        Self {
            name: None,
            geometry,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}
