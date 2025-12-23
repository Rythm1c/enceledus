pub mod camera;
pub mod light;

use camera::Camera;
use light::PointLightManager;

use math::{mat4::Mat4, quaternion::Quat, transform::Transform, vec3::Vec3};

use crate::src::scene::{camera::Direction, light::PointLight};

#[derive(Clone)]
pub struct Scene {
    pub camera: Camera,
    point_light_manager: PointLightManager,
    pub transfrom: Transform, //for the whole scene
    pub nodes: Vec<usize>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            point_light_manager: PointLightManager::new(),
            transfrom: Transform::DEFAULT,
            nodes: Vec::new(),
        }
    }

    pub fn get_cam_pos(&self) -> Vec3 {
        self.camera.pos
    }

    pub fn get_cam_view(&self) -> Mat4 {
        self.camera.get_view()
    }

    pub fn get_cam_projection(&self, ratio: f32) -> Mat4 {
        self.camera.get_pojection(ratio)
    }

    pub fn get_point_lights(&self) -> &Vec<PointLight> {
        self.point_light_manager.get_point_lights()
    }

    pub fn get_node(&self, index: usize) -> &usize {
        &self.nodes[index]
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn set_nodes(&mut self, nodes: Vec<usize>) {
        self.nodes = nodes;
    }

    pub fn scale_transform(&mut self, scale: Vec3) {
        self.transfrom.scaling = scale;
    }

    pub fn translate_tranform(&mut self, pos: Vec3) {
        self.transfrom.translation = pos;
    }

    pub fn rotate_tranform(&mut self, rotation: Quat) {
        self.transfrom.orientation = rotation;
    }

    pub fn get_transform(&self) -> Transform {
        self.transfrom
    }

    pub fn rotate_camera(&mut self, x: i32, y: i32) {
        self.camera.rotate(x, y);
    }

    pub fn set_camera_dir(&mut self, dir: Direction) {
        self.camera.dir = dir;
    }
}
