use std::fmt::Display;

use super::area::Area;
use super::collision::{Contains, PointIter, Points};

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Contains for Rectangle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && self.x + self.width >= x && self.y <= y && self.y + self.height >= y;
    }
}

impl Points for Rectangle {
    fn points(&self) -> PointIter {
        return vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x + self.width, self.y + self.height),
            (self.x, self.y + self.height),
        ]
        .into();
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        return Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle (on points {}, {}), with the width*height of: {}*{}",
            self.x, self.y, self.width, self.height
        );
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f64 {
        return self.width * self.height;
    }
}
