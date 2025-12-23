use math::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pos: Vec3,
    col: Vec3,
}

impl PointLight {
    fn new(pos: Vec3, col: Vec3) -> Self {
        Self { pos, col }
    }

    pub fn get_pos(&self) -> Vec3 {
        self.pos
    }

    pub fn get_col(&self) -> Vec3 {
        self.col
    }
}

#[derive(Debug, Clone)]
pub struct PointLightManager {
    point_lights: Vec<PointLight>,
}

impl PointLightManager {
    pub fn new() -> Self {
        Self {
            point_lights: Vec::new(),
        }
    }

    pub fn add_point_light(&mut self, pos: Vec3, col: Vec3) {
        self.point_lights.push(PointLight::new(pos, col));
    }

    pub fn get_point_light(&self, index: usize) -> &PointLight {
        &self.point_lights[index]
    }

    pub fn update_point_light(&mut self, index: usize, pos: Vec3, col: Vec3) {
        if let Some(point_light) = self.point_lights.get_mut(index) {
            point_light.pos = pos;
            point_light.col = col;
        } else {
            panic!("point light index({}) out of bounds!", index);
        }
    }

    pub fn get_point_lights(&self) -> &Vec<PointLight> {
        &self.point_lights
    }
}
