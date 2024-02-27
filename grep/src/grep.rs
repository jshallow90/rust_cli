use colored::Colorize;
use regex::Captures;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

use super::args::ArgOptions;


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
        let mut found = false;
        
        for file in &self.args.files {
            let result = self.find_in_file(&file);
            if ! found && result {
                found = true
            }
        }
        found
    }

    fn find_in_file(&self, file: &String) -> bool {
        let f = File::open(&file)
            .expect(format!("{}: file {} does not exists", 
                "ERROR".red(),
                file).as_str());

        let mut found = false;
        let file_out = if self.args.files.len() > 1 {
            format!("{}:", file.purple())
        } else {
            "".to_string()
        };
        
        let buffer = BufReader::new(f);
        for line in buffer.lines() {
            let line = line.unwrap();
            if self.args.re.is_match(&line) {
                found = true;
                if self.args.file_only {
                    println!("{}", file_out);
                    return found
                }
                
                let line = self.args.re.replace_all(&line, |caps: &Captures| {
                    format!("{}", &caps[0].green())
                });
                println!("{}{}", file_out, line);
            }
        }
        found
    }
}


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
        let mut found = false;
        
        let lines = stdin().lines();
        for line in lines {
            let line = line.unwrap();
            if self.args.re.is_match(&line) {
                found = true;
                let line = self.args.re.replace_all(&line, |caps: &Captures| {
                    format!("{}", &caps[1].green())
                });
                println!("{}", line);
            }
        }
        found
    }
}