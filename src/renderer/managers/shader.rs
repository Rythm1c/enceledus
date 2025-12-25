use crate::src::model::material::Material;
use crate::src::model::texture::Texture;
use crate::src::viewer::camera::Camera;
use crate::src::viewer::light::PointLight;
use math::{mat4::Mat4, vec3::Vec3, vec4::Vec4};
use std::collections::HashMap;

use crate::src::renderer::shader::Program;

#[derive(Clone, Copy, PartialEq)]
pub enum UniformLevel {
    Frame,
    Model,
    Node,
    Primitive,
}

#[derive(Clone)]
pub enum UniformValue {
    Int(i32),
    Float(f32),
    Vector3f(Vec3),
    Vector4f(Vec4),
    Matrix4x4(Mat4),
    Texture(Texture, u32),
}

#[derive(Clone)]
pub struct Uniform(UniformLevel, UniformValue);

impl Uniform {
    pub fn new(level: UniformLevel, value: UniformValue) -> Self {
        Self(level, value)
    }

    pub fn get_level(&self) -> UniformLevel {
        self.0
    }

    pub fn get_value(&self) -> &UniformValue {
        &self.1
    }
}

#[derive(Clone)]
pub struct ShaderManager {
    program: Program,
    uniforms: HashMap<String, Uniform>,
}

impl ShaderManager {
    pub fn new(gl: &glow::Context) -> Self {
        let vert_src = String::from("shaders/shader.vert");
        let frag_src = String::from("shaders/shader.frag");

        let program = Program::from_files(gl, &vert_src[..], &frag_src[..]);

        program.bind(gl);
        program.set_uniform_i32(gl, "albedoMap", 0);
        program.set_uniform_i32(gl, "metallicMap", 1);
        program.set_uniform_i32(gl, "normalMap", 2);

        Self {
            program,
            uniforms: HashMap::new(),
        }
    }

    pub fn bind_shader(&self, gl: &glow::Context) {
        self.program.bind(gl);
    }

    pub fn update_camera_uniforms(&mut self, camera: &Camera, ratio: f32) -> &mut Self {
        self.set_camera_pos_uniform(camera.pos)
            .set_view_uniform(&camera.get_view())
            .set_projection_uniform(&camera.get_pojection(ratio))
    }

    pub fn update_material_uniforms(&mut self, material: &Material) -> &mut Self {
        self.set_base_color_uniform(Vec4::from(&material.base_color_factor))
            .set_metallic_factor_uniform(material.metallic_factor)
            .set_roughness_uniform(material.roughness_factor)
            .set_ao_uniform(material.ao)
            .set_has_base_tex_uniform(material.base_color_texture.is_some())
            .set_has_metallic_map_uniform(material.metallic_roughness_texture.is_some())
    }

    pub fn set_transform_uniform(&mut self, value: &Mat4) -> &mut Self {
        self.set_uniform(
            "transform",
            Uniform::new(UniformLevel::Node, UniformValue::Matrix4x4(*value)),
        );
        self
    }

    pub fn set_view_uniform(&mut self, value: &Mat4) -> &mut Self {
        self.set_uniform(
            "view",
            Uniform::new(UniformLevel::Frame, UniformValue::Matrix4x4(*value)),
        );
        self
    }

    pub fn set_projection_uniform(&mut self, value: &Mat4) -> &mut Self {
        self.set_uniform(
            "projection",
            Uniform::new(UniformLevel::Frame, UniformValue::Matrix4x4(*value)),
        );
        self
    }

    pub fn set_camera_pos_uniform(&mut self, value: Vec3) -> &mut Self {
        self.set_uniform(
            "camPos",
            Uniform::new(UniformLevel::Frame, UniformValue::Vector3f(value)),
        );
        self
    }

    pub fn set_base_color_uniform(&mut self, value: Vec4) -> &mut Self {
        self.set_uniform(
            "baseColor",
            Uniform::new(UniformLevel::Primitive, UniformValue::Vector4f(value)),
        );
        self
    }

    pub fn set_metallic_factor_uniform(&mut self, value: f32) -> &mut Self {
        self.set_uniform(
            "metallicFactor",
            Uniform::new(UniformLevel::Primitive, UniformValue::Float(value)),
        );
        self
    }

    pub fn set_roughness_uniform(&mut self, value: f32) -> &mut Self {
        self.set_uniform(
            "roughness",
            Uniform::new(UniformLevel::Primitive, UniformValue::Float(value)),
        );
        self
    }

    pub fn set_ao_uniform(&mut self, value: f32) -> &mut Self {
        self.set_uniform(
            "ao",
            Uniform::new(UniformLevel::Primitive, UniformValue::Float(value)),
        );
        self
    }

    pub fn set_base_texture_uniform(&mut self, value: &Texture) -> &mut Self {
        self.set_uniform(
            "albedoMap",
            Uniform::new(UniformLevel::Primitive, UniformValue::Texture(*value, 0)),
        );
        self
    }

    pub fn set_metallic_map_uniform(&mut self, value: &Texture) -> &mut Self {
        self.set_uniform(
            "metallicMap",
            Uniform::new(UniformLevel::Primitive, UniformValue::Texture(*value, 1)),
        );
        self
    }

    pub fn set_normal_map_uniform(&mut self, value: &Texture) -> &mut Self {
        self.set_uniform(
            "normalMap",
            Uniform::new(UniformLevel::Primitive, UniformValue::Texture(*value, 2)),
        );
        self
    }

    pub fn set_animated_uniform(&mut self, value: bool) -> &mut Self {
        self.set_uniform(
            "animated",
            Uniform::new(UniformLevel::Model, UniformValue::Int(value as i32)),
        );
        self
    }

    pub fn set_has_base_tex_uniform(&mut self, value: bool) -> &mut Self {
        self.set_uniform(
            "hasBaseTexture",
            Uniform::new(UniformLevel::Primitive, UniformValue::Int(value as i32)),
        );
        self
    }

    pub fn set_has_metallic_map_uniform(&mut self, value: bool) -> &mut Self {
        self.set_uniform(
            "hasMetallicMap",
            Uniform::new(UniformLevel::Primitive, UniformValue::Int(value as i32)),
        );

        self
    }

    fn set_point_light_count_uniform(&mut self, count: i32) {
        self.set_uniform(
            "lightCount",
            Uniform::new(UniformLevel::Frame, UniformValue::Int(count)),
        );
    }
    pub fn set_point_lights_uniform(&mut self, point_lights: &Vec<PointLight>) -> &mut Self {
        self.set_point_light_count_uniform(point_lights.len() as i32);

        point_lights.iter().enumerate().for_each(|(i, pl)| {
            self.set_uniform(
                &format!("lights[{i}].position")[..],
                Uniform::new(UniformLevel::Frame, UniformValue::Vector3f(pl.get_pos())),
            );
            self.set_uniform(
                &format!("lights[{i}].color")[..],
                Uniform::new(UniformLevel::Frame, UniformValue::Vector3f(pl.get_col())),
            );
        });
        self
    }

    pub fn set_skeleton_matrices() {}

    pub fn update_per_frame_uniforms(&self, gl: &glow::Context) {
        self.uniforms.iter().for_each(|uniform| {
            if uniform.1.get_level() == UniformLevel::Frame {
                self.upload_uniform(gl, &uniform.0[..], uniform.1)
            }
        });
    }
    pub fn update_per_model_uniforms(&self, gl: &glow::Context) {
        self.uniforms.iter().for_each(|uniform| {
            if uniform.1.get_level() == UniformLevel::Model {
                self.upload_uniform(gl, &uniform.0[..], uniform.1)
            }
        });
    }

    pub fn update_per_primitive_uniforms(&self, gl: &glow::Context) {
        self.uniforms.iter().for_each(|uniform| {
            if uniform.1.get_level() == UniformLevel::Primitive {
                self.upload_uniform(gl, &uniform.0[..], uniform.1)
            }
        });
    }

    pub fn update_per_node_uniforms(&self, gl: &glow::Context) {
        self.uniforms.iter().for_each(|uniform| {
            if uniform.1.get_level() == UniformLevel::Node {
                self.upload_uniform(gl, &uniform.0[..], uniform.1)
            }
        });
    }

    fn upload_uniform(&self, gl: &glow::Context, name: &str, uniform: &Uniform) {
        match uniform.get_value() {
            UniformValue::Int(value) => {
                self.program.set_uniform_i32(gl, name, *value);
            }

            UniformValue::Float(value) => {
                self.program.set_uniform_f32(gl, name, *value);
            }

            UniformValue::Vector3f(value) => {
                self.program.set_uniform_vec3(gl, name, &value.to_array());
            }

            UniformValue::Vector4f(value) => {
                self.program.set_uniform_vec4(gl, name, &value.to_array());
            }

            UniformValue::Matrix4x4(value) => {
                self.program
                    .set_uniform_mat4(gl, name, value.data.as_flattened());
            }

            UniformValue::Texture(texture, unit) => {
                texture.bind_to_uint(gl, *unit);
            }
        }
    }

    fn set_uniform(&mut self, name: &str, value: Uniform) {
        self.uniforms
            .entry(String::from(name))
            .and_modify(|v| *v = value.clone())
            .or_insert(value);
    }
}
