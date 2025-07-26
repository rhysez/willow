use crate::visualiser::traverser::TreeTraverser;
use std::env;

pub mod visualiser;

fn main() {
    // Using .collect() to create a collection of the iterator values in args.
    // Annotating as a vector to tell Rust that we want to store these in a vector.
    let args: Vec<String> = env::args().collect();
    let (origin_path, include_paths) = parse_args(&args);

    let mut tree_traverser = TreeTraverser {
        origin_path,
        max_traversal_depth: 2,
        current_traversal_depth: 1,
        accumulative_file_count: 0,
        accumulative_dir_count: 0,
        include_paths,
    };

    tree_traverser.traverse();

    println!("Found {} files.", &tree_traverser.accumulative_file_count)
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let path = &args[1];
    let include_paths_specifier = &args[2];

    (path, include_paths_specifier)
}
