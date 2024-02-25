pub struct ArgOptions {
    pub case_insensitive: bool,
    pub file_only: bool,
    pub search: String,
    pub files: Vec<String>
}

impl ArgOptions {
    pub fn new(args: Vec<String>) -> Self {
        
        let named_ars: Vec<_> = args
            .iter()
            .filter(|arg| arg.starts_with("-"))
            .collect();
        
        let pos_args: Vec<_> = args
            .iter()
            .filter(|arg| ! arg.starts_with("-"))
            .map(|arg| arg.to_string())
            .collect();
        
        // let pos_args_owned: Vec<String> = Vec::new();
        // for v in pos_args {
        //     pos_args_owned.push(v.to_string())
        // } 

        if pos_args.len() < 2 {
            ArgOptions::help();
            panic!("Invalid arguments supplied, see help above.");
        }

        let mut case_insensitive = false;
        let mut file_only = false;
        for arg in named_ars {
            match(arg).as_ref() {
                "-i" => case_insensitive = true,
                "-l" => file_only = true,
                _ => println!("Arg {arg} not recognized, ignoring")
            }
        }

        let search = &pos_args[0];
        if pos_args.len() > 1 {
            ArgOptions {
                case_insensitive: case_insensitive,
                file_only: file_only,
                search: search.to_owned(),
                files: pos_args[1..].to_owned()
            }
        } else {
            ArgOptions {
                case_insensitive: case_insensitive,
                file_only: false,
                search: search.to_owned(),
                files: Vec::new() 
            }
        }

    }

    fn help() {
        println!("Grep command to search in files\n");
        println!("Usage: grep [OPT] search_string file_name\n");
        println!("Valid values for [opt] are:\n\t-i = case insensitive search\n");
    }
}