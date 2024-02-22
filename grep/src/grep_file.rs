use colored::Colorize;
use regex::{Captures, RegexBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::args::Args;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrepFile {
    search: String,
    file: String,
    case_insensitive: bool,
}

impl GrepFile {
    pub fn new(args: Args) -> Self{
        GrepFile {
            search: args.search,
            file: args.file,
            case_insensitive: args.case_insensitive
        }
    }

    pub fn findall(&self) -> bool {
        println!("Searching for {} in file {} with arg {}", self.search, self.file, self.case_insensitive);

        let file = File::open(&self.file)
            .expect(format!("{}: file {} does not exists", 
                "ERROR".red(),
                self.file).as_str());

        let search = format!(r"({})", &self.search);
        let re = RegexBuilder::new(&search)
            .case_insensitive(self.case_insensitive)
            .build()
            .expect("Invalid Regex");
        let mut found = false;
        
        let buffer = BufReader::new(file);
        for line in buffer.lines() {
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