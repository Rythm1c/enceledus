use math::mat4::Mat4;

#[derive(Clone)]
pub struct Skin {
    pub id: usize,
    // node ids for the skin
    pub joints: Vec<usize>,
    pub inverse_bind_posses: Option<Vec<Mat4>>,
    pub skeleton: Option<usize>,
}

impl Skin {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            joints: Vec::new(),
            inverse_bind_posses: None,
            skeleton: None,
        }
    }
}
