pub mod shader_manager;

use std::collections::HashMap;

use glow::HasContext;
use math::transform::Transform;
use shader_manager::ShaderManager;

use crate::src::{
    cpu::model::Model,
    gpu::{mesh::GpuMesh, texture::GpuTexture},
    scene::Scene,
};

#[derive(Clone)]
pub struct GpuResourceManager {
    meshes: HashMap<usize, GpuMesh>,
    textures: HashMap<usize, GpuTexture>,
}
impl GpuResourceManager {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
            textures: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct Renderer {
    shader_manager: ShaderManager,
    resource_manager: GpuResourceManager,
    bg_color: [f32; 3],
}

impl Renderer {
    pub fn new(gl: &glow::Context) -> Self {
        Self {
            resource_manager: GpuResourceManager::new(),
            shader_manager: ShaderManager::new(gl),
            bg_color: [0.4, 0.4, 0.8],
        }
    }

    pub fn set_back_ground_color(&mut self, r: f32, g: f32, b: f32) {
        self.bg_color = [r, g, b];
    }

    pub fn upload_model(&mut self, gl: &glow::Context, model: &Model) {
        model.meshes.iter().enumerate().for_each(|(i, mesh)| {
            self.resource_manager
                .meshes
                .insert(i, GpuMesh::from_cpu(gl, mesh));
        });

        model.textures.iter().enumerate().for_each(|(i, texture)| {
            self.resource_manager
                .textures
                .insert(i, GpuTexture::from_cpu(gl, texture));
        });
    }

    pub fn begin_frame(&mut self, gl: &glow::Context, ratio: f32, scene: &Scene) {
        let bg = self.bg_color;
        unsafe {
            gl.clear_color(bg[0], bg[1], bg[2], 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT)
        }

        self.shader_manager.bind_shader(gl);
        self.shader_manager
            .update_camera_uniforms(&scene.camera, ratio)
            .set_point_lights_uniform(scene.get_point_lights())
            .set_per_frame_uniforms(gl);
    }

    fn render_mesh(
        &mut self,
        gl: &glow::Context,
        index: usize,
        model: &Model,
        trasform: &Transform,
    ) {
        let gpu_mesh = &self.resource_manager.meshes[&index];
        let cpu_mesh = &model.meshes[index];

        for index in 0..gpu_mesh.primitive_count() {
            let gpu_primitive = gpu_mesh.get_primitive(index);
            let cpu_primitive = &cpu_mesh.primitives[index];

            let material = &model.materials[cpu_primitive.material];

            self.shader_manager.bind_shader(gl);
            self.shader_manager
                .update_material_uniforms(material)
                .set_world_uniform(&trasform.to_mat());

            if let Some(index) = material.base_color_texture {
                let texture = &self.resource_manager.textures[&index];
                self.shader_manager.set_base_texture_uniform(texture);
            }

            if let Some(index) = material.metallic_roughness_texture {
                let texture = &self.resource_manager.textures[&index];
                self.shader_manager.set_metallic_map_uniform(texture);
            }

            self.shader_manager.set_per_primitive_uniforms(gl);

            gpu_primitive.draw(gl);
        }
    }

    fn render_node(
        &mut self,
        gl: &glow::Context,
        model: &Model,
        node_index: usize,
        parent_transform: &Transform,
    ) {
        let node = &model.nodes[node_index];

        let world_tranfrom = parent_transform.combine(&node.transform);

        if let Some(mesh_index) = node.mesh {
            self.render_mesh(gl, mesh_index, model, &world_tranfrom);
        }

        for child in node.children.clone() {
            self.render_node(gl, model, child, &world_tranfrom);
        }
    }

    pub fn render_scene(&mut self, gl: &glow::Context, scene: &Scene, model: &Model) {
        self.shader_manager
            .set_animated_uniform(false)
            .set_per_model_uniforms(gl);

        for node_index in &scene.nodes {
            self.render_node(gl, model, *node_index, &scene.get_transform());
        }
    }

    pub fn clean_resources(&self, gl: &glow::Context) {
        for mesh in self.resource_manager.meshes.values() {
            mesh.delete(gl);
        }

        for texture in self.resource_manager.textures.values() {
            texture.delete(gl);
        }
    }
}
