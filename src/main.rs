use crate::visualiser::traverser::TreeTraverser;

pub mod visualiser;

fn main() {
    let tree_traverser: TreeTraverser = TreeTraverser {
        origin_path: String::from("~/"),
        max_traversal_depth: 2,
        current_traversal_depth: 1,
        current_file_name: String::from("Origin"),
    };

    TreeTraverser::traverse()
}
