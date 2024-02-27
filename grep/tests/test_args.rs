#[cfg(test)]
mod test_args {
    use grep::args::ArgOptions;
    use grep::args::InputType;
    use regex::RegexBuilder;
    
    #[test]
    fn file_named_args() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("-i"));
        args.push(String::from("-l"));
        args.push(String::from("t.*T"));
        args.push(String::from("test_file.txt"));
        
        let arg_opts = ArgOptions::new(args, InputType::File);
        let re = RegexBuilder::new("t.*T")
            .build()
            .expect("Invalid Regex");
        
        assert_eq!(arg_opts.file_only, true);
        assert_eq!(arg_opts.files, ["test_file.txt"]);
        assert_eq!(arg_opts.re.as_str(), re.as_str());
        assert_eq!(arg_opts.input_type, InputType::File);
    }

    #[test]
    fn pipe_named_args() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("-i"));
        args.push(String::from("-l"));
        args.push(String::from("t.*T"));
        
        let arg_opts = ArgOptions::new(args, InputType::Pipe);
        let re = RegexBuilder::new("t.*T")
            .build()
            .expect("Invalid Regex");
        
        assert_eq!(arg_opts.file_only, false);
        assert_eq!(arg_opts.files, Vec::<String>::new());
        assert_eq!(arg_opts.re.as_str(), re.as_str());
        assert_eq!(arg_opts.input_type, InputType::Pipe);
    }

    #[test]
    fn mutliple_files_def_args() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("t.*T"));
        args.push(String::from("test_file.txt"));
        args.push(String::from("test_file2.txt"));
        
        let arg_opts = ArgOptions::new(args, InputType::File);
        assert_eq!(arg_opts.files, ["test_file.txt", "test_file2.txt"]);
    }

    #[test]
    fn mutliple_files_named_args() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("t.*T"));
        args.push(String::from("-il"));
        args.push(String::from("test_file.txt"));
        args.push(String::from("test_file2.txt"));
        
        let arg_opts = ArgOptions::new(args, InputType::File);
        let re = RegexBuilder::new("t.*T")
            .case_insensitive(true)
            .build()
            .expect("Invalid Regex");

        assert_eq!(arg_opts.re.as_str(), re.as_str());
        assert_eq!(arg_opts.file_only, true);
        assert_eq!(arg_opts.files, ["test_file.txt", "test_file2.txt"]);
    }
}
