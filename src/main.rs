use crate::visualiser::traverser::TreeTraverser;

pub mod visualiser;

fn main() {
    let origin = String::from("~/");

    let tree_traverser = TreeTraverser {
        origin_path: &origin,
        max_traversal_depth: 2,
        current_traversal_depth: 1,
    };

    tree_traverser.traverse();
}
