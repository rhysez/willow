use crate::visualiser::traverser::TreeTraverser;

pub mod visualiser;

fn main() {
    let origin_path = String::from("./");
    let should_include_paths = false;

    let mut tree_traverser = TreeTraverser {
        origin_path: &origin_path,
        max_traversal_depth: 2,
        current_traversal_depth: 1,
        accumulative_file_count: 0,
        accumulative_dir_count: 0,
        include_paths: should_include_paths,
    };

    tree_traverser.traverse();

    println!("Found {} files.", &tree_traverser.accumulative_file_count)
}
