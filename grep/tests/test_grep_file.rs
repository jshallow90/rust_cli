#[cfg(test)]
mod test_args {
    #[test]
    fn file_args() {
        let args = grep_file::GrepFile::new();
    }
}

#[cfg(test)]
mod test_grep_file {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
