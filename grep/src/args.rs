use regex::{Regex, RegexBuilder};


#[derive(Debug, PartialEq)]
pub enum InputType {
    File,
    Pipe
}

pub struct ArgOptions {
    pub input_type: InputType,
    pub file_only: bool,
    pub re: Regex,
    pub files: Vec<String>
}

impl ArgOptions {
    pub fn new(args: Vec<String>, input_type: InputType) -> Self {
        
        let custom_args: Vec<_> = args
            .iter()
            .filter(|arg| arg.starts_with("-"))
            .map(|arg| arg.replace("-", ""))
            .collect();
        
        let mut named_args: Vec<char> = Vec::new();
        for arg in &custom_args {
            for inner_arg in arg.chars() {
                named_args.push(inner_arg)
            }
        }
        
        let pos_args: Vec<_> = args
            .iter()
            .filter(|arg| ! arg.starts_with("-"))
            .map(|arg| arg.to_string())
            .collect();

        if (pos_args.len() < 1 && input_type == InputType::Pipe) || 
                (pos_args.len() < 2 && input_type == InputType::File) {
            ArgOptions::help();
            panic!("Invalid arguments supplied, see help above.");
        }

        let mut case_insensitive = false;
        let mut file_only = false;
        for arg in named_args {
            match arg {
                'i' => case_insensitive = true,
                'l' => file_only = true,
                _ => println!("Arg {arg} not recognized, ignoring")
            }
        }

        let re = RegexBuilder::new(&pos_args[0])
            .case_insensitive(case_insensitive)
            .build()
            .expect("Invalid Regex");
        
        match input_type {
            InputType::File => ArgOptions {
                input_type: InputType::File,
                file_only: file_only,
                re: re,
                files: pos_args[1..].to_owned()
            },
            InputType::Pipe => ArgOptions {
                input_type: InputType::Pipe,
                file_only: false,
                re: re,
                files: Vec::new()
            }
        }

    }

    fn help() {
        println!("Grep command to search in files\n");
        println!("Usage: grep [OPT] search_string file_name\n");
        println!("Valid values for [opt] are:\n\t-i = case insensitive search\n\t-l = list files only\n");
    }
}