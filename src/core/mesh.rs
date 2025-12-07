
#[derive(Clone)]
pub struct CPUMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub uvs: Option<Vec<[f32; 2]>>,
    // pub colors: Option<Vec<[f32; 4]>>,
    pub joints: Option<Vec<[i32; 4]>>,
    pub weights: Option<Vec<[f32; 4]>>,
    pub indices: Option<Vec<u32>>,
    pub material: Option<usize>,
    pub name: Option<String>,
    pub attributes: MeshAttributes,
}
impl CPUMesh {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: None,
            uvs: None,
            // colors: None,

            // joint ids that affect each vertex
            joints: None,

            // joint weights for skinning
            weights: None,
            indices: None,
            material: None,
            name: None,
            attributes: MeshAttributes::default(),
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

    /*  pub fn set_colors(&mut self, data: Vec<[f32; 4]>) {
        self.colors = Some(data);
    } */

    pub fn set_bone_ids(&mut self, data: Vec<[i32; 4]>) {
        self.joints = Some(data);
    }

    pub fn set_bone_weights(&mut self, data: Vec<[f32; 4]>) {
        self.weights = Some(data);
    }

    pub fn set_material(&mut self, material_index: usize) {
        self.material = Some(material_index);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
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

    pub fn detect_attributes(&self) -> MeshAttributes {
        MeshAttributes {
            has_normal: self.normals.is_some(),
            has_uv: self.uvs.is_some(),
            has_skinning: self.joints.is_some() && self.weights.is_some(),
            has_indices: self.indices.is_some(),
        }
    }

    // compute stride based on attributes
    pub fn compute_stride(&self) -> usize {
        let mut stride = 3 * std::mem::size_of::<f32>(); // positions
        if self.attributes.has_normal {
            stride += 3 * std::mem::size_of::<f32>();
        }
        if self.attributes.has_uv {
            stride += 2 * std::mem::size_of::<f32>();
        }

        if self.attributes.has_skinning {
            stride += 4 * std::mem::size_of::<f32>(); /* weights */
            stride += 4 * std::mem::size_of::<i32>(); /* joint ids */
        }
        stride
    }
}

#[derive(Clone)]

pub struct MeshAttributes {
    pub has_uv: bool,
    pub has_normal: bool,
    pub has_skinning: bool,
    pub has_indices: bool,
}

impl Default for MeshAttributes {
    fn default() -> Self {
        Self {
            has_uv: false,
            has_normal: false,
            has_skinning: false,
            has_indices: false,
        }
    }
}
