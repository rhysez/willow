use crate::visualiser::traverser::TreeTraverser;

pub mod visualiser;

fn main() {
    let origin_path = String::from("./");

    let mut tree_traverser = TreeTraverser {
        origin_path: &origin_path,
        max_traversal_depth: 2,
        current_traversal_depth: 1,
        accumulative_file_count: 0,
        accumulative_dir_count: 0,
    };

    tree_traverser.traverse();

    println!("Found {} files.", &tree_traverser.accumulative_file_count)
}
