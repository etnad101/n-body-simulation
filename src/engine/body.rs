use crate::engine::model::{Instance, Model};

#[derive(Clone)]
pub struct Body {
    position: glam::Vec3,
    colour: glam::Vec3,
    velocity: glam::Vec3,
    radius: f32,
    mass: f32,
}

impl Body {
    pub fn new(x: f32, y: f32, z: f32, radius: f32, mass: f32) -> Self {
        let four_thirds = 4.0 / 3.0;
        let volume = four_thirds * std::f32::consts::PI * radius.powi(3);
        Self {
            radius,
            mass,
            position: glam::Vec3::new(x, y, z),
            colour: glam::Vec3::new(1.0, 1.0, 1.0),
            velocity: glam::Vec3::ZERO,
        }
    }

    pub fn with_colour(mut self, r: f32, g: f32, b: f32) -> Self {
        self.colour.x = r;
        self.colour.y = g;
        self.colour.z = b;
        self
    }

    pub fn with_velocity(mut self, x: f32, y: f32, z: f32) -> Self {
        self.velocity.x = x;
        self.velocity.y = y;
        self.velocity.z = z;
        self
    }

    pub fn calculate_gravity_force(&self, other: &Body) -> glam::Vec3 {
        const GRAVITY_CONSTANT: f32 = 0.1;
        const EPSILON: f32 = 0.1;
        let mass = self.mass * other.mass;
        let dir = other.position - self.position;
        let bottom = (dir.length_squared() + EPSILON * EPSILON).powf(0.75);
        let force = mass * dir / bottom;
        force * GRAVITY_CONSTANT
    }

    pub fn accelerate(&mut self, force: glam::Vec3) {
        self.velocity += force / self.mass;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
        println!("Position: {}", self.position);
    }

    pub fn create_instance(&self) -> Instance {
        Instance::new(self.position, self.radius, self.colour)
    }
}
