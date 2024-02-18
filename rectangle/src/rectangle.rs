pub struct Rectangle {
    x: u16,
    y: u16,
}

impl Rectangle {
    pub fn new(x: u16, y: u16) -> Rectangle {
        Rectangle {
            x: x,
            y: y
        }
    }
    
    fn area(&self) -> u16 {
        self.x * self.y
    }

    fn perimeter(&self) -> u16 {
        2 * (self.x + self.y)
    }

    pub fn print_info(&self) {
        println!("[INFO] Area={}, Perimeter={}", self.area(), self.perimeter());
    }
}