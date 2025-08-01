use config::Config;
use interpreter::TreeInterpreter;
use std::env;
use std::path::*;

pub mod config;
pub mod interpreter;

const DEPTH_INITIAL: usize = 0;
const ACC_FILE_COUNT: usize = 0;
const ACC_DIR_COUNT: usize = 0;

fn main() {
    // Using .collect() to create a collection of the iterator values in args.
    // Annotating as a vector to tell Rust that we want to store these in a vector.
    let args: Vec<String> = env::args().collect();
    // Shadow args with our new args struct instance.
    let args: Config = Config::new(&args);

    let root = PathBuf::from(&args.path);
    let max_depth = args.max_traversal_depth;

    let mut interpreter = TreeInterpreter::new(
        root,
        max_depth,
        DEPTH_INITIAL,
        ACC_FILE_COUNT,
        ACC_DIR_COUNT,
        &args.format_specifier,
    );

    interpreter.traverse(Path::new(&args.path));

    println!(
        "Found {} files and {} directories in this tree.",
        &interpreter.accumulative_file_count, &interpreter.accumulative_dir_count
    );
}
