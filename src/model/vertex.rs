use bytemuck::{Pod, Zeroable, offset_of};
use glow::HasContext;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub weights: [f32; 4],
    pub joints: [i32; 4],
}

impl Vertex {
    pub fn set_attributes(gl: &glow::Context) {
        unsafe {
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                size_of::<Self>() as i32,
                offset_of!(Self, position) as i32,
            );

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                size_of::<Self>() as i32,
                offset_of!(Self, normal) as i32,
            );

            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                size_of::<Self>() as i32,
                offset_of!(Self, uv) as i32,
            );

            //bone weights
            gl.enable_vertex_attrib_array(3);
            gl.vertex_attrib_pointer_f32(
                3,
                4,
                glow::FLOAT,
                false,
                size_of::<Self>() as i32,
                offset_of!(Self, weights) as i32,
            );

            //bone ids
            gl.enable_vertex_attrib_array(4);
            gl.vertex_attrib_pointer_i32(
                4,
                4,
                glow::FLOAT,
                size_of::<Self>() as i32,
                offset_of!(Self, joints) as i32,
            );
        }
    }
}
