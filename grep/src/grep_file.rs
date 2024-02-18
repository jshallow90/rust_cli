use std::fs::File;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrepFileOptions {
  pub search: String,
  pub file_path: String,
  pub case_sensitive: bool,
}

impl GrepFileOptions {
    fn findall() {
        println!("Searching for {}", query);
        println!("In file {}", file_path);

        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("{}", contents);
    }
}