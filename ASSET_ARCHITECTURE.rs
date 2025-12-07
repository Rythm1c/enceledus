/*
ASSET & RESOURCE ARCHITECTURE
================================

┌─────────────────────────────────────────────────────────────┐
│                     APPLICATION LAYER                       │
│  (main_loop, gui, scene_manager, physics_engine)            │
└─────────────────────────────────────────────────────────────┘
                              ↓
                ┌─────────────────────────────┐
                │    SCENE MANAGER            │
                │  - Tracks RenderableInstances
                │  - Updates transforms       │
                │  - Manages physics bodies   │
                └─────────────────────────────┘
                    ↓              ↓
    ┌───────────────────┐  ┌──────────────────┐
    │  ASSET MANAGER    │  │ RESOURCE MANAGER │
    │  (src/loader/)    │  │ (src/gpu/)       │
    │                   │  │                  │
    │ CPU-side:         │  │ GPU-side:        │
    │ - Load .glb/.gltf │  │ - Upload meshes  │
    │ - Cache CPUModels │  │ - Upload textures│
    │ - Return CPUModel │  │ - Manage GPU mem │
    │   references      │  │ - Return Handles │
    └───────────────────┘  └──────────────────┘
            │                       │
            ↓                       ↓
    ┌──────────────────────────────────────┐
    │      RENDER LAYER                    │
    │  - RenderableInstance (GPU + CPU)    │
    │  - ShaderDefines (conditional shader)│
    │  - Transform (world space)           │
    └──────────────────────────────────────┘
                    ↓
    ┌──────────────────────────────────────┐
    │      RENDERER                        │
    │  - Binds shaders based on defines    │
    │  - Uploads transforms                │
    │  - Draws batches                     │
    └──────────────────────────────────────┘


USAGE FLOW
==========

1. GUI User loads "models/astronaut.glb"
   │
   ├→ AssetManager::load_model("astronaut", "models/astronaut.glb")
   │  Returns: CPUModel with all meshes, materials, textures
   │
   └→ ResourceManager::upload_model(&cpu_model)
      Returns: GPUModelHandle(0) - GPU resources uploaded
   
2. Create instance in scene:
   │
   ├→ ShaderDefines::from_mesh_and_material(&mesh, &material)
   │
   └→ RenderableInstance::new(GPUModelHandle(0), transform, defines)
      Result: Ready to render
   
3. Each frame:
   │
   ├→ Scene updates transform (physics, animation, user input)
   │
   ├→ Renderer::render_instance(&renderable)
   │  - Gets GPUModel from ResourceManager via handle
   │  - Binds shader with defines
   │  - Uploads transform
   │  - Draws
   │
   └→ Screen shows astronaut at new position


BENEFITS OF THIS SPLIT
======================

AssetManager (Loader)
  ✓ Separate from GPU concerns
  ✓ Can batch-load multiple models
  ✓ Can unload CPU data after upload
  ✓ Easy to implement different formats (.glb, .obj, .fbx)
  ✓ Can serialize/cache processed models

ResourceManager (GPU)
  ✓ Manages GPU memory lifetime
  ✓ Deferred loading/streaming possible
  ✓ Easy to implement LOD (Level of Detail)
  ✓ Can batch similar resources
  ✓ Separate GPU resource lifecycle from CPU
  ✓ Handles from GPU is lightweight (just usize wrapper)

RenderableInstance
  ✓ Owns transform (not GPU or CPU model)
  ✓ References GPU via handle (cheap copy)
  ✓ Can override materials per-instance
  ✓ Conditional shaders via ShaderDefines
  ✓ Easy to add physics bodies later

Physics Integration
  ✓ Each RenderableInstance can have a physics::Body
  ✓ Physics updates transform each frame
  ✓ Renderer uses updated transform
  ✓ No coupling between physics and GPU
*/
