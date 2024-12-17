use figures::color::Color;
use figures::square::Square;
use figures::rect::Rect;
use figures::circle::Circle;

fn main() {
    let mut col = Color::new("синий".to_string());
    let rect = Rect::new(3.0, 3.0, &col);
    col.set_color("зелёный".to_string());
    let circle = Circle::new(3.0, &col);
    col.set_color("красный".to_string());
    let square = Square::new(3.0, &col);
    println!("{}", rect);
    println!("{}", circle);
    println!("{}", square);
}
