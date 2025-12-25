use math::mat4::Mat4;

#[derive(Clone)]
pub struct Skin {
    // node ids for the skin
    pub joints: Vec<usize>,
    pub inverse_bind_posses: Option<Vec<Mat4>>,
    pub skeleton: Option<usize>,
}

impl Skin {
    pub fn new() -> Self {
        Self {
            joints: Vec::new(),
            inverse_bind_posses: None,
            skeleton: None,
        }
    }

    pub fn from_gltf(skin: &gltf::Skin) -> Self {
        let mut skeleton: Option<usize> = None;
        if let Some(s) = skin.skeleton() {
            skeleton = Some(s.index());
        }

        Self {
            joints: skin
                .joints()
                .map(|joint| joint.index())
                .collect::<Vec<usize>>(),
            inverse_bind_posses: None,
            skeleton,
        }
    }
}
