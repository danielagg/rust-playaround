use std::fmt::Display;

use super::area::Area;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
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
