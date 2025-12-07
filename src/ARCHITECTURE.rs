/// ARCHITECTURE OVERVIEW
/// 
/// This document describes the scalable renderer architecture designed for:
/// - GUI-based model loading (egui)
/// - Future physics integration
/// - Clean separation of concerns
/// 
/// Data Flow:
/// 
/// 1. MODEL LOADING PIPELINE
/// ========================
/// 
///    User (GUI) → AssetManager.load_model()
///                 ↓
///              CPUModel (in RAM, with mesh data)
///                 ↓
///              GPU Upload (in main loop)
///                 ↓
///              GPUModel (GPU resources: meshes, textures, materials)
///                 
/// 
/// 2. SCENE MANAGEMENT
/// ===================
/// 
///    Scene contains SceneEntity objects:
///    
///    SceneEntity {
///        id: EntityId (unique)
///        name: String
///        transform: Transform (position, rotation, scale)
///        gpu_model_id: usize (reference to GPU resources)
///        physics_body_id: Option<usize> (link to physics)
///        is_visible, is_animated
///    }
/// 
/// 
/// 3. RENDERING PIPELINE
/// ====================
/// 
///    Scene.get_renderable_entities()
///         ↓
///    For each SceneEntity:
///        - Create RenderableInstance (GPU ID + Transform + ShaderDefines)
///        - Get GPU resources by gpu_model_id
///        - Submit to Renderer
///         ↓
///    Renderer binds shaders and draws
/// 
/// 
/// 4. PHYSICS INTEGRATION (Future)
/// ================================
/// 
///    PhysicsWorld.step(delta_time)
///         ↓
///    Update velocities based on forces/gravity
///         ↓
///    For each PhysicsBody:
///        - Get linked SceneEntity
///        - Update entity.transform based on body position
///         ↓
///    Scene entities move without manual updates
/// 
/// 
/// 5. GUI INTERACTION FLOW
/// ========================
/// 
///    User clicks "Load Model" in GUI
///         ↓
///    GuiState.model_name_input captured
///         ↓
///    GuiState.create_model_from_gui() called
///         ↓
///    Scene.create_entity() → EntityId
///         ↓
///    Entity appears in scene, can be selected/edited
/// 
/// 
/// KEY DESIGN PRINCIPLES
/// ======================
/// 
/// 1. Separation of Concerns:
///    - CPUModel: Data representation (meshes, transforms)
///    - GPUModel: GPU resources only
///    - SceneEntity: World instance with metadata
///    - RenderableInstance: Ephemeral, created each frame from SceneEntity
/// 
/// 2. Extensibility:
///    - PhysicsBody can be added to any entity
///    - EntityMetadata allows custom per-entity data
///    - ShaderDefines automatically generated from attributes
/// 
/// 3. Performance:
///    - Entities batched by ShaderDefines in renderer
///    - Physics bodies only simulated if physics_enabled
///    - GPU models shared between multiple scene entities
/// 
/// 
/// INTEGRATION POINTS
/// ===================
/// 
/// In main_loop.rs:
/// 
///    1. Initialize:
///       let asset_manager = AssetManager::new();
///       let mut scene = Scene::new("Main");
///       let mut gpu_model_manager = GPUModelManager::new();
///       let mut physics_world = PhysicsWorld::new([0.0, -9.81, 0.0]);
///       let gui_system = GuiSystem::new();
/// 
///    2. Each Frame:
///       - Handle GUI input → scene updates
///       - Physics.step(delta_time) → entity transforms updated
///       - For each renderable entity → create RenderableInstance
///       - Renderer.render_instances(renderable_list)
/// 
/// 
/// EXAMPLE: ADD A MODEL VIA GUI
/// =============================
/// 
///    1. User selects model in GUI (e.g., "astronaut")
///    2. Enters name (e.g., "Player")
///    3. Clicks "Create"
///    4. Code:
///       let gpu_id = gpu_model_manager.create_model();
///       scene.create_entity("Player".into(), gpu_id);
///    5. Entity appears in scene
/// 
/// 
/// EXAMPLE: ADD PHYSICS
/// ====================
/// 
///    1. User enables physics in GUI
///    2. Creates body:
///       let body = PhysicsBody::new_dynamic(10.0); // mass
///       let body_id = physics_world.add_body(body);
///    3. Link to entity:
///       entity.physics_body_id = Some(body_id);
///    4. Each frame:
///       physics_world.step(dt);
///       // Internally updates entity transforms
/// 
///
pub struct ArchitectureGuide;
