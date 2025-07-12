pub struct TreeTraverser {
    pub origin_path: String, // The starting path that Willow starts traversing from.
    pub max_traversal_depth: u32, // The maximum traversal depth allowed.
    pub current_traversal_depth: u32, // The current depth of the traversal at some point in time.
    pub current_file_name: String, // The file that the traverser is currently interpreting.
}

impl TreeTraverser {
    pub fn traverse() {
        println!("Traversing")
    }
}
