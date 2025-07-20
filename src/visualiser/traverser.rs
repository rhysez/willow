use std::fs;

// Using string slices so that struct does not take ownership.
// Also passing generic lifetime that origin_path and current_file_name share.
pub struct TreeTraverser<'a> {
    pub origin_path: &'a str, // The starting path that Willow starts traversing from.
    pub max_traversal_depth: u32, // The maximum traversal depth allowed.
    pub current_traversal_depth: u32, // The current depth of the traversal at some point in time.
    pub accumulative_file_count: u32, // The total number of files found thus far.
}

// TO DO
// 1. Iterate through wd and print files.
// 2. Return file count to main function.
impl<'a> TreeTraverser<'a> {
    pub fn traverse(&self) {
        let entries = fs::read_dir(self.origin_path).unwrap();
        for entry in entries {
            println!("{}", entry.unwrap().path().display())
        }
    }
}
