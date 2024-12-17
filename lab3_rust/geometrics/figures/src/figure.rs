pub trait Figure{
    fn repr(&self) -> String;
    fn get_area(&self) -> f64;
}