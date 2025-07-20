// Using string slices so that struct does not take ownership.
// Also passing generic lifetime that origin_path and current_file_name share.
pub struct TreeTraverser<'a> {
    pub origin_path: &'a str, // The starting path that Willow starts traversing from.
    pub max_traversal_depth: u32, // The maximum traversal depth allowed.
    pub current_traversal_depth: u32, // The current depth of the traversal at some point in time.
}

// TO DO
// 1. Iterate through wd and print files.

impl<'a> TreeTraverser<'a> {
    pub fn traverse(&self) {
        // Starts with first file in origin path.
        //let mut current_file: String = String::from("");

        println!("Traversing from {}", self.origin_path);
    }
}
