use std::fs;
use std::path::Path;

// Using string slices so that struct does not take ownership.
// Also passing generic lifetime that origin_path and current_file_name share.
pub struct TreeTraverser<'a> {
    pub path: &'a str, // The starting path that Willow starts traversing from.
    pub max_traversal_depth: u32, // The maximum traversal depth allowed.
    pub current_traversal_depth: usize, // The current depth of the traversal at some point in time.
    pub accumulative_file_count: usize, // The total number of files found thus far.
    pub accumulative_dir_count: usize, // The total number of directories found thus far.
    pub format_specifier: &'a str, // Whether the program should print entries as paths.
}

// TODO:
// 1. Iterate through wd and print files. DONE
// 2. Return file count to main function. DONE
// 3. Add accumulative_dir_count and count the directories found.
// 4. Iterate deeper into tree based on max_traversal_depth. This should
// probably be done by defining the max traversal depth in the main function, removing
// it from the struct, and instantiating a new TreeTraverser for each depth level.
// This may remove the complexity required to handle the depth logic in traverse().
impl<'a> TreeTraverser<'a> {
    pub fn new(
        p_path: &'a str,
        p_max_traversal_depth: u32,
        p_current_traversal_depth: usize,
        p_accumulative_file_count: usize,
        p_accumulative_dir_count: usize,
        p_format_specifier: &'a str,
    ) -> TreeTraverser<'a> {
        let path = p_path;
        let max_traversal_depth = p_max_traversal_depth;
        let current_traversal_depth = p_current_traversal_depth;
        let accumulative_file_count = p_accumulative_file_count;
        let accumulative_dir_count = p_accumulative_dir_count;
        let format_specifier = p_format_specifier;

        TreeTraverser {
            path,
            max_traversal_depth,
            current_traversal_depth,
            accumulative_file_count,
            accumulative_dir_count,
            format_specifier,
        }
    }

    pub fn traverse(&mut self) {
        let entries = fs::read_dir(self.path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let indent = "-".repeat(self.current_traversal_depth);

            if self.format_specifier == "--paths" {
                println!("|{}{}", indent, entry.path().display());
            } else {
                println!("|{}{}", indent, entry.file_name().display());
            }

            if path.is_dir() {
                self.accumulative_dir_count += 1;
            } else {
                self.accumulative_file_count += 1;
            }
        }
    }
}
