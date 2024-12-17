use std::fmt;
use crate::figure::Figure;
use crate::color::Color;
use crate::rect::Rect;

pub struct Square{
    figure_type: String,
    rect: Rect
}

impl Square{
    pub fn new(s: f64, col: &Color) -> Square{
        let c: Color = Color::new(col.get_color());
        let r: Rect = Rect::new(s, s, &c);
        Square {figure_type: "square".to_string(), rect: r}
    }
    pub fn get_type(&self) -> String{
        self.figure_type.clone()
    }
}

impl Figure for Square{
    fn get_area(&self) -> f64{
        self.rect.get_area()
    }
    fn repr(&self) -> String{
        format!("Тип: {}, Сторона: {}, Цвет: {}, Площадь: {}", self.get_type(), self.rect.width, self.rect.color.get_color(), self.get_area())
    }
}

impl std::fmt::Display for Square{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}