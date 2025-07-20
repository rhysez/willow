use std::fs;

// Using string slices so that struct does not take ownership.
// Also passing generic lifetime that origin_path and current_file_name share.
pub struct TreeTraverser<'a> {
    pub origin_path: &'a str, // The starting path that Willow starts traversing from.
    pub max_traversal_depth: u32, // The maximum traversal depth allowed.
    pub current_traversal_depth: u32, // The current depth of the traversal at some point in time.
    pub accumulative_file_count: usize, // The total number of files found thus far.
}

// TO DO
// 1. Iterate through wd and print files. DONE
// 2. Return file count to main function. DONE
// 3. Add accumulative_dir_count and count the directories found.
// 4. Iterate deeper into tree based on max_traversal_depth.
impl<'a> TreeTraverser<'a> {
    pub fn traverse(&mut self) {
        let entries = fs::read_dir(self.origin_path).unwrap();
        for entry in entries {
            println!("{}", entry.unwrap().path().display());
            self.accumulative_file_count += 1;
        }
    }
}
