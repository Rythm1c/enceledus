use super::material::Material;
use super::mesh::CPUMesh;
use super::texture::CPUTexture;
use math::transform::Transform;
pub struct CPUModel {
    pub meshes: Vec<CPUMesh>,
    pub materials: Vec<Material>,
    pub texture: Vec<CPUTexture>,
    pub transform: Transform,
}

impl CPUModel {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
            texture: Vec::new(),
            transform: Transform::DEFAULT,
        }
    }

    pub fn add_mesh(&mut self, mesh: CPUMesh) {
        self.meshes.push(mesh);
    }

    pub fn add_material(&mut self, material: Material) {
        self.materials.push(material);
    }

    pub fn add_texture(&mut self, texture: CPUTexture) {
        self.texture.push(texture);
    }

    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }

    pub fn material_count(&self) -> usize {
        self.materials.len()
    }

    pub fn total_vertex_count(&self) -> usize {
        self.meshes.iter().map(|m| m.vertex_count()).sum()
    }
}
