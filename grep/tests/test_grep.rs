#[cfg(test)]
mod tests_grep_file {

    use grep::args::ArgOptions;
    use grep::args::InputType;
    use grep::grep::GrepFile;
    use std::env;

    fn get_test_dir() -> String {
        let mut current_dir = env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        current_dir += "/tests/";
        current_dir
    }

    #[test]
    fn test_no_args() {
        let current_dir = get_test_dir();
        
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("t.*T"));
        args.push(String::from(format!("{}/test_file.txt", current_dir)));

        let args = ArgOptions::new(args, InputType::File);
        let grep = GrepFile::new(args);
        let result = grep.find_all();
        assert_eq!(result, true);
    }

    #[test]
    fn test_args() {
        let current_dir = get_test_dir();
        
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("cazz"));
        args.push(String::from("-i"));
        args.push(String::from(format!("{}/test_file.txt", current_dir)));

        let args = ArgOptions::new(args, InputType::File);
        let grep = GrepFile::new(args);
        let result = grep.find_all();
        assert_eq!(result, true);
    }

    #[test]
    fn test_no_match() {
        let current_dir = get_test_dir();
        
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("cazz"));
        args.push(String::from(format!("{}/test_file.txt", current_dir)));

        let args = ArgOptions::new(args, InputType::File);
        let grep = GrepFile::new(args);
        let result = grep.find_all();
        assert_eq!(result, false);
    }

    #[test]
    fn test_multiple_files_found() {
        let current_dir = get_test_dir();
        
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("ew[a-z][kl][w-z]r"));
        args.push(String::from("-i"));
        args.push(String::from(format!("{}/test_file.txt", current_dir)));
        args.push(String::from(format!("{}/test_file2.txt", current_dir)));

        let args = ArgOptions::new(args, InputType::File);
        let grep = GrepFile::new(args);
        let result = grep.find_all();
        assert_eq!(result, true);
    }

    #[test]
    fn test_multiple_files_not_found() {
        let current_dir = get_test_dir();
        
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("ew[a-z][kl][w-z]r"));
        args.push(String::from(format!("{}/test_file.txt", current_dir)));
        args.push(String::from(format!("{}/test_file2.txt", current_dir)));

        let args = ArgOptions::new(args, InputType::File);
        let grep = GrepFile::new(args);
        let result = grep.find_all();
        assert_eq!(result, false);
    }
}

#[cfg(test)]
mod test_grep_pipe {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}