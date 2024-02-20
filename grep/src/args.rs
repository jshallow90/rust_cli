use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Grep")]
#[command(version = "1.0")]
#[command(about = "Search in file for strings", long_about = "A longer about")]
pub struct Args {
    #[arg(short, long)]
    pub search: String,

    #[arg(short, long)]
    pub file: String,
}
