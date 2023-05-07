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

pub struct RectangleIterator {
    points: Vec<(f64, f64)>,
    index: usize,
}

impl Iterator for RectangleIterator {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.points.len() {
            return None;
        }

        let point = self.points[self.index];
        self.index += 1;

        return Some(point);
    }
}

impl IntoIterator for Rectangle {
    type Item = (f64, f64);
    type IntoIter = RectangleIterator;

    fn into_iter(self) -> Self::IntoIter {
        let points = vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x + self.width, self.y + self.height),
            (self.x, self.y + self.height),
        ];

        return RectangleIterator {
            points: points,
            index: 0,
        };
    }
}