pub struct Config {
    pub path: String,
    pub format_specifier: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        const MIN_ARG_COUNT_BINARY: usize = 3;
        const MIN_ARG_COUNT_ACTUAL: usize = MIN_ARG_COUNT_BINARY - 1;

        if args.len() < MIN_ARG_COUNT_BINARY {
            panic!("Insufficient arguments. Provide at least {MIN_ARG_COUNT_ACTUAL} arguments.");
        }

        let mut path = String::from("./");
        let path_index = 1;

        let mut format_specifier = String::from("--names");
        let format_index = 2;

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

        Config {
            path,
            format_specifier,
        }
    }
}
