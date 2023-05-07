pub trait Area {
    fn get_area(&self) -> f64;
}

impl Area for f64 {
    fn get_area(&self) -> f64 {
        return self * 10.0;
    }
}
