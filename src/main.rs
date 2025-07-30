use crate::visualiser::traverser::TreeTraverser;
use config::Config;
use std::env;
use std::path::*;

pub mod config;
pub mod visualiser;

fn main() {
    // Using .collect() to create a collection of the iterator values in args.
    // Annotating as a vector to tell Rust that we want to store these in a vector.
    let args: Vec<String> = env::args().collect();
    // Shadow args with our new args struct instance.
    let args: Config = Config::new(&args);

    let root = PathBuf::from(&args.path);
    let max_depth = 2;
    let current_depth = 0;
    let acc_f_count = 0;
    let acc_d_count = 0;

    let mut tree_traverser = TreeTraverser::new(
        root,
        &args.path,
        max_depth,
        current_depth,
        acc_f_count,
        acc_d_count,
        &args.format_specifier,
    );

    tree_traverser.traverse(Path::new(&args.path));

    println!(
        "Found {} files and {} directories in this tree.",
        &tree_traverser.accumulative_file_count, &tree_traverser.accumulative_dir_count
    );
}
