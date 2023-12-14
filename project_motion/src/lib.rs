const GRAVITY: f32 = 9.8;

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
        let actual_position = init_position.clone();
        let actual_velocity = init_velocity.clone();

        ThrowObject {
            init_position,
            init_velocity,
            actual_position,
            actual_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.actual_position.y <= 0.0 {
            return None;
        }

        self.time += 1.0;

        self.actual_velocity.x = self.init_velocity.x;
        self.actual_velocity.y = (self.init_velocity.y - GRAVITY * self.time).round_to(3);

        self.actual_position.x = (self.init_position.x + self.init_velocity.x * self.time).round_to(3);
        self.actual_position.y = (self.init_position.y + self.init_velocity.y * self.time - 0.5 * GRAVITY * self.time * self.time).round_to(3);

        if self.actual_position.y <= 0.0 {
            self.actual_position.y = 0.0;  // Ensure y doesn't go below 0 due to floating point inaccuracies.
            return None;
        }

        Some(self.clone())
    }
}

trait RoundTo {
    fn round_to(&self, places: u32) -> f32;
}

impl RoundTo for f32 {
    fn round_to(&self, places: u32) -> f32 {
        let multiplier = 10f32.powi(places as i32);
        (self * multiplier).round() / multiplier
    }
}