use crate::visualiser::traverser::TreeTraverser;
use std::env;

pub mod visualiser;

fn main() {
    // Using .collect() to create a collection of the iterator values in args.
    // Annotating as a vector to tell Rust that we want to store these in a vector.
    let args: Vec<String> = env::args().collect();
    // Shadow args with our new args struct instance.
    let args: Config = Config::new(&args);

    // TODO: Replace magic numbers with meaningful named variables.
    let mut tree_traverser = TreeTraverser::new(&args.path, 2, 1, 0, 0, &args.format_specifier);

    tree_traverser.traverse();

    println!("Found {} files.", &tree_traverser.accumulative_file_count)
}

struct Config {
    path: String,
    format_specifier: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let path = args[1].clone();
        let format_specifier = args[2].clone();

        Config {
            path,
            format_specifier,
        }
    }
}
