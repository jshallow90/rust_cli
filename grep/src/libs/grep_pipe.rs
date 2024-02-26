use colored::Colorize;
use regex::{Captures, RegexBuilder};
use std::io::stdin;

use crate::libs::args::ArgOptions;


pub struct GrepPipe {
    args: ArgOptions
}

impl GrepPipe {
    pub fn new(args: ArgOptions) -> Self{
        GrepPipe {
            args: args
        }
    }

    pub fn find_all(&self) -> bool {
        let search = format!(r"({})", &self.args.search);
        let re = RegexBuilder::new(&search)
            .case_insensitive(self.args.case_insensitive)
            .build()
            .expect("Invalid Regex");
        let mut found = false;
        
        let lines = stdin().lines();
        for line in lines {
            let line = line.unwrap();
            if re.is_match(&line) {
                found = true;
                let line = re.replace_all(&line, |caps: &Captures| {
                    format!("{}", &caps[1].green())
                });
                println!("{}", line);
            }
        }
        found
    }
}