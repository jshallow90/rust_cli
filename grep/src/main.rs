mod grep_file;
mod args;

use args::Args;
use grep_file::GrepFile;
use std::env;

fn help() {
    print!("Grep command to search in files\n\nUsage: grep [OPT] search_string file_name\n\nValid values for [opt] are:\n\t-i = case insensitive search")
}

fn grep(search: &String, file: &String, case_sensitive: bool) -> i32 {
    let args = Args::new(search.to_string(), file.to_string(), case_sensitive);
    let grep = GrepFile::new(args);
    if grep.findall() { 0 } else { 1 }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let args: &[String] = &args[1..];
    let mut exit_code = 0;

    dbg!(args);
    
    if args.len() < 2 { 
        help();
        exit_code = 1;
    } else if args.len() == 2 {
        exit_code = grep(&args[0], &args[1], false);
    } else if args.len() == 3 && &args[0] == "-i" {
        exit_code = grep(&args[1], &args[2], true);
    }
    
    std::process::exit(exit_code)
    
    
    // NEXT STEPS:
    // 4. add pipe grep option
    // 5. add way to search in multiple files
    // 6. add enum for multiple args

}
