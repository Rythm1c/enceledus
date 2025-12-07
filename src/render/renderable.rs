use crate::src::core::material::Material;
use crate::src::core::mesh::MeshAttributes;
use crate::src::gpu::resource_manager::GPUModelHandle;
use math::transform::Transform;

/// Information about what attributes a renderable has
/// Used to generate shader #defines like HAS_NORMALS, HAS_TEXCOORDS, etc
#[derive(Clone, Copy, Debug)]
pub struct ShaderDefines {
    pub has_normals: bool,
    pub has_texcoords: bool,
    pub has_skinning: bool,
    pub has_basecolor_texture: bool,
    pub has_metallic_texture: bool,
    pub has_normal_texture: bool,
    pub has_occlusion_texture: bool,
    pub has_emissive_texture: bool,
}

impl ShaderDefines {
    /// Build shader defines from mesh attributes and material
    pub fn from_mesh_and_material(
        mesh_attrs: &MeshAttributes,
        material: Option<&Material>,
    ) -> Self {
        let default_material = Material::new();
        let mat = material.unwrap_or(&default_material);

        Self {
            has_normals: mesh_attrs.has_normal,
            has_texcoords: mesh_attrs.has_uv,
            has_skinning: mesh_attrs.has_skinning,
            has_basecolor_texture: mat.base_color_texture.is_some(),
            has_metallic_texture: mat.metallic_roughness_texture.is_some(),
            has_normal_texture: mat.normal_texture.is_some(),
            has_occlusion_texture: mat.occlusion_texture.is_some(),
            has_emissive_texture: mat.emissive_texture.is_some(),
        }
    }

    /// Generate preprocessor defines string for shader compilation
    pub fn to_define_string(&self) -> String {
        let mut defines = String::new();

        if self.has_normals {
            defines.push_str("#define HAS_NORMALS\n");
        }
        if self.has_texcoords {
            defines.push_str("#define HAS_TEXCOORDS\n");
        }
        if self.has_skinning {
            defines.push_str("#define HAS_SKINNING\n");
        }
        if self.has_basecolor_texture {
            defines.push_str("#define HAS_BASECOLOR_TEXTURE\n");
        }
        if self.has_metallic_texture {
            defines.push_str("#define HAS_METALLIC_TEXTURE\n");
        }
        if self.has_normal_texture {
            defines.push_str("#define HAS_NORMAL_TEXTURE\n");
        }
        if self.has_occlusion_texture {
            defines.push_str("#define HAS_OCCLUSION_TEXTURE\n");
        }
        if self.has_emissive_texture {
            defines.push_str("#define HAS_EMISSIVE_TEXTURE\n");
        }

        defines
    }
}

/// A renderable instance in the scene
/// Combines GPU model resources with transform and material info
pub struct RenderableInstance {
    /// Handle to the GPU model resource
    pub gpu_model: GPUModelHandle,

    /// World transform applied to this instance
    pub transform: Transform,

    /// Material parameter overrides (optional, uses material from model if None)
    pub material_overrides: Option<Material>,

    /// Shader defines for this instance's rendering
    pub shader_defines: ShaderDefines,

    /// Whether to use skinning/animation
    pub is_animated: bool,
}

impl RenderableInstance {
    pub fn new(
        gpu_model: GPUModelHandle,
        transform: Transform,
        shader_defines: ShaderDefines,
    ) -> Self {
        Self {
            gpu_model,
            transform,
            material_overrides: None,
            shader_defines,
            is_animated: false,
        }
    }

    pub fn with_animation(mut self, enabled: bool) -> Self {
        self.is_animated = enabled;
        self
    }

    pub fn with_material_override(mut self, material: Material) -> Self {
        self.material_overrides = Some(material);
        self
    }

    pub fn get_transform_matrix(&self) -> math::mat4::Mat4 {
        self.transform.to_mat()
    }
}
