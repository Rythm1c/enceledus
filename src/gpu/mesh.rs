use crate::src::cpu::mesh::Mesh;

use super::primitive::GpuPrimitive;

#[derive(Clone)]
pub struct GpuMesh {
    primitives: Vec<GpuPrimitive>,
}

impl GpuMesh {
    pub fn from_cpu(gl: &glow::Context, cpu: &Mesh) -> Self {
        let mut primitives = Vec::new();
        cpu.primitives
            .iter()
            .for_each(|primitive| primitives.push(GpuPrimitive::from_cpu(gl, primitive)));

        Self { primitives }
    }

    pub fn get_primitive(&self, index: usize) -> &GpuPrimitive {
        &self.primitives[index]
    }

    pub fn primitive_count(&self) -> usize {
        self.primitives.len()
    }

    pub fn delete(&self, gl: &glow::Context) {
        for primitive in &self.primitives {
            primitive.delete(gl);
        }
    }
}
