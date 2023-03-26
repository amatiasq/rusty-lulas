pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
