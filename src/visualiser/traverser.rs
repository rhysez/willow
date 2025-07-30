use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

// Using string slices so that struct does not take ownership.
// Also passing generic lifetime that origin_path and current_file_name share.
pub struct TreeTraverser<'a> {
    root_path: PathBuf, // The starting path that Willow starts traversing from.
    pub max_traversal_depth: usize, // The maximum traversal depth allowed.
    current_traversal_depth: usize, // The current depth of the traversal at some point in time.
    pub accumulative_file_count: usize, // The total number of files found thus far.
    pub accumulative_dir_count: usize, // The total number of directories found thus far.
    format_specifier: &'a str, // Whether the program should print entries as paths.
}

impl<'a> TreeTraverser<'a> {
    pub fn new(
        p_root_path: PathBuf,
        p_max_traversal_depth: usize,
        p_current_traversal_depth: usize,
        p_accumulative_file_count: usize,
        p_accumulative_dir_count: usize,
        p_format_specifier: &'a str,
    ) -> TreeTraverser<'a> {
        let root_path = p_root_path;
        let max_traversal_depth = p_max_traversal_depth;
        let current_traversal_depth = p_current_traversal_depth;
        let accumulative_file_count = p_accumulative_file_count;
        let accumulative_dir_count = p_accumulative_dir_count;
        let format_specifier = p_format_specifier;

        TreeTraverser {
            root_path,
            max_traversal_depth,
            current_traversal_depth,
            accumulative_file_count,
            accumulative_dir_count,
            format_specifier,
        }
    }

    fn get_entries(&self, path: &Path) -> std::fs::ReadDir {
        match fs::read_dir(path) {
            Ok(values) => values,
            Err(_) => panic!("The program was unable to read the file tree."),
        }
    }

    fn calculate_and_assign_depth(&mut self, entry: &DirEntry) -> usize {
        let depth = entry.path().components().count() - self.root_path.components().count();
        self.current_traversal_depth = depth;

        self.current_traversal_depth
    }

    fn format_leaf(&self, entry: &DirEntry, path: &Path) {
        let indent: String = " ".repeat(self.current_traversal_depth);

        if self.format_specifier == "--paths" {
            println!("{}|-{}", indent, entry.path().display());
        } else if path.is_dir() {
            println!("{}|-{}/", indent, entry.file_name().display());
        } else {
            println!("{}|-{}", indent, entry.file_name().display());
        }
    }

    fn can_traverse_next_depth_level(&self) -> bool {
        self.current_traversal_depth < self.max_traversal_depth
    }

    // TODO:
    // Allow max depth to be defined in runtime args.
    pub fn traverse(&mut self, path: &Path) {
        let entries = self.get_entries(path);

        for entry in entries {
            let entry = match entry {
                Ok(value) => value,
                Err(_) => continue,
            };

            let path = entry.path();
            let file_name = match entry.file_name().into_string() {
                Ok(string) => string,
                Err(_) => continue,
            };

            // Skip dotfiles
            if file_name.starts_with(".") {
                continue;
            }

            self.calculate_and_assign_depth(&entry);
            self.format_leaf(&entry, &path);

            if path.is_dir() {
                self.accumulative_dir_count += 1;
                if self.can_traverse_next_depth_level() {
                    self.traverse(&path);
                }
            } else {
                self.accumulative_file_count += 1;
            }
        }
    }
}
