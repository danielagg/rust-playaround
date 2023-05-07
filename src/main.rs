use shapes::{area::Area, circle::Circle, rectangle::Rectangle};

mod shapes;

fn main() {
    let rect_1 = Rectangle::default();
    let circle_1 = Circle::default();

    println!("{}", rect_1);
    println!("{}", circle_1);

    let my_float: f64 = 6.0;
    println!("{}", my_float.get_area());
}
