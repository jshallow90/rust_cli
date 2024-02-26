use colored::Colorize;
use regex::{Captures, Regex, RegexBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::libs::args::ArgOptions;


pub struct GrepFile {
    args: ArgOptions
}

impl GrepFile {
    pub fn new(args: ArgOptions) -> Self{
        GrepFile {
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
        
        for file in &self.args.files {
            let result = self.find_in_file(&file, &re);
            if ! found && result {
                found = true
            }
        }
        found
    }

    fn find_in_file(&self, file: &String, re: &Regex) -> bool {
        let f = File::open(&file)
            .expect(format!("{}: file {} does not exists", 
                "ERROR".red(),
                file).as_str());

        let mut found = false;
        let file_out = if self.args.files.len() > 1 {
            format!("{}", file.purple())
        } else {
            "".to_string()
        };
        
        let buffer = BufReader::new(f);
        for line in buffer.lines() {
            let line = line.unwrap();
            if re.is_match(&line) {
                found = true;
                if self.args.file_only {
                    println!("{}", file_out);
                    return found
                }
                
                let line = re.replace_all(&line, |caps: &Captures| {
                    format!("{}", &caps[1].green())
                });
                println!("{}:{}", file_out, line);
            }
        }
        found
    }
}