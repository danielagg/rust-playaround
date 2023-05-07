use shapes::{circle::Circle, rectangle::Rectangle};

mod shapes;

fn main() {
    let rect_1 = Rectangle::default();
    let circle_1 = Circle::default();

    println!("{}", rect_1);
    println!("{}", circle_1);
}
