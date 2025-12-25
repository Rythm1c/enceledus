
/// GPU-side animation skeleton
/// Stores bone matrices ready for GPU skinning
pub struct Skeleton {
    pub id: usize,
    pub bone_matrices: Vec<math::mat4::Mat4>,
    pub bone_count: usize,
}

impl Skeleton {
    pub fn new(id: usize, bone_count: usize) -> Self {
        Self {
            id,
            bone_matrices: vec![math::mat4::Mat4::IDENTITY; bone_count],
            bone_count,
        }
    }

    /// Update a bone's transform matrix
    pub fn set_bone_matrix(&mut self, bone_index: usize, matrix: math::mat4::Mat4) {
        if bone_index < self.bone_matrices.len() {
            self.bone_matrices[bone_index] = matrix;
        }
    }

    /// Get all bone matrices for GPU upload
    pub fn get_bone_matrices(&self) -> &[math::mat4::Mat4] {
        &self.bone_matrices
    }
}

/// Links a GPU model with its skeleton for animation
/// This is what you render when the model is animated
pub struct GPUAnimatedModel {}