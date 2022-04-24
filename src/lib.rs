use std::{path::{PathBuf, Path}, fs};

use clap::{Arg, Command, crate_authors, crate_description, crate_name, crate_version};
use regex::Regex;

pub struct Config {
    starting_points: Vec<PathBuf>,
    options: Vec<(String, String)>,
}

impl Config {
    pub fn new() -> Config {
        let matches = Command::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!())
            .arg(
                Arg::new("starting-point")
                    .value_name("STARTING-POINT")
                    .help("The starting-point of the program")
                    .default_value(".")
                    .multiple_values(true),
            )
            .arg(
                Arg::new("name")
                    .value_name("NAME")
                    .help("The regex to match")
                    .short('n')
                    .long("name")
                    .takes_value(true),
            )
            .arg(
                Arg::new("type")
                    .value_name("TYPE")
                    .help("The type of the file")
                    .short('t')
                    .long("type")
                    .takes_value(true)
                    .possible_values(["f", "d", "l"]),
            )
            .get_matches();

        let starting_points = matches
            .values_of("starting-point")
            .unwrap()
            .map(|value| PathBuf::from(value))
            .collect();

        let mut options = Vec::new();

        if let Some(value) = matches.value_of("name") {
            options.push(("name".to_string(), value.to_owned()));
        }

        if let Some(value) = matches.value_of("type") {
            options.push(("type".to_string(), value.to_owned()));
        }

        Config {
            starting_points,
            options,
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
        for starting_point in &self.config.starting_points {
            self.visit_path(starting_point);
        }
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
        self.config.options.iter().all(|(option, value)| {
            match option.as_str() {
                "name" => self.name_option(value, path.file_name().unwrap().to_str().unwrap()),
                "type" => self.type_option(value, path),
                _ => panic!("invalid option"),
            }
        })
    }

    fn name_option(&self, value: &str, filename: &str) -> bool {
        let re = Regex::new(value).expect("invalid regex");

        re.is_match(filename)
    }

    fn type_option(&self, value: &str, path: &Path) -> bool {
        match value {
            "f" => path.is_file(),
            "d" => path.is_dir(),
            "l" => path.is_symlink(),
            _ => panic!("invalid file type"),
        }
    }
}
