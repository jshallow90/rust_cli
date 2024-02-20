use colored::Colorize;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrepFile {
    search: String,
    file: String,
}

impl GrepFile {
    pub fn new(search: String, file: String) -> GrepFile {
        GrepFile {
            search: search,
            file: file,
        }
    }

    pub fn findall(&self) -> Result<(), Error> {
        println!("Searching for {} in file {}", self.search, self.file);

        let file = File::open(&self.file)
            .expect(format!("{}: file {} does not exists", 
                "ERROR".red(),
                self.file).as_str());

        let search = format!(r"({})", &self.search);
        let re: Regex = Regex::new(&search).unwrap();
        
        let buffer = BufReader::new(file);
        for line in buffer.lines() {
            let line = line?;
            if re.is_match(&line) {
                let line = re.replace_all(&line, |caps: &Captures| {
                    format!("{}", &caps[1].green())
                });
                println!("{}", line);
            }
        }
        Ok(())
    }
}