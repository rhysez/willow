pub struct Config {
    pub path: String,
    pub format_specifier: String,
    pub max_traversal_depth: usize,
}

const MIN_ARG_COUNT_BINARY: usize = 3;
const MIN_ARG_COUNT_ACTUAL: usize = MIN_ARG_COUNT_BINARY - 1;

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < MIN_ARG_COUNT_BINARY {
            panic!("Insufficient arguments. Provide at least {MIN_ARG_COUNT_ACTUAL} arguments.");
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
                Err(_) => panic!(
                    "There was an error while attempting to process the provided depth level."
                ),
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

    #[test]
    #[should_panic]
    fn bad_config_panics() {
        let args = vec![String::from("bin_placeholder"), String::from("./")];
        let cfg = Config::new(&args);
    }
}
