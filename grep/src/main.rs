use atty::Stream;
use grep::args::{ArgOptions, InputType};
use grep::grep::{GrepFile, GrepPipe};
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

}
