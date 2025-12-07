use crate::src::render::RenderableInstance;
use crate::src::gpu::resource_manager::GPUModelHandle;
use math::transform::Transform;
use std::collections::HashMap;

/// Unique identifier for entities in the scene
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        EntityId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

/// What kind of model this entity uses
#[derive(Clone, Copy, Debug)]
pub enum ModelReference {
    /// Static geometry model
    Geometry(GPUModelHandle),
    /// Animated model (geometry + skeleton)
    Animated(usize), // ID from ResourceManager
}

/// Represents a renderable object in the scene
#[derive(Clone)]
pub struct SceneEntity {
    pub id: EntityId,
    pub name: String,
    pub transform: Transform,
    pub model: ModelReference,
    pub is_visible: bool,
    
    // Physics (optional, for future extension)
    pub physics_body_id: Option<usize>,
    
    // Custom user data
    pub user_data: HashMap<String, EntityMetadata>,
}

#[derive(Clone, Debug)]
pub enum EntityMetadata {
    String(String),
    Float(f32),
    Int(i32),
    Bool(bool),
}

impl SceneEntity {
    pub fn new(id: EntityId, name: String, model: ModelReference) -> Self {
        Self {
            id,
            name,
            transform: Transform::DEFAULT,
            model,
            is_visible: true,
            physics_body_id: None,
            user_data: HashMap::new(),
        }
    }

    pub fn to_renderable(&self, shader_defines: crate::src::render::ShaderDefines) -> RenderableInstance {
        match self.model {
            ModelReference::Geometry(handle) => {
                RenderableInstance::new(handle, self.transform, shader_defines)
            }
            ModelReference::Animated(_) => {
                // For animated models, still use the handle but mark as animated
                // The renderer will look up the animated model separately
                RenderableInstance::new(GPUModelHandle(0), self.transform, shader_defines)
                    .with_animation(true)
            }
        }
    }

    pub fn is_animated(&self) -> bool {
        matches!(self.model, ModelReference::Animated(_))
    }
}

/// The main scene containing all entities and world state
pub struct Scene {
    entities: HashMap<EntityId, SceneEntity>,
    entity_counter: u64,
    
    // Scene metadata
    pub name: String,
    pub ambient_light: [f32; 3],
    pub background_color: [f32; 4],
}

impl Scene {
    pub fn new(name: String) -> Self {
        Self {
            entities: HashMap::new(),
            entity_counter: 0,
            name,
            ambient_light: [0.3, 0.3, 0.3],
            background_color: [0.1, 0.1, 0.1, 1.0],
        }
    }

    /// Add an entity to the scene
    pub fn add_entity(&mut self, entity: SceneEntity) -> EntityId {
        let id = entity.id;
        self.entities.insert(id, entity);
        id
    }

    /// Create and add a new entity
    pub fn create_entity(&mut self, name: String, gpu_model_id: usize) -> EntityId {
        let id = EntityId::new();
        let entity = SceneEntity::new(id, name, gpu_model_id);
        self.add_entity(entity);
        id
    }

    /// Get a reference to an entity
    pub fn get_entity(&self, id: EntityId) -> Option<&SceneEntity> {
        self.entities.get(&id)
    }

    /// Get a mutable reference to an entity (for transform updates)
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut SceneEntity> {
        self.entities.get_mut(&id)
    }

    /// Remove an entity from the scene
    pub fn remove_entity(&mut self, id: EntityId) -> Option<SceneEntity> {
        self.entities.remove(&id)
    }

    /// Get all visible, renderable entities
    pub fn get_renderable_entities(&self) -> Vec<&SceneEntity> {
        self.entities
            .values()
            .filter(|e| e.is_visible)
            .collect()
    }

    /// Update all entity transforms (useful for physics simulation step)
    pub fn update(&mut self, delta_time: f32) {
        // In the future, this would:
        // 1. Step physics simulation
        // 2. Update animation states
        // 3. Update transforms based on physics bodies
        
        for entity in self.entities.values_mut() {
            // Placeholder for future physics/animation updates
            if entity.is_animated {
                // Animation updates would go here
            }
        }
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }
}