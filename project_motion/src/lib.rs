#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        let g = 9.8; // Gravity constant
        let dt = 1.0; // Time step

        // Update positions
        self.actual_position.x += self.actual_velocity.x * dt;
        self.actual_position.y += self.actual_velocity.y * dt - 0.5 * g * dt * dt;

        // Update velocities
        self.actual_velocity.y += -g * dt;

        // Increment time
        self.time += dt;

        // Check if the object has hit the ground
        if self.actual_position.y <= 0.0 {
            self.actual_position.y = 0.0; // Ensure the position is exactly on the ground
            return None; // Object has reached the floor
        }

        Some(self.clone())
    }
}

fn main() {
    let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
}
