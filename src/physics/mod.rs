use math::transform::Transform;
/// Physics module for future physics simulation
/// Designed to integrate with the Scene and entities
use math::vec3::Vec3;

/// Physics body types
#[derive(Clone, Copy, Debug)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic,
}

/// A physics body associated with a scene entity
pub struct PhysicsBody {
    pub body_type: BodyType,
    pub mass: f32,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub gravity_scale: f32,
    pub is_enabled: bool,
}

impl PhysicsBody {
    pub fn new_static() -> Self {
        Self {
            body_type: BodyType::Static,
            mass: 0.0,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            angular_velocity: Vec3::new(0.0, 0.0, 0.0),
            gravity_scale: 1.0,
            is_enabled: true,
        }
    }

    pub fn new_dynamic(mass: f32) -> Self {
        Self {
            body_type: BodyType::Dynamic,
            mass,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            angular_velocity: Vec3::new(0.0, 0.0, 0.0),
            gravity_scale: 1.0,
            is_enabled: true,
        }
    }

    pub fn apply_force(&mut self, force: Vec3, delta_time: f32) {
        if self.mass > 0.0 && self.is_enabled {
            // F = ma => a = F/m => v += a * dt
            let acceleration = Vec3::new(
                force.x / self.mass,
                force.y / self.mass,
                force.z / self.mass,
            );
            self.velocity = Vec3::new(
                self.velocity.x + acceleration.x * delta_time,
                self.velocity.y + acceleration.y * delta_time,
                self.velocity.z + acceleration.z * delta_time,
            );
        }
    }
}

/// Physics world/engine
pub struct PhysicsWorld {
    bodies: Vec<PhysicsBody>,
    gravity: Vec3,
    delta_time: f32,
}

impl PhysicsWorld {
    pub fn new(gravity: Vec3) -> Self {
        Self {
            bodies: Vec::new(),
            gravity,
            delta_time: 0.016, // ~60 FPS default
        }
    }

    /// Add a body to the physics world
    pub fn add_body(&mut self, body: PhysicsBody) -> usize {
        let id = self.bodies.len();
        self.bodies.push(body);
        id
    }

    /// Update physics simulation (called once per frame)
    pub fn step(&mut self, delta_time: f32) {
        self.delta_time = delta_time;

        for body in &mut self.bodies {
            if !body.is_enabled {
                continue;
            }

            // Apply gravity
            if matches!(body.body_type, BodyType::Dynamic) {
                let gravity_force = Vec3::new(
                    self.gravity.x * body.mass * body.gravity_scale,
                    self.gravity.y * body.mass * body.gravity_scale,
                    self.gravity.z * body.mass * body.gravity_scale,
                );
                body.apply_force(gravity_force, delta_time);
            }

            // Update position based on velocity
            // This would integrate with the scene entities' transforms
        }
    }

    pub fn get_body(&self, index: usize) -> Option<&PhysicsBody> {
        self.bodies.get(index)
    }

    pub fn get_body_mut(&mut self, index: usize) -> Option<&mut PhysicsBody> {
        self.bodies.get_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_body_creation() {
        let static_body = PhysicsBody::new_static();
        let dynamic_body = PhysicsBody::new_dynamic(2.0);

        assert_eq!(static_body.mass, 0.0);
        assert_eq!(dynamic_body.mass, 2.0);
    }

    #[test]
    fn test_physics_world() {
        let mut world = PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0));
        let body_id = world.add_body(PhysicsBody::new_dynamic(1.0));

        assert!(world.get_body(body_id).is_some());
    }
}
