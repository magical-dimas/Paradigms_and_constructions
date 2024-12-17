pub struct Color{
    color: String
}

impl Color{
    pub fn new(str: String) -> Color{
        Color {color: str}
    }
    pub fn set_color(&mut self, str: String){
        self.color = str;
    }
    pub fn get_color(&self) -> String{
        self.color.clone()
    }
}