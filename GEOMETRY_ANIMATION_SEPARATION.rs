/*
CLEAN SEPARATION: GEOMETRY vs ANIMATION
========================================

PROBLEM SOLVED:
  ❌ Before: CPUModel contained both geometry AND animation
  ✅ After:  Separate concerns, linked when needed

┌──────────────────────────────────────────────────────────────┐
│                   CPU SIDE (Loading)                         │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  CPUModel (Pure Geometry)        CPUSkeleton (Pure Animation)│
│  ├── meshes                      ├── skeleton                │
│  ├── materials                   └── clips: Vec<Clip>        │
│  └── textures                                                │
│                                                               │
│  StaticModel wrapper:       AnimatedModel wrapper:           │
│  ├── name                   ├── name                         │
│  └── geometry: CPUModel     ├── geometry: CPUModel           │
│                             └── skeleton: CPUSkeleton        │
│                                                               │
└──────────────────────────────────────────────────────────────┘
                              ↓
        ┌─────────────────────────────────────┐
        │  ResourceManager.upload_*()         │
        │  - upload_model(cpu_model)          │
        │  - upload_skeleton(cpu_skeleton)    │
        │  - create_animated_model(geo, skel) │
        └─────────────────────────────────────┘
                              ↓
┌──────────────────────────────────────────────────────────────┐
│                  GPU SIDE (Rendering)                        │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  GPUModel (GPU Geometry)         GPUSkeleton (GPU Bones)     │
│  ├── VAO/VBO/EBO                 ├── bone_matrices[]         │
│  ├── materials                   └── bone_count              │
│  └── textures                                                │
│  Handle: GPUModelHandle(0)       Handle: GPUSkeletonHandle(0)│
│                                                               │
│  GPUAnimatedModel (Link):                                    │
│  ├── geometry: GPUModelHandle(0)                             │
│  └── skeleton: GPUSkeletonHandle(0)                          │
│  ID: 0                                                       │
│                                                               │
└──────────────────────────────────────────────────────────────┘
                              ↓
┌──────────────────────────────────────────────────────────────┐
│           SCENE (ECS-like Layer)                             │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  SceneEntity:                                                │
│  ├── name: "Astronaut"                                       │
│  ├── transform                                               │
│  ├── model: ModelReference::Animated(animated_id)            │
│  ├── is_visible                                              │
│  └── physics_body_id                                         │
│                                                               │
└──────────────────────────────────────────────────────────────┘
                              ↓
┌──────────────────────────────────────────────────────────────┐
│              RENDERER (Each Frame)                           │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  1. entity.to_renderable() → RenderableInstance              │
│  2. Get GPU resources from ResourceManager                   │
│  3. If animated:                                             │
│     - Get GPUAnimatedModel                                   │
│     - Get GPUSkeleton                                        │
│     - Upload bone matrices to shader                         │
│  4. Render with shader defines                              │
│                                                               │
└──────────────────────────────────────────────────────────────┘


BENEFITS
========

✓ Clean Separation:
  - Geometry = VBOs, indices, materials
  - Animation = Bones, keyframes, clips
  - No coupling between them

✓ Flexible Loading:
  - Load only geometry (static models)
  - Load only animation (reuse geometry)
  - Load both together (character models)

✓ Memory Efficient:
  - Share geometry across multiple skeletons
  - Share skeleton across multiple models
  - Separate lifecycle management

✓ Physics Integration:
  - Physics updates SceneEntity.transform
  - Animation updates bone matrices separately
  - No conflicts

✓ Future Features:
  - LOD: Different geometry for different distances
  - Blend shapes: Separate from skeleton
  - IK: Update bones independently
  - Motion capture: Retarget to different skeletons


USAGE EXAMPLE
=============

// Load astronaut with animation
let mut asset_mgr = AssetManager::new();
asset_mgr.load_model("astronaut", "models/astronaut.glb")?;
let cpu_model = asset_mgr.get_model("astronaut")?;

// Separate geometry and animation
let static_model = StaticModel::new(cpu_model.geometry.clone());
let skeleton = CPUSkeleton::new()
    .with_skeleton(cpu_model.skeleton.clone());

// Upload to GPU
let geo_handle = res_mgr.upload_model(&static_model.geometry);
let skel_handle = res_mgr.upload_skeleton(&skeleton)?;
let animated_id = res_mgr.create_animated_model(geo_handle, skel_handle);

// Create scene entity
let entity = SceneEntity::new(
    EntityId::new(),
    "Astronaut".to_string(),
    ModelReference::Animated(animated_id)
);

// Each frame:
let defines = ShaderDefines::from_mesh_and_material(...);
let renderable = entity.to_renderable(defines);
renderer.render_instance(&renderable);

// Update bones (from animation controller)
if let ModelReference::Animated(id) = entity.model {
    if let Some(animated) = res_mgr.get_animated_model(id) {
        if let Some(skel_handle) = GPUSkeletonHandle(animated.skeleton) {
            if let Some(skeleton) = res_mgr.get_skeleton_mut(skel_handle) {
                skeleton.set_bone_matrix(bone_idx, new_matrix);
            }
        }
    }
}
*/
