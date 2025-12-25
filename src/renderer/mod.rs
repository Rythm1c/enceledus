pub mod command;
pub mod managers;
pub mod shader;

use glow::HasContext;
use math::transform::Transform;

use managers::{resources::ResourceManager, shader::ShaderManager};

use crate::src::{
    model::{loader::GltfFile, material::Material, mesh::Mesh, scene::Scene},
    viewer::{camera::Camera, light::PointLight},
};

use crate::src::utils::color::*;

#[derive(Clone)]
pub struct Renderer {
    shader_manager: ShaderManager,
    resource_manager: ResourceManager,
    background_color: ColorRGB,
}

impl Renderer {
    pub fn new(gl: &glow::Context, file: &GltfFile) -> Self {
        Self {
            resource_manager: ResourceManager::from_gltf(gl, &file),
            shader_manager: ShaderManager::new(gl),
            background_color: color_rgb(0.4, 0.4, 0.8),
        }
    }

    pub fn set_back_ground_color(&mut self, r: f32, g: f32, b: f32) {
        self.background_color = color_rgb(r, g, b);
    }

    fn render_scene(&mut self, gl: &glow::Context, scene: &Scene, scene_tranform: &Transform) {
        self.shader_manager
            .set_animated_uniform(false)
            .update_per_model_uniforms(gl);

        for node in &scene.nodes {
            render_node(
                gl,
                *node,
                &self.resource_manager,
                &mut self.shader_manager,
                &scene_tranform,
            );
        }
    }

    pub fn render(
        &mut self,
        gl: &glow::Context,
        window_ratio: f32,
        scene: &Scene,
        camera: &Camera,
        lights: &Vec<PointLight>,
        scene_tranform: &Transform,
    ) {
        let bg = &self.background_color;

        Self::clear_with_color(gl, bg.r, bg.g, bg.b);
        Self::clear_screen(gl);

        self.shader_manager.bind_shader(gl);
        self.shader_manager
            .update_camera_uniforms(&camera, window_ratio)
            .set_point_lights_uniform(lights)
            .update_per_frame_uniforms(gl);

        self.render_scene(gl, scene, scene_tranform);
    }

    fn clear_screen(gl: &glow::Context) {
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn clear_with_color(gl: &glow::Context, r: f32, g: f32, b: f32) {
        unsafe {
            gl.clear_color(r, g, b, 1.0);
        }
    }

    pub fn clean_resources(&self, gl: &glow::Context) {
        for mesh in &self.resource_manager.meshes {
            mesh.delete(gl);
        }
        for texture in &self.resource_manager.textures {
            texture.delete(gl);
        }
    }
}

fn render_node(
    gl: &glow::Context,
    node_index: usize,
    resources: &ResourceManager,
    shader: &mut ShaderManager,
    parent_transform: &Transform,
) {
    let node = resources.get_node(node_index);

    let world_tranform = parent_transform.combine(&node.transform);

    if let Some(mesh_index) = node.mesh {
        shader
            .set_transform_uniform(&world_tranform.to_mat())
            .update_per_node_uniforms(gl);

        render_mesh(gl, resources, shader, resources.get_mesh(mesh_index));
    }
    node.children.iter().for_each(|child| {
        render_node(gl, *child, resources, shader, &world_tranform);
    });
}

fn render_mesh(
    gl: &glow::Context,
    resources: &ResourceManager,
    shader: &mut ShaderManager,
    mesh: &Mesh,
) {
    shader.bind_shader(gl);

    mesh.get_primitives().iter().for_each(|primitive| {
        let mut material = &Material::default();
        if let Some(index) = primitive.get_material() {
            material = resources.get_material(index);
        }

        shader.update_material_uniforms(material);

        if let Some(index) = material.base_color_texture {
            let texture = resources.get_texture(index);
            shader.set_base_texture_uniform(texture);
        }

        if let Some(index) = material.metallic_roughness_texture {
            let texture = resources.get_texture(index);
            shader.set_metallic_map_uniform(texture);
        }

        shader.update_per_primitive_uniforms(gl);

        primitive.draw(gl);
    });
}
