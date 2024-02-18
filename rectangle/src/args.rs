use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Rectange")]
#[command(version = "1.0")]
#[command(about = "Rectangle, prints area and perimeter", long_about = "A longer about")]
pub struct CLIArgs {
    #[arg(short, long)]
    pub width: u16,

    #[arg(short, long)]
    pub length: u16,
}
