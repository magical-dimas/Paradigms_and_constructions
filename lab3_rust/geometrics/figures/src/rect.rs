use crate::color::Color;
use std::fmt;
use crate::figure::Figure;

pub struct Rect{
    figure_type: String,
    pub width: f64,
    height: f64,
    pub color: Color
}

impl Rect{
    pub fn new(h: f64, w: f64, col: &Color) -> Rect{
        let c: Color = Color::new(col.get_color());
        Rect {figure_type: "rectangle".to_string(), width: w, height: h, color: c}
    }
    pub fn get_type(&self) -> String{
        self.figure_type.clone()
    }
}

impl Figure for Rect{
    fn get_area(&self) -> f64{
        self.width*self.height
    }
    fn repr(&self) -> String{
        format!("Тип: {}, Ширина: {}, Высота: {}, Цвет: {}, Площадь: {}", self.get_type(), self.width, self.height, self.color.get_color(), self.get_area())
    }
}

impl std::fmt::Display for Rect{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}