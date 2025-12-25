use crate::src::model::{
    loader::GltfFile, material::Material, mesh::Mesh, node::Node, skin::Skin, texture::Texture,
};

#[derive(Clone)]
pub struct ResourceManager {
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub nodes: Vec<Node>,
    pub skins: Vec<Skin>,
}
impl ResourceManager {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            textures: Vec::new(),
            materials: Vec::new(),
            nodes: Vec::new(),
            skins: Vec::new(),
        }
    }

    pub fn from_gltf(gl: &glow::Context, file: &GltfFile) -> Self {
        let doc = file.get_document();
        Self {
            meshes: (doc
                .meshes()
                .map(|mesh| Mesh::from_gltf(gl, &mesh, file))
                .collect::<Vec<Mesh>>()),
            textures: (doc
                .textures()
                .map(|texture| Texture::from_gltf(gl, &texture, file))
                .collect::<Vec<Texture>>()),
            materials: (doc
                .materials()
                .map(|material| Material::from_gltf(&material))
                .collect::<Vec<Material>>()),
            nodes: (doc
                .nodes()
                .map(|node| Node::from_gltf(&node))
                .collect::<Vec<Node>>()),
            skins: (doc
                .skins()
                .map(|skin| Skin::from_gltf(&skin))
                .collect::<Vec<Skin>>()),
        }
    }

    pub fn get_mesh(&self, index: usize) -> &Mesh {
        if let Some(mesh) = self.meshes.get(index) {
            return mesh;
        } else {
            panic!("mesh index out of bounds");
        }
    }

    pub fn get_texture(&self, index: usize) -> &Texture {
        if let Some(texture) = self.textures.get(index) {
            return texture;
        } else {
            panic!("mesh index out of bounds");
        }
    }

    pub fn get_material(&self, index: usize) -> &Material {
        if let Some(material) = self.materials.get(index) {
            return material;
        } else {
            panic!("mesh index out of bounds");
        }
    }

    pub fn get_node(&self, index: usize) -> &Node {
        if let Some(node) = self.nodes.get(index) {
            return node;
        } else {
            panic!("mesh index out of bounds");
        }
    }
}
