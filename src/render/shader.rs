use std::fs;

use glow::HasContext;

pub struct Shader {
    id: glow::NativeProgram,
}

impl Shader {
    pub fn from_files(gl: &glow::Context, vert_src: &str, frag_src: &str) -> Self {
        let vert_src = fs::read_to_string(vert_src).expect("failed to read vertex shader");
        let frag_src = fs::read_to_string(frag_src).expect("failed to read vertex shader");

        Self::from_src(gl, &vert_src, &frag_src)
    }

    pub fn from_src(gl: &glow::Context, vert_src: &str, frag_src: &str) -> Self {
        unsafe {
            let program = gl.create_program().expect("cannot create program");

            let vs = gl
                .create_shader(glow::VERTEX_SHADER)
                .expect("cannot create vertex shader");
            gl.shader_source(vs, vert_src);
            gl.compile_shader(vs);
            if !gl.get_shader_compile_status(vs) {
                panic!(
                    "failed to cmpile vertex shader: {}",
                    gl.get_shader_info_log(vs)
                );
            }

            let fs = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .expect("cannot create vertex shader");
            gl.shader_source(fs, frag_src);
            gl.compile_shader(fs);
            if !gl.get_shader_compile_status(fs) {
                panic!(
                    "failed to cmpile vertex shader: {}",
                    gl.get_shader_info_log(fs)
                );
            }

            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);

            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                panic!(
                    "failed to link shader program: {}",
                    gl.get_program_info_log(program)
                );
            }

            gl.delete_shader(vs);
            gl.delete_shader(fs);

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

    pub fn set_uniform_f32(&self, gl: &glow::Context, name: &str, value: i32) {
        unsafe {
            let loc = gl.get_uniform_location(self.id, name);

            if let Some(loc) = loc {
                gl.uniform_1_i32(Some(&loc), value);
            }
        }
    }

    pub fn set_uniform_mat4(&self, gl: &glow::Context, name: &str, mat: &[f32; 16]) {
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
}
