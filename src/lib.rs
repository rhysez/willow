pub mod config {
    pub struct Config {
        pub path: String,
        pub format_specifier: String,
        pub max_traversal_depth: usize,
    }

    const MIN_ARG_COUNT_BINARY: usize = 3;
    // const MIN_ARG_COUNT_ACTUAL: usize = MIN_ARG_COUNT_BINARY - 1;

    impl Config {
        pub fn new(args: &[String]) -> Config {
            if args.len() < MIN_ARG_COUNT_BINARY {
                eprintln!("Insufficient arguments. Provide at least 2 arguments.");
                std::process::exit(1);
            }

            let mut path = String::from("./");
            let path_index = 1;

            let mut format_specifier = String::from("--names");
            let format_index = 2;

            let mut max_traversal_depth = 0;
            let max_traversal_depth_index = 3;

            if let Some(value) = args.get(path_index) {
                let specified_path = String::from(value);
                path = specified_path;
            }

            if let Some(value) = args.get(format_index) {
                let specified_format = String::from(value);
                let acceptable_formats = [String::from("--paths"), String::from("--names")];

                if acceptable_formats.contains(value) {
                    format_specifier = specified_format;
                }
            }

            if let Some(value) = args.get(max_traversal_depth_index) {
                max_traversal_depth = match value.parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("There was an error while trying to read at the provided depth.");
                        std::process::exit(1);
                    }
                }
            }

            Config {
                path,
                format_specifier,
                max_traversal_depth,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn can_instantiate_config_with_args() {
            let args = vec![
                String::from("bin_placeholder"),
                String::from("./"),
                String::from("--names"),
                String::from("2"),
            ];
            let cfg = Config::new(&args);
            assert_eq!(cfg.path, "./");
        }
    }
}

pub mod interpreter {
    use std::fs;
    use std::fs::*;
    use std::path::*;

    // Using string slices so that struct does not take ownership.
    // Also passing generic lifetime that origin_path and current_file_name share.
    pub struct TreeInterpreter<'a> {
        root_path: PathBuf, // The starting path that Willow starts traversing from.
        pub max_traversal_depth: usize, // The maximum traversal depth allowed.
        current_traversal_depth: usize, // The current depth of the traversal at some point in time.
        pub accumulative_file_count: usize, // The total number of files found thus far.
        pub accumulative_dir_count: usize, // The total number of directories found thus far.
        format_specifier: &'a str, // Whether the program should print entries as paths.
    }

    impl<'a> TreeInterpreter<'a> {
        pub fn new(
            p_root_path: PathBuf,
            p_max_traversal_depth: usize,
            p_current_traversal_depth: usize,
            p_accumulative_file_count: usize,
            p_accumulative_dir_count: usize,
            p_format_specifier: &'a str,
        ) -> TreeInterpreter<'a> {
            let root_path = p_root_path;
            let max_traversal_depth = p_max_traversal_depth;
            let current_traversal_depth = p_current_traversal_depth;
            let accumulative_file_count = p_accumulative_file_count;
            let accumulative_dir_count = p_accumulative_dir_count;
            let format_specifier = p_format_specifier;

            TreeInterpreter {
                root_path,
                max_traversal_depth,
                current_traversal_depth,
                accumulative_file_count,
                accumulative_dir_count,
                format_specifier,
            }
        }

        pub fn get_entries(&self, path: &Path) -> std::fs::ReadDir {
            match fs::read_dir(path) {
                Ok(values) => values,
                Err(_) => {
                    eprintln!("The program was unable to read the file tree.");
                    std::process::exit(1);
                }
            }
        }

        fn calculate_and_assign_depth(&mut self, entry: &DirEntry) -> usize {
            let depth = entry.path().components().count() - self.root_path.components().count();
            self.current_traversal_depth = depth;

            self.current_traversal_depth
        }

        fn format_leaf(&self, entry: &DirEntry, path: &Path) {
            let indent: String = " ".repeat(self.current_traversal_depth);

            if self.format_specifier == "--paths" {
                println!("{}|-{}", indent, entry.path().display());
            } else if path.is_dir() {
                println!("{}|-{}/", indent, entry.file_name().display());
            } else {
                println!("{}|-{}", indent, entry.file_name().display());
            }
        }

        pub fn can_traverse_next_depth_level(&self) -> bool {
            self.current_traversal_depth < self.max_traversal_depth
        }

        pub fn traverse(&mut self, path: &Path) {
            let entries = self.get_entries(path);

            for entry in entries {
                let entry = match entry {
                    Ok(value) => value,
                    Err(_) => continue,
                };

                let path = entry.path();
                let file_name = match entry.file_name().into_string() {
                    Ok(string) => string,
                    Err(_) => continue,
                };

                // Skip dotfiles
                if file_name.starts_with(".") {
                    continue;
                }

                self.calculate_and_assign_depth(&entry);
                self.format_leaf(&entry, &path);

                if path.is_dir() {
                    self.accumulative_dir_count += 1;
                    if self.can_traverse_next_depth_level() {
                        self.traverse(&path);
                    }
                } else {
                    self.accumulative_file_count += 1;
                }
            }
        }
    }
}

pub mod runner {
    use std::path::*;

    pub fn run(path: &Path, mut interpreter: crate::interpreter::TreeInterpreter) {
        interpreter.traverse(path);

        println!(
            "Found {} files and {} directories in this tree.",
            interpreter.accumulative_file_count, interpreter.accumulative_dir_count
        );
    }
}
