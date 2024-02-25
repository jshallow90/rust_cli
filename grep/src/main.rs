mod args;
mod grep_file;
mod grep_pipe;

use args::ArgOptions;
use atty::Stream;
use grep_file::GrepFile;
use grep_pipe::GrepPipe;
use std::env;


fn help() {
    println!("Grep command to search in files\n");
    println!("Usage: grep [OPT] search_string file_name\n");
    println!("Valid values for [opt] are:\n\t-i = case insensitive search\n");
}

fn file_grepper(args: ArgOptions) -> i32 {
    let grep = GrepFile::new(args);
    if grep.find_all() { 0 } else { 1 }
}

fn pipe_grepper(args: ArgOptions) -> i32 {
    let grep = GrepPipe::new(args);
    if grep.find_all() { 0 } else { 1 }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let args = &args[1..];
    let args = ArgOptions::new(args.to_vec());

    let exit_code = match atty::is(Stream::Stdin) {
        true => file_grepper(args),
        false => pipe_grepper(args)
    };
    
    std::process::exit(exit_code)
    
    
    // NEXT STEPS:
    // 5. add way to search in multiple files
    // 6. add enum for multiple args

}
