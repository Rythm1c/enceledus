use crate::src::cpu::primitive::Primitive;
use glow::HasContext;

#[derive(Clone)]
pub struct GpuPrimitive {
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub ebo: Option<glow::NativeBuffer>,
    pub index_count: u32,
    pub vertex_count: u32,
}

impl GpuPrimitive {
    // upload CPU mesh data to GPU and create VAO, VBO, EBO
    pub fn from_cpu(gl: &glow::Context, cpu: &Primitive) -> Self {
        unsafe {
            let vao = gl.create_vertex_array().expect("could not create vao");
            let vbo = gl.create_buffer().expect("could not create vbo");
            let mut ebo: Option<glow::NativeBuffer> = None;

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            //let attributes = cpu.attributes.clone();

            let mut buffer: Vec<f32> = Vec::new();

            // interleaving vertex attributes to use one vbo
            // for now just use dummy values

            for i in 0..cpu.vertex_count() {
                buffer.extend_from_slice(&cpu.positions[i]);

                if let Some(normals) = &cpu.normals {
                    buffer.extend_from_slice(&normals[i]);
                } else {
                    buffer.extend_from_slice(&[1.0, 1.0, 1.0]);
                }

                if let Some(uvs) = &cpu.uvs {
                    buffer.extend_from_slice(&uvs[i]);
                } else {
                    buffer.extend_from_slice(&[0.0, 0.0]);
                }

                if let Some(weights) = &cpu.weights {
                    buffer.extend_from_slice(&weights[i]);
                } else {
                    buffer.extend_from_slice(&[0.0, 0.0, 0.0, 0.0]);
                }

                if let Some(joints) = &cpu.joints {
                    for j in &joints[i] {
                        buffer.push(*j as f32);
                    }
                } else {
                    buffer.extend_from_slice(&[0.0, 0.0, 0.0, 0.0]);
                }
            }

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&buffer),
                glow::STATIC_DRAW,
            );

            let mut offset = 0;
            let stride = Self::compute_stride() as i32;

            // vao attributes setup
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, offset);
            offset += 3 * std::mem::size_of::<f32>() as i32;

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, stride, offset);
            offset += 3 * std::mem::size_of::<f32>() as i32;

            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, stride, offset);
            offset += 2 * std::mem::size_of::<f32>() as i32;

            //bone weights
            gl.enable_vertex_attrib_array(3);
            gl.vertex_attrib_pointer_f32(3, 4, glow::FLOAT, false, stride, offset);
            offset += 4 * std::mem::size_of::<f32>() as i32;

            //bone ids
            gl.enable_vertex_attrib_array(4);
            gl.vertex_attrib_pointer_f32(4, 4, glow::FLOAT, false, stride, offset);
            // offset += 4 * std::mem::size_of::<i32>() as i32;

            if let Some(indices) = &cpu.indices {
                let ibo = gl.create_buffer().expect("create EBO");
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
                gl.buffer_data_u8_slice(
                    glow::ELEMENT_ARRAY_BUFFER,
                    bytemuck::cast_slice(indices),
                    glow::STATIC_DRAW,
                );
                ebo = Some(ibo);
            }

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);

            Self {
                vao,
                vbo,
                ebo,
                index_count: cpu.index_count() as u32,
                vertex_count: cpu.vertex_count() as u32,
            }
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            if let Some(_ebo) = self.ebo {
                gl.draw_elements(
                    glow::TRIANGLES,
                    self.index_count as i32,
                    glow::UNSIGNED_INT,
                    0,
                );
            } else {
                gl.draw_arrays(glow::TRIANGLES, 0, self.vertex_count as i32);
            }
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);
            if let Some(ebo) = self.ebo {
                gl.delete_buffer(ebo);
            }
            gl.delete_vertex_array(self.vao);
        }
    }

    fn compute_stride() -> usize {
        let mut stride = 3 * std::mem::size_of::<f32>(); // positions
        stride += 3 * std::mem::size_of::<f32>(); // normals
        stride += 2 * std::mem::size_of::<f32>(); // uvs
        stride += 4 * std::mem::size_of::<f32>(); /* weights */
        stride += 4 * std::mem::size_of::<i32>(); /* joint ids */

        stride
    }
}
