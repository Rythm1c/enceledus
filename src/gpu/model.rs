use super::mesh::GPUMesh;
use super::texture::GPUTexture;

use crate::src::core::material::Material;

/// GPU-side model resource
pub struct GPUModel {
    pub id: usize,
    pub meshes: Vec<GPUMesh>,
    pub materials: Vec<Material>,
    pub textures: Vec<GPUTexture>,
}

impl GPUModel {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            meshes: Vec::new(),
            materials: Vec::new(),
            textures: Vec::new(),
        }
    }
}

/// Manager for GPU models
pub struct GPUModelManager {
    models: Vec<GPUModel>,
    model_counter: usize,
}

impl GPUModelManager {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            model_counter: 0,
        }
    }

    /// Create a new GPU model and return its ID
    pub fn create_model(&mut self) -> usize {
        let id = self.model_counter;
        self.models.push(GPUModel::new(id));
        self.model_counter += 1;
        id
    }

    /// Get a reference to a GPU model
    pub fn get_model(&self, id: usize) -> Option<&GPUModel> {
        self.models.iter().find(|m| m.id == id)
    }

    /// Get a mutable reference to a GPU model
    pub fn get_model_mut(&mut self, id: usize) -> Option<&mut GPUModel> {
        self.models.iter_mut().find(|m| m.id == id)
    }

    pub fn model_count(&self) -> usize {
        self.models.len()
    }
}


