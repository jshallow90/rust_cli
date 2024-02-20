mod grep_file;
mod args;

use args::Args;
use clap::Parser;
use grep_file::GrepFile;

fn main() {
    let args = Args::parse();
    let grep = GrepFile::new(args.search, args.file);

    grep.findall()
        .expect("ERROR: process failed");

    // NEXT STEPS:
    // 2. add case insensitivty
    // 3. return value for no match found
    // 4. add pipe grep option

}
