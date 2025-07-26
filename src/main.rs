use crate::visualiser::traverser::TreeTraverser;
use std::env;

pub mod visualiser;

fn main() {
    // Using .collect() to create a collection of the iterator values in args.
    // Annotating as a vector to tell Rust that we want to store these in a vector.
    let args: Vec<String> = env::args().collect();
    // Shadow args with our new args struct instance.
    let args: Args = Args::new(&args);

    // TODO: Replace magic numbers with meaningful named variables.
    let mut tree_traverser = TreeTraverser::new(&args.path, 2, 1, 0, 0, &args.format_specifier);

    tree_traverser.traverse();

    println!("Found {} files.", &tree_traverser.accumulative_file_count)
}

struct Args {
    path: String,
    format_specifier: String,
}

impl Args {
    fn new(args: &[String]) -> Args {
        let path = args[1].clone();
        let format_specifier = args[2].clone();

        Args {
            path,
            format_specifier,
        }
    }
}
