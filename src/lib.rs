use std::env::Args;
use std::fs;
use std::path::Path;

use regex::Regex;

pub struct Config {
    starting_point: String,
    expression: Vec<(String, String)>,
}

impl Config {
    pub fn new(mut args: Args) -> Config {
        args.next(); // The first argument is the program name, discard it

        let starting_point = args.next().unwrap();
        let mut expression = Vec::new();

        while let Some(action) = args.next() {
            if let Some(value) = args.next() {
                expression.push((action, value));
            }
        }

        Config {
            starting_point,
            expression,
        }
    }
}

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> App {
        App { config }
    }

    pub fn run(&self) {
        self.visit_path(&Path::new(&self.config.starting_point));
    }

    fn visit_path(&self, path: &Path) {
        let entries = fs::read_dir(path).expect("invalid path");

        for entry in entries {
            let path = entry.unwrap().path();

            if self.is_match(&path) {
                println!("{}", path.display());
            }

            if path.is_dir() {
                self.visit_path(&path);
            }
        }
    }

    fn is_match(&self, path: &Path) -> bool {
        self.config
            .expression
            .iter()
            .all(|(name, value)| match name.as_str() {
                "-name" => self.name_action(value, path.file_name().unwrap().to_str().unwrap()),
                "-type" => self.type_action(value, path),
                _ => panic!("invalid action"),
            })
    }

    fn name_action(&self, value: &String, filename: &str) -> bool {
        let re = Regex::new(value).expect("invalid regex");

        re.is_match(filename)
    }

    fn type_action(&self, value: &String, path: &Path) -> bool {
        match value.as_str() {
            "f" => path.is_file(),
            "d" => path.is_dir(),
            "l" => path.is_symlink(),
            _ => panic!("invalid file type"),
        }
    }
}
