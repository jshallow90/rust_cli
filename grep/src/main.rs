mod grep_file;
mod args;

use args::Args;
use clap::Parser;
use grep_file::GrepFile;

fn main() {
    let args = Args::parse();
    let grep = GrepFile::new(args.search, args.file);

    let result = if grep.findall() { 0 } else { 1 };
    std::process::exit(result)
    // NEXT STEPS:
    // 2. add case insensitivty
    // 4. add pipe grep option

}
