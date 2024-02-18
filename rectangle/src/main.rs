mod args;
mod rectangle;

use args::CLIArgs;
use clap::Parser;
use rectangle::Rectangle;

fn main() {
    let args = CLIArgs::parse();
    let shape = Rectangle::new(args.width, args.length);
    shape.print_info();
}
