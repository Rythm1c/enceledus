pub mod camera;
pub mod light;

use std::path::Path;

use camera::Camera;
use light::PointLightManager;

use math::{
    mat4::Mat4,
    quaternion::Quat,
    transform::Transform,
    vec3::{Vec3, vec3},
};

use crate::src::{
    model::{loader::GltfFile, scene::Scene},
    renderer::Renderer,
    viewer::{camera::Direction, light::PointLight},
};

#[derive(Clone)]
pub struct Viewer {
    pub camera: Camera,
    point_light_manager: PointLightManager,
    pub transform: Transform, //for the whole scene
    pub scenes: Vec<Scene>,
    pub current_scene: usize,
    pub renderer: Renderer,
}

impl Viewer {
    pub fn new(gl: &glow::Context, folder: &Path) -> Self {
        let mut point_light_manager = PointLightManager::new();

        //default
        point_light_manager
            .add_point_light(vec3(-40.0, 10.0, -40.0), vec3(300.0, 300.0, 0.0))
            .add_point_light(vec3(-40.0, 10.0, 40.0), vec3(300.0, 0.0, 300.0))
            .add_point_light(vec3(40.0, 10.0, -40.0), vec3(0.0, 300.0, 300.0))
            .add_point_light(vec3(40.0, 10.0, 40.0), vec3(300.0, 300.0, 300.0));

        let mut transform = Transform::DEFAULT;
        transform.orientation = Quat::create(180.0, vec3(0.0, 1.0, 0.0));
        transform.translation = vec3(0.0, 2.0, 5.0);

        let file = GltfFile::load_gltf(folder).unwrap();
        let scenes = file
            .get_document()
            .scenes()
            .map(|scene| Scene::from_gltf(&scene))
            .collect::<Vec<Scene>>();

        Self {
            camera: Camera::default(),
            point_light_manager,
            transform,
            scenes,
            current_scene: 0,
            renderer: Renderer::new(gl, &file),
        }
    }

    pub fn update(&mut self) {
        self.camera.update_motion();
    }

    pub fn run_renderer(&mut self, gl: &glow::Context, window_ratio: f32) {
        let camera = self.camera;
        let scene = self.get_current_scene().clone();

        self.renderer.render(
            gl,
            window_ratio,
            &scene,
            &camera,
            self.point_light_manager.get_point_lights(),
            &self.transform,
        );
    }

    pub fn set_scene(&mut self, index: usize) {
        if !(index >= self.scenes.len()) {
            self.current_scene = index;
        } else {
            panic!("scene index {} out of scope!", index);
        }
    }

    pub fn get_current_scene(&self) -> &Scene {
        &self.scenes[self.current_scene]
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

    pub fn scale_transform(&mut self, scale: Vec3) {
        self.transform.scaling = scale;
    }

    pub fn translate_tranform(&mut self, pos: Vec3) {
        self.transform.translation = pos;
    }

    pub fn rotate_tranform(&mut self, rotation: Quat) {
        self.transform.orientation = rotation;
    }

    pub fn get_transform(&self) -> Transform {
        self.transform
    }

    pub fn rotate_camera(&mut self, x: i32, y: i32) {
        self.camera.rotate(x, y);
    }

    pub fn set_camera_dir(&mut self, dir: Direction) {
        self.camera.dir = dir;
    }
}
