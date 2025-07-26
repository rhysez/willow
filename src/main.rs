use crate::defs::constants;
use crate::visualiser::traverser::TreeTraverser;
use std::env;

pub mod defs;
pub mod visualiser;

fn main() {
    // Using .collect() to create a collection of the iterator values in args.
    // Annotating as a vector to tell Rust that we want to store these in a vector.
    let args: Vec<String> = env::args().collect();
    // Shadow args with our new args struct instance.
    let args: Config = Config::new(&args);

    let max_depth = 2;
    let current_depth = 1;
    let acc_f_count = 0;
    let acc_d_count = 0;

    let mut tree_traverser = TreeTraverser::new(
        &args.path,
        max_depth,
        current_depth,
        acc_f_count,
        acc_d_count,
        &args.format_specifier,
    );

    tree_traverser.traverse();

    println!("Found {} files.", &tree_traverser.accumulative_file_count)
}

struct Config {
    path: String,
    format_specifier: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let min_arg_count = 3;
        if args.len() < min_arg_count {
            panic!("Insufficient arguments. Provide at least {min_arg_count} arguments.");
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
            format_specifier = specified_format;
        }

        Config {
            path,
            format_specifier,
        }
    }
}
