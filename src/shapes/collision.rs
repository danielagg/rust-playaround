pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;
    fn collides(&self, other: &[T]) -> bool {
        for item in other {
            if self.collide(item) {
                return true;
            }
        }

        return false;
    }
}

pub struct PointIter {
    points: Vec<(f64, f64)>,
    index: usize,
}

impl Iterator for PointIter {
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

impl From<Vec<(f64, f64)>> for PointIter {
    fn from(points: Vec<(f64, f64)>) -> Self {
        return PointIter {
            points: points,
            index: 0,
        };
    }
}

pub trait Points {
    fn points(&self) -> PointIter;
}

pub trait Contains {
    fn contains_point(&self, point: (f64, f64)) -> bool;
}

// T is other, V is self
impl<T, V> Collidable<T> for V
where
    T: Points,
    V: Contains,
{
    fn collide(&self, other: &T) -> bool {
        for point in other.points() {
            if self.contains_point(point) {
                return true;
            }
        }

        return false;
    }
}
