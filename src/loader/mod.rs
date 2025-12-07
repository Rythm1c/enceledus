/// Asset management and model loading
/// Handles loading models from disk and managing GPU resources

use crate::src::core::model::CPUModel;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;

/// Manages loaded assets (models, textures, etc)
pub struct AssetManager {
    /// Loaded CPU models, indexed by asset name
    models: HashMap<String, CPUModel>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    /// Load a model from disk (supports gLTF/glb for now)
    pub fn load_model(&mut self, name: String, path: &Path) -> Result<(), Box<dyn Error>> {
        let cpu_model = self.load_gltf_model(path)?;
        self.models.insert(name, cpu_model);
        Ok(())
    }

    /// Get a reference to a loaded model
    pub fn get_model(&self, name: &str) -> Option<&CPUModel> {
        self.models.get(name)
    }

    /// Get a mutable reference to a loaded model
    pub fn get_model_mut(&mut self, name: &str) -> Option<&mut CPUModel> {
        self.models.get_mut(name)
    }

    /// Check if a model is loaded
    pub fn has_model(&self, name: &str) -> bool {
        self.models.contains_key(name)
    }

    /// List all loaded models
    pub fn list_models(&self) -> Vec<&str> {
        self.models.keys().map(|s| s.as_str()).collect()
    }

    /// Unload a model from memory
    pub fn unload_model(&mut self, name: &str) -> Option<CPUModel> {
        self.models.remove(name)
    }

    /// Clear all loaded models
    pub fn clear(&mut self) {
        self.models.clear();
    }

    // --- Private loading methods ---

    fn load_gltf_model(&self, path: &Path) -> Result<CPUModel, Box<dyn Error>> {
        // TODO: Implement gLTF loading
        // For now, return a placeholder
        Ok(CPUModel::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_manager_load_unload() {
        let mut manager = AssetManager::new();
        let model_name = "test_model".to_string();
        
        manager.models.insert(model_name.clone(), CPUModel::new());
        assert!(manager.has_model(&model_name));
        
        manager.unload_model(&model_name);
        assert!(!manager.has_model(&model_name));
    }
}
