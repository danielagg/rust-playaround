use std::fmt::Display;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Default for Circle {
    fn default() -> Self {
        return Circle {
            x: 0.0,
            y: 0.0,
            radius: 10.0,
        };
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Circle (on points {}, {}), with the radius of: {}",
            self.x, self.y, self.radius
        );
    }
}
