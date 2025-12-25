use super::loader::GltfFile;

use super::vertex::Vertex;
use glow::HasContext;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    min: [f32; 3],
    max: [f32; 3],
}

#[derive(Clone)]
pub struct Primitive {
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    ebo: Option<glow::NativeBuffer>,

    draw_mode: u32,

    material: Option<usize>,

    index_count: u32,
    vertex_count: u32,

    bounds: BoundingBox,
}

impl Primitive {
    // upload CPU mesh data to GPU and create VAO, VBO, EBO

    pub fn from_gltf(gl: &glow::Context, primitive: &gltf::Primitive, file: &GltfFile) -> Self {
        let data = primitive_data_from_gltf(primitive, file);

        Self::upload(gl, &data)
    }

    fn upload(gl: &glow::Context, data: &PrimitiveData) -> Self {
        unsafe {
            let vao = gl.create_vertex_array().expect("could not create vao");
            let vbo = gl.create_buffer().expect("could not create vbo");
            let mut ebo: Option<glow::NativeBuffer> = None;

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&data.vertices),
                glow::STATIC_DRAW,
            );

            Vertex::set_attributes(gl);

            let vertex_count = data.vertices.len() as u32;
            let mut index_count = 0;
            if let Some(indices) = &data.indices {
                let ibo = gl.create_buffer().expect("create EBO");
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
                gl.buffer_data_u8_slice(
                    glow::ELEMENT_ARRAY_BUFFER,
                    bytemuck::cast_slice(indices),
                    glow::STATIC_DRAW,
                );
                ebo = Some(ibo);

                index_count = indices.len() as u32;
            }

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);

            Self {
                bounds: data.bounds,
                vao,
                vbo,
                ebo,
                draw_mode: data.mode,
                material: data.material,
                index_count,
                vertex_count,
            }
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            if self.ebo.is_some() {
                gl.draw_elements(
                    self.draw_mode,
                    self.index_count as i32,
                    glow::UNSIGNED_INT,
                    0,
                );
            } else {
                gl.draw_arrays(self.draw_mode, 0, self.vertex_count as i32);
            }
            gl.bind_vertex_array(None);
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);

            gl.delete_vertex_array(self.vao);

            if let Some(ebo) = self.ebo {
                gl.delete_buffer(ebo);
            }
        }
    }

    pub fn get_vert_count(&self) -> usize {
        self.vertex_count as usize
    }

    pub fn get_index_count(&self) -> usize {
        self.index_count as usize
    }

    pub fn get_min_bounds(&self) -> [f32; 3] {
        self.bounds.min
    }

    pub fn get_max_bounds(&self) -> [f32; 3] {
        self.bounds.max
    }

    pub fn get_material(&self) -> Option<usize> {
        self.material
    }
}

struct PrimitiveData {
    vertices: Vec<Vertex>,
    indices: Option<Vec<u32>>,
    material: Option<usize>,
    mode: u32,
    bounds: BoundingBox,
}

fn primitive_data_from_gltf(primitive: &gltf::Primitive, file: &GltfFile) -> PrimitiveData {
    let mut material: Option<usize> = None;

    if let Some(mat) = primitive.material().index() {
        material = Some(mat);
    }
    let mode = primitive.mode().as_gl_enum();

    let mut vertices = Vec::new();
    let mut indices: Option<Vec<u32>> = None;

    let reader = primitive.reader(|buffer| Some(&file.get_buffers()[buffer.index()]));

    let bounds = BoundingBox {
        min: primitive.bounding_box().min,
        max: primitive.bounding_box().max,
    };

    if let Some(positions) = reader.read_positions() {
        positions.for_each(|position| {
            vertices.push(Vertex {
                position,
                ..Default::default()
            })
        });
    } else {
        panic!(
            "a Primitive in folder {} does not contain the position attribute!",
            file.get_folder()
        );
    }

    if let Some(normals) = reader.read_normals() {
        normals
            .enumerate()
            .for_each(|(i, normal)| vertices[i].normal = normal);
    }

    if let Some(tex_coords) = reader.read_tex_coords(0) {
        tex_coords
            .into_f32()
            .enumerate()
            .for_each(|(i, uv)| vertices[i].uv = uv);
    }

    if let Some(weights) = reader.read_weights(0) {
        weights
            .into_f32()
            .enumerate()
            .for_each(|(i, weight_bach)| vertices[i].weights = weight_bach);
    }

    if let Some(joints) = reader.read_joints(0) {
        joints.into_u16().enumerate().for_each(|(i, joint_batch)| {
            vertices[i].joints = joint_batch.map(|joint| joint as i32)
        });
    }

    if let Some(indices_reader) = reader.read_indices() {
        indices = Some(indices_reader.into_u32().collect::<Vec<u32>>());
    }

    PrimitiveData {
        vertices,
        indices,
        material,
        mode,
        bounds,
    }
}
