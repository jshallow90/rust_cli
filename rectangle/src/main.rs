mod rectangle;

use std::env;
use rectangle::Rectangle;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
