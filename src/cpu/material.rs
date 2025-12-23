/// CPU representation of a material
/// pbr material with textures and factors
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Material {
    pub id: Option<usize>,
    pub ao: f32,
    pub base_color_factor: [f32; 4],
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    //pub emissive_factor: [f32; 3],

    // textures
    pub base_color_texture: Option<usize>,
    pub metallic_roughness_texture: Option<usize>,
    /*
    pub normal_texture: Option<usize>,
    pub occlusion_texture: Option<usize>,
    pub emissive_texture: Option<usize>, */
}

impl Material {
    pub fn new() -> Self {
        Self {
            id: None,
            ao: 1.0,
            base_color_factor: [1.0, 1.0, 1.0, 1.0],
            metallic_factor: 0.5,
            roughness_factor: 0.5,
            //emissive_factor: [0.0, 0.0, 0.0],
            base_color_texture: None,
            metallic_roughness_texture: None,
            /*
            normal_texture: None,
            occlusion_texture: None,
            emissive_texture: None, */
        }
    }
}
