/// GPU Resource Manager
/// Manages GPU-side resources (models, meshes, textures, skeletons)
/// Handles uploading CPU data to GPU and managing GPU memory

use crate::src::core::model::CPUModel;
use crate::src::core::skeleton::CPUSkeleton;
use crate::src::gpu::model::GPUModel;
use crate::src::gpu::mesh::GPUMesh;
use crate::src::gpu::texture::GPUTexture;
use crate::src::gpu::skeleton::{GPUSkeleton, GPUAnimatedModel};
use std::collections::HashMap;

/// Handle to identify GPU models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GPUModelHandle(pub usize);

/// Handle to identify GPU skeletons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GPUSkeletonHandle(pub usize);

/// Manages all GPU resources
pub struct ResourceManager {
    /// GPU models indexed by handle
    gpu_models: HashMap<GPUModelHandle, GPUModel>,
    
    /// GPU skeletons indexed by handle
    gpu_skeletons: HashMap<GPUSkeletonHandle, GPUSkeleton>,
    
    /// Animated models linking geometry + skeleton
    animated_models: HashMap<usize, GPUAnimatedModel>,
    
    /// Next available model handle
    next_model_handle: usize,
    
    /// Next available skeleton handle
    next_skeleton_handle: usize,
    
    /// Next available animated model ID
    next_animated_id: usize,
    
    /// Optional texture cache (for shared textures)
    texture_cache: HashMap<String, GPUTexture>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            gpu_models: HashMap::new(),
            gpu_skeletons: HashMap::new(),
            animated_models: HashMap::new(),
            next_model_handle: 0,
            next_skeleton_handle: 0,
            next_animated_id: 0,
            texture_cache: HashMap::new(),
        }
    }

    /// Upload a CPU model to GPU and get a handle
    pub fn upload_model(&mut self, cpu_model: &CPUModel) -> GPUModelHandle {
        let handle = GPUModelHandle(self.next_model_handle);
        self.next_model_handle += 1;

        // Convert CPU resources to GPU resources
        let gpu_meshes = cpu_model
            .meshes
            .iter()
            .map(|cpu_mesh| self.upload_mesh(cpu_mesh))
            .collect();

        let gpu_model = GPUModel {
            id: handle.0,
            meshes: gpu_meshes,
            materials: cpu_model.materials.clone(),
            textures: Vec::new(), // TODO: Upload textures
        };

        self.gpu_models.insert(handle, gpu_model);
        handle
    }

    /// Upload a skeleton for animation
    pub fn upload_skeleton(&mut self, cpu_skeleton: &CPUSkeleton) -> Option<GPUSkeletonHandle> {
        let skeleton = cpu_skeleton.skeleton.as_ref()?;
        
        let handle = GPUSkeletonHandle(self.next_skeleton_handle);
        self.next_skeleton_handle += 1;

        // TODO: Extract bone count from skeleton properly
        let bone_count = 100; // Placeholder
        let gpu_skeleton = GPUSkeleton::new(handle.0, bone_count);

        self.gpu_skeletons.insert(handle, gpu_skeleton);
        Some(handle)
    }

    /// Create an animated model linking geometry + skeleton
    pub fn create_animated_model(
        &mut self,
        geometry_handle: GPUModelHandle,
        skeleton_handle: GPUSkeletonHandle,
    ) -> usize {
        let id = self.next_animated_id;
        self.next_animated_id += 1;

        let animated = GPUAnimatedModel::new(id, geometry_handle)
            .with_skeleton(skeleton_handle.0);

        self.animated_models.insert(id, animated);
        id
    }

    /// Get a GPU model by handle
    pub fn get_model(&self, handle: GPUModelHandle) -> Option<&GPUModel> {
        self.gpu_models.get(&handle)
    }

    /// Get a mutable GPU model by handle
    pub fn get_model_mut(&mut self, handle: GPUModelHandle) -> Option<&mut GPUModel> {
        self.gpu_models.get_mut(&handle)
    }

    /// Get a GPU skeleton by handle
    pub fn get_skeleton(&self, handle: GPUSkeletonHandle) -> Option<&GPUSkeleton> {
        self.gpu_skeletons.get(&handle)
    }

    /// Get a mutable GPU skeleton by handle
    pub fn get_skeleton_mut(&mut self, handle: GPUSkeletonHandle) -> Option<&mut GPUSkeleton> {
        self.gpu_skeletons.get_mut(&handle)
    }

    /// Get an animated model
    pub fn get_animated_model(&self, id: usize) -> Option<&GPUAnimatedModel> {
        self.animated_models.get(&id)
    }

    /// Check if a model is loaded
    pub fn has_model(&self, handle: GPUModelHandle) -> bool {
        self.gpu_models.contains_key(&handle)
    }

    /// Check if a skeleton is loaded
    pub fn has_skeleton(&self, handle: GPUSkeletonHandle) -> bool {
        self.gpu_skeletons.contains_key(&handle)
    }

    /// Unload a GPU model, freeing its resources
    pub fn unload_model(&mut self, handle: GPUModelHandle) -> Option<GPUModel> {
        self.gpu_models.remove(&handle)
    }

    /// Unload a GPU skeleton
    pub fn unload_skeleton(&mut self, handle: GPUSkeletonHandle) -> Option<GPUSkeleton> {
        self.gpu_skeletons.remove(&handle)
    }

    /// Clear all GPU resources
    pub fn clear(&mut self) {
        self.gpu_models.clear();
        self.gpu_skeletons.clear();
        self.animated_models.clear();
        self.texture_cache.clear();
    }

    /// Get memory usage stats (for profiling/debugging)
    pub fn get_stats(&self) -> ResourceStats {
        ResourceStats {
            model_count: self.gpu_models.len(),
            skeleton_count: self.gpu_skeletons.len(),
            animated_model_count: self.animated_models.len(),
            texture_cache_size: self.texture_cache.len(),
        }
    }

    // --- Private upload methods ---

    fn upload_mesh(&mut self, cpu_mesh: &crate::src::core::mesh::CPUMesh) -> GPUMesh {
        // TODO: Implement actual GPU mesh upload
        // This would involve:
        // - Creating VAO, VBO, EBO
        // - Uploading vertex data to GPU
        // - Setting up vertex attribute pointers
        GPUMesh {
            // placeholder
        }
    }
}

/// Resource statistics for profiling
#[derive(Debug, Clone)]
pub struct ResourceStats {
    pub model_count: usize,
    pub skeleton_count: usize,
    pub animated_model_count: usize,
    pub texture_cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_manager_upload() {
        let mut manager = ResourceManager::new();
        let cpu_model = CPUModel::new();

        let handle1 = manager.upload_model(&cpu_model);
        let handle2 = manager.upload_model(&cpu_model);

        assert!(handle1 != handle2);
        assert!(manager.has_model(handle1));
        assert!(manager.has_model(handle2));

        manager.unload_model(handle1);
        assert!(!manager.has_model(handle1));
        assert!(manager.has_model(handle2));
    }
}

