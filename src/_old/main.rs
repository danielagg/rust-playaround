use shapes::{area::Area, circle::Circle, rectangle::Rectangle};

use crate::shapes::collision::Collidable;

mod shapes;

fn old_main() {
    let rect_1 = Rectangle::default();
    let rect_2 = Rectangle::default();
    let circle_1 = Circle::default();
    let circle_2 = Circle::default();

    println!("{}", rect_1);
    println!("{}", rect_2);
    println!("{}", circle_1);
    println!("{}", circle_2);

    println!("{}", rect_1.collide(&rect_2));
    println!("{}", rect_2.collide(&circle_1));
    println!("{}", circle_1.collide(&circle_2));
    println!("{}", circle_2.collide(&rect_1));

    let my_float: f64 = 6.0;
    println!("{}", my_float.get_area());
}
