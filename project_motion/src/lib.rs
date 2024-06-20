#[derive(Debug, Clone, PartialEq, Copy)]
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
            init_position,
            init_velocity,
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

const GRAVITY: f32 = 9.8;

impl ThrowObject {
    fn calculate_next_position(&self, time: f32) -> Object {
        Object {
            x: ((self.init_position.x + self.init_velocity.x * time) * 10.0).ceil() / 10.0,
            y: ((self.init_position.y
                + self.init_velocity.y * time
                + (-(1.0 * GRAVITY / 2.0) * (time * time)))
                * 10.0)
                .ceil()
                / 10.0,
        }
    }

    fn calculate_next_velocity(&self, time: f32) -> Object {
        Object {
            x: self.init_velocity.x,
            y: ((self.init_velocity.y - GRAVITY * time) * 10.0).ceil() / 10.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1.0;
        self.actual_position = self.calculate_next_position(self.time);
        self.actual_velocity = self.calculate_next_velocity(self.time);

        if self.actual_position.y <= 0.0 {
            return None;
        }

        Some(ThrowObject {
            init_position: self.init_position,
            init_velocity: self.init_velocity,
            actual_position: self.actual_position,
            actual_velocity: self.actual_velocity,
            time: self.time,
        })
    }
}

// fn main() {
//     let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });
//     println!("{:?}", obj.next());
//     println!("{:?}", obj.next());
//     println!("{:?}", obj.next());
//     println!("{:?}", obj.next());
//     println!("{:?}", obj.next());
// }