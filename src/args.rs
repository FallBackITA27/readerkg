
pub struct Args {
    pub file_path: Option<String>,
    pub verbose: bool,
}

impl Args {
    pub fn new_parse_args(args: &mut impl std::iter::Iterator<Item = String>) -> Self {
        let mut final_args = Self {
            file_path: None,
            verbose: false,
        };

        let mut args = args.peekable();
        while args.peek().is_some() {
            final_args.parse_args(&mut args);
        }

        final_args
    }

    pub fn parse_args(&mut self, args: &mut impl std::iter::Iterator<Item = String>) {
        match args.next().unwrap().as_str() {
            "-f" | "--file" | "--file-path" => self.file_path = Some(args.next().unwrap()),
            "-v" | "--verbose" => self.verbose = true,
            k => panic!("Unknown flag `{k}`!"),
        }
    }
}
