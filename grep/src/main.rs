mod grep_file;
mod grep_pipe;

use atty::Stream;
use grep_file::GrepFile;
use grep_pipe::GrepPipe;
use std::env;


fn help() {
    println!("Grep command to search in files\n");
    println!("Usage: grep [OPT] search_string file_name\n");
    println!("Valid values for [opt] are:\n\t-i = case insensitive search\n");
}

fn file_grepper(args: &Vec<String>) -> i32 {
    if args.len() == 2 {
        let grep = GrepFile::new(&args[0], &args[1], false);
        if grep.findall() { 0 } else { 1 }
    } else if args.len() == 3 && &args[0] == "-i" {
        let grep = GrepFile::new(&args[0], &args[1], true);
        if grep.findall() { 0 } else { 1 }
    } else {
        help();
        1
    }
}

fn pipe_grepper(args: &Vec<String>) -> i32 {
    if args.len() == 1 {
        let grep = GrepPipe::new(&args[0], false);
        if grep.findall() { 0 } else { 1 }
    } else if args.len() == 3 && &args[0] == "-i" {
        let grep = GrepPipe::new(&args[0], true);
        if grep.findall() { 0 } else { 1 }
    } else {
        help();
        1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..].to_vec();

    let exit_code = match atty::is(Stream::Stdin) {
        true => file_grepper(args),
        false => pipe_grepper(args)
    };
    
    std::process::exit(exit_code)
    
    
    // NEXT STEPS:
    // 4. add pipe grep option
    // 5. add way to search in multiple files
    // 6. add enum for multiple args

}
