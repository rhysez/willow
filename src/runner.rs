use crate::interpreter::TreeInterpreter;
use std::path::*;

pub fn run(path: &Path, mut interpreter: TreeInterpreter) {
    interpreter.traverse(path);

    println!(
        "Found {} files and {} directories in this tree.",
        interpreter.accumulative_file_count, interpreter.accumulative_dir_count
    );
}
