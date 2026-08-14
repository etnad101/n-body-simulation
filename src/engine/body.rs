use crate::engine::model::{Instance, Model};

pub struct Body {
    position: glam::Vec3,
    radius: f32,
    colour: glam::Vec3,
}

impl Body {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Self {
        Self {
            radius,
            position: glam::Vec3::new(x, y, z),
            colour: glam::Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn with_colour(mut self, r: f32, g: f32, b: f32) -> Self {
        self.colour.x = r;
        self.colour.y = g;
        self.colour.z = b;
        self
    }

    pub fn update(&mut self) {}

    pub fn create_instance(&self) -> Instance {
        Instance::new(self.position, self.radius, self.colour)
    }
}
