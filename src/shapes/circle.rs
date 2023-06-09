use std::f64::consts::PI;
use std::fmt::Display;

use super::area::Area;
use super::collision::{Contains, PointIter, Points};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

impl Points for Circle {
    fn points(&self) -> PointIter {
        return vec![(self.x, self.y)].into();
    }
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

impl Area for Circle {
    fn get_area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}
