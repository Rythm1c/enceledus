use glow::HasContext;

use std::fs;

#[derive(Clone)]
pub struct Program {
    id: glow::NativeProgram,
}

impl Program {
    pub fn from_files(gl: &glow::Context, vert_src: &str, frag_src: &str) -> Self {
        let vert_src = fs::read_to_string(vert_src).expect("failed to read vertex shader");
        let frag_src = fs::read_to_string(frag_src).expect("failed to read fragment shader");

        Self::from_src(
            gl,
            [
                (&vert_src[..], glow::VERTEX_SHADER),
                (&frag_src[..], glow::FRAGMENT_SHADER),
            ]
            .as_ref(),
        )
    }

    pub fn from_src(gl: &glow::Context, shaders: &[(&str, u32)]) -> Self {
        unsafe {
            let program = gl.create_program().expect("cannot create program");

            for src in shaders {
                let shader = Shader::from_shader(gl, src.0, src.1);
                gl.attach_shader(program, shader.id);
                shader.delete(gl);
            }

            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                panic!(
                    "failed to link shader program: {}",
                    gl.get_program_info_log(program)
                );
            }

            Self { id: program }
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.id));
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.id);
        }
    }

    pub fn set_uniform_i32(&self, gl: &glow::Context, name: &str, value: i32) {
        unsafe {
            let loc = gl.get_uniform_location(self.id, name);

            if let Some(loc) = loc {
                gl.uniform_1_i32(Some(&loc), value);
            }
        }
    }

    pub fn set_uniform_f32(&self, gl: &glow::Context, name: &str, value: f32) {
        unsafe {
            let loc = gl.get_uniform_location(self.id, name);

            if let Some(loc) = loc {
                gl.uniform_1_f32(Some(&loc), value);
            }
        }
    }

    pub fn set_uniform_mat4(&self, gl: &glow::Context, name: &str, mat: &[f32]) {
        unsafe {
            let loc = gl.get_uniform_location(self.id, name);

            if let Some(loc) = loc {
                gl.uniform_matrix_4_f32_slice(Some(&loc), true, mat);
            }
        }
    }

    pub fn set_uniform_vec3(&self, gl: &glow::Context, name: &str, vec: &[f32; 3]) {
        unsafe {
            let loc = gl.get_uniform_location(self.id, name);

            if let Some(loc) = loc {
                gl.uniform_3_f32(Some(&loc), vec[0], vec[1], vec[2]);
            }
        }
    }

    pub fn set_uniform_vec4(&self, gl: &glow::Context, name: &str, vec: &[f32; 4]) {
        unsafe {
            let loc = gl.get_uniform_location(self.id, name);

            if let Some(loc) = loc {
                gl.uniform_4_f32(Some(&loc), vec[0], vec[1], vec[2], vec[3]);
            }
        }
    }
}

#[derive(Clone)]
struct Shader {
    id: glow::NativeShader,
}

impl Shader {
    pub fn from_shader(gl: &glow::Context, src: &str, stage: u32) -> Self {
        unsafe {
            let id = gl.create_shader(stage).expect("cannot create shader");
            gl.shader_source(id, src);
            gl.compile_shader(id);
            if !gl.get_shader_compile_status(id) {
                panic!("failed to compile shader: {}", gl.get_shader_info_log(id));
            }

            Self { id }
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_shader(self.id);
        }
    }
}
