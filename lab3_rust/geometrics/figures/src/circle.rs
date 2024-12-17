use crate::color::Color;
use std::fmt;
use crate::figure::Figure;

pub struct Circle{
    figure_type: String,
    radius: f64,
    color: Color
}

impl Circle{
    pub fn new(rad: f64, col: &Color) -> Circle{
        let c: Color = Color::new(col.get_color());
        Circle {figure_type: "circle".to_string(), radius: rad, color: c}
    }
    pub fn get_type(&self) -> String{
        self.figure_type.clone()
    }
}

impl Figure for Circle{
    fn get_area(&self) -> f64{
        std::f64::consts::PI*self.radius.powi(2)
    }
    fn repr(&self) -> String{
        format!("Тип: {}, Радиус: {}, Цвет: {}, Площадь: {}", self.get_type(), self.radius, self.color.get_color(), self.get_area())
    }
}

impl std::fmt::Display for Circle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}