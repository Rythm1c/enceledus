#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    pub bone_ids: [i32; 4],
    pub bone_weights: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: [0.0; 3],
            normal: [0.0; 3],
            uv: [0.0, 0.0],
            color: [1.0; 4],
            bone_ids: [-1; 4],
            bone_weights: [0.0; 4],
        }
    }
}

impl Vertex {
    pub fn new(
        pos: [f32; 3],
        normal: [f32; 3],
        uv: [f32; 2],
        color: [f32; 4],
        bone_ids: [i32; 4],
        bone_weights: [f32; 4],
    ) -> Self {
        Self {
            pos,
            normal,
            uv,
            color,
            bone_ids,
            bone_weights,
        }
    }
}

