pub struct Rectangle {
    x: f64,
    y: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64) -> Rectangle {
        Rectangle {
            x: x,
            y: y
        }
    }
    
    fn area(&self) -> f64 {
        self.x * self.y
    }

    fn perimeter(&self) -> f64 {
        2. * (self.x + self.y)
    }

    pub fn print_info(&self) {
        println!("[INFO] Area={}, Perimeter={}", self.area(), self.perimeter());
    }
}