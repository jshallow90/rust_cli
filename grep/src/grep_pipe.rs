use colored::Colorize;
use regex::{Captures, RegexBuilder};
use std::io::stdin;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrepPipe {
    search: String,
    case_insensitive: bool,
}

impl GrepPipe {
    pub fn new(search: &String, case_insensitive: bool) -> Self{
        GrepPipe {
            search: search.to_string(),
            case_insensitive: case_insensitive
        }
    }

    pub fn findall(&self) -> bool {
        let search = format!(r"({})", &self.search);
        let re = RegexBuilder::new(&search)
            .case_insensitive(self.case_insensitive)
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