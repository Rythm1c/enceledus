#[derive(Debug, Clone)]
pub struct Primitive {
    pub positions: Vec<[f32; 3]>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub uvs: Option<Vec<[f32; 2]>>,
    pub weights: Option<Vec<[f32; 4]>>,
    pub joints: Option<Vec<[i32; 4]>>,
    pub indices: Option<Vec<u32>>,
    pub material: usize,
}

impl Primitive {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: None,
            uvs: None,
            joints: None,
            weights: None,
            indices: None,
            material: 0,
        }
    }

    pub fn set_positions(&mut self, data: Vec<[f32; 3]>) {
        self.positions = data;
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = Some(indices);
    }

    pub fn set_normals(&mut self, data: Vec<[f32; 3]>) {
        self.normals = Some(data);
    }

    pub fn set_uvs(&mut self, data: Vec<[f32; 2]>) {
        self.uvs = Some(data);
    }

    pub fn set_bone_ids(&mut self, data: Vec<[i32; 4]>) {
        self.joints = Some(data);
    }

    pub fn set_bone_weights(&mut self, data: Vec<[f32; 4]>) {
        self.weights = Some(data);
    }

    pub fn set_material(&mut self, material_index: usize) {
        self.material = material_index;
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    pub fn index_count(&self) -> usize {
        match &self.indices {
            Some(indices) => indices.len(),
            None => 0,
        }
    }

    // get smallest component and largest component of the position attrinbuts
    pub fn get_bounds(&self) -> (f32, f32) {
        let minimum = self
            .positions
            .iter()
            .map(|position| position[0].min(position[1]).min(position[2]))
            .reduce(f32::min)
            .expect("could not find smallest component in primitive!");

        let maximum = self
            .positions
            .iter()
            .map(|position| position[0].max(position[1]).max(position[2]))
            .reduce(f32::max)
            .expect("could not find largest component in primitive!");

        (minimum, maximum)
    }
}
