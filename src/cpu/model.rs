use math::vec3::vec3;

use super::material::Material;
use super::mesh::Mesh;
use super::node::Node;
use super::skin::Skin;
use super::texture::Texture;

/// Pure geometry data, no animation
#[derive(Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub skins: Vec<Skin>,
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub nodes: Vec<Node>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            skins: Vec::new(),
            textures: Vec::new(),
            materials: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }

    pub fn total_vertex_count(&self) -> usize {
        self.meshes.iter().map(|m| m.vertex_count()).sum()
    }

    pub fn total_index_count(&self) -> usize {
        self.meshes.iter().map(|m| m.index_count()).sum()
    }

    pub fn get_point_scale(&self) -> math::vec3::Vec3 {
        let bounds = self.meshes.iter().map(|mesh| mesh.get_bounds());

        let minimum = bounds
            .clone()
            .map(|bound| bound.0)
            .reduce(f32::min)
            .expect("could not get minimum point in model!");

        let maximum = bounds
            .map(|bound| bound.1)
            .reduce(f32::max)
            .expect("could not get maximum point in model!");

        let factor = 1.0 / (maximum - minimum).abs();

        vec3(factor, factor, factor)
    }
}
