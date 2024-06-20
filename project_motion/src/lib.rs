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
    type Item = Option<ThrowObject>;

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate new position and velocity
        let g = 9.8; // Gravity constant
        let dt = 1.0; // Time step

        // Update velocities
        self.actual_velocity.x += self.init_velocity.x * dt;
        self.actual_velocity.y += self.init_velocity.y * dt - g * dt;

        // Update positions
        self.actual_position.x += self.actual_velocity.x * dt;
        self.actual_position.y += self.actual_velocity.y * dt;

        // Check if the object has hit the ground
        if self.actual_position.y <= 0.0 {
            return None; // Object has reached the floor
        }

        // Increment time
        self.time += dt;

        Some(Some(ThrowObject {
            init_position: self.init_position.clone(),
            init_velocity: self.init_velocity.clone(),
            actual_position: self.actual_position.clone(),
            actual_velocity: self.actual_velocity.clone(),
            time: self.time,
        }))
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
