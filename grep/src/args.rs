pub struct Args {
    pub search: String,
    pub file: String,
    pub case_insensitive: bool,
}

impl Args {
    pub fn new(search: String, file: String, case_insensitive: bool) -> Self {
        Args {
            search: search,
            file: file,
            case_insensitive: case_insensitive
        }
    }
}