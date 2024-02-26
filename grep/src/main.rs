mod libs;

use atty::Stream;
use libs::args::{ArgOptions, InputType};
use libs::grep_file::GrepFile;
use libs::grep_pipe::GrepPipe;
use std::env;


fn main() {
    let args: Vec<_> = env::args().collect();
    let args = &args[1..];

    let exit_code = match atty::is(Stream::Stdin) {
        true => {
            let grep = GrepFile::new(
                ArgOptions::new(args.to_vec(), InputType::File)
            );
            if grep.find_all() { 0 } else { 1 }
        },
        false => {
            let grep = GrepPipe::new(
                ArgOptions::new(args.to_vec(), InputType::Pipe)
            );
            if grep.find_all() { 0 } else { 1 }
        }
    };
    
    std::process::exit(exit_code)
        
    // NEXT STEPS:
    // 7. add test cases

}
