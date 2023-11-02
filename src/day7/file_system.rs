use std::collections::VecDeque;
use std::collections::{HashMap, hash_map::Entry};
use std::fmt::Write;

const ROOT_DIR_ID: usize = 0;

pub struct FileSystem {
    current_dir_id: usize,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            current_dir_id: ROOT_DIR_ID,
            files: Vec::new(),
            dirs: vec![Dir::new("/".to_string(), None)]
        }
    }

    pub fn add_dir(&mut self, name: String) -> Result<(), String> {
        let new_entry_index = self.dirs.len();
        let current_dir = &mut self.dirs[self.current_dir_id];
        match current_dir.dir_lookup.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(new_entry_index);
                self.dirs.push(Dir::new(name.clone(), Some(self.current_dir_id)));
            }
            Entry::Occupied(_) => {
                return Err(format!("dir '{}' already exists", name));
            }
        }

        Ok(())
    }

    pub fn add_file(&mut self, name: String, size: usize) -> Result<(), String> {
        let new_entry_index = self.files.len();
        let current_dir = &mut self.dirs[self.current_dir_id];
        match current_dir.file_lookup.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(new_entry_index);
                self.files.push(File { name: name.clone(), size });
            }
            Entry::Occupied(_) => {
                return Err(format!("file '{}' already exists", name.clone()));
            }
        }

        Ok(())
    }

    pub fn cd(&mut self, dir: &str) -> Result<(), String> {
        match dir {
            "/" => {
                self.current_dir_id = ROOT_DIR_ID;
            },
            ".." => {
                let current_dir = &self.dirs[self.current_dir_id];

                if let Some(id) = current_dir.parent_id {
                    self.current_dir_id = id
                }
            }
            _ => {
                let current_dir = &self.dirs[self.current_dir_id];
                self.current_dir_id = *current_dir.dir_lookup.get(dir)
                    .ok_or_else(|| format!("dir '{}' doesn\'t exist", dir))?;
            }
        }

        Ok(())
    }

    pub fn string_representation(&self) -> String {
        let mut result = String::new();
        self.dir_string_representation(&mut result, &self.dirs[ROOT_DIR_ID], 0);
        result
    }

    pub fn dir_sizes_iter(&self) -> DirSizes {
        DirSizes::new(self)
    }

    fn dir_string_representation(&self, result: &mut String, dir: &Dir, indent: usize) {
        write!(result, "{}dir {}\n", " ".repeat(indent), dir.name).unwrap();

        for id in Self::sorted_values(&dir.file_lookup) {
            let file = &self.files[id];
            write!(result, "{}{} {}\n", " ".repeat(indent + 2), file.size, file.name).unwrap();
        }

        for id in Self::sorted_values(&dir.dir_lookup) {
            let dir = &self.dirs[id];
            self.dir_string_representation(result, dir, indent + 2);
        }
    }

    fn sorted_values(map: &HashMap<String, usize>) -> Vec<usize> {
        let mut values: Vec<_> = map.values().map(|x| *x).collect();
        values.sort();
        values
    }
}

struct Dir {
    name: String,
    parent_id: Option<usize>,
    dir_lookup: HashMap<String, usize>,
    file_lookup: HashMap<String, usize>,
}

impl Dir {
    fn new(name: String, parent_id: Option<usize>) -> Self {
        Dir { name, parent_id, dir_lookup: HashMap::new(), file_lookup: HashMap::new() }
    }
}

struct File {
    name: String,
    size: usize,
}

pub struct DirSizes<'a> {
    fs: &'a FileSystem,
    dir_stack: Vec<(usize, VecDeque<usize>)>
}

impl<'a> DirSizes<'a> {
    fn new(fs: &'a FileSystem) -> Self {
        let child_dir_ids= Self::child_dir_ids(fs, ROOT_DIR_ID);
        DirSizes { fs, dir_stack: vec![(ROOT_DIR_ID, child_dir_ids)] }
    }

    fn dir_size(&self, dir: &Dir) -> usize {
        let mut result = 0;

        for id in Self::sorted_values(&dir.file_lookup) {
            let file = &self.fs.files[id];
            result += file.size;
        }

        for id in Self::sorted_values(&dir.dir_lookup) {
            let dir = &self.fs.dirs[id];
            result += self.dir_size(dir);
        }

        result
    }

    fn sorted_values(map: &HashMap<String, usize>) -> Vec<usize> {
        let mut values: Vec<_> = map.values().map(|x| *x).collect();
        values.sort();
        values
    }

    fn fill_dir_stack(&mut self, dir_id: usize) {
        let mut child_dir_ids = Self::child_dir_ids(self.fs, dir_id);
        let child_dir_id_result = child_dir_ids.pop_front();

        self.dir_stack.push((dir_id, child_dir_ids));

        if child_dir_id_result.is_some() {
            self.fill_dir_stack(child_dir_id_result.unwrap())
        }
    }

    fn child_dir_ids(fs: &FileSystem, dir_id: usize) -> VecDeque<usize> {
        fs.dirs[dir_id].dir_lookup.values().map(|x| *x).collect()
    }
}

impl<'a> Iterator for DirSizes<'a> {
    type Item = (String, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let top_item_result = self.dir_stack.last_mut();
        // Stack is empty, meaning we visited all the dirs
        if top_item_result.is_none() {
            return None
        }

        let top_item = top_item_result.unwrap();
        let (_, child_ids) = top_item;
        let child_id_result = child_ids.pop_front();

        // Top dir is not empty, meaning we should visit all of its children
        if child_id_result.is_some() {
            // After filling the stack, we have guarantee that the top dir is a leaf
            self.fill_dir_stack(child_id_result.unwrap());
        }

        // Pop and yield top dir
        let (id, _) = &self.dir_stack.pop().unwrap();
        let dir = &self.fs.dirs[*id];
        Some((dir.name.clone(), self.dir_size(dir)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day7::file_system::*;

    #[test]
    fn adds_entry_to_current_dir() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_file("a".to_string(), 12).unwrap();
        fs.add_dir("b".to_string()).unwrap();

        // path: /b
        fs.cd("b").unwrap();
        fs.add_file("a".to_string(), 34).unwrap();
        fs.add_dir("b".to_string()).unwrap();

        // path: /b/b
        fs.cd("b").unwrap();
        fs.add_file("a".to_string(), 56).unwrap();

        // path: /b
        fs.cd("..").unwrap();
        fs.add_file("c".to_string(), 78).unwrap();

        // path: /
        fs.cd("/").unwrap();
        fs.add_file("c".to_string(), 90).unwrap();

        let mut expected_representation = String::new();
        write!(expected_representation, "dir /\n").unwrap();
        write!(expected_representation, "  12 a\n").unwrap();
        write!(expected_representation, "  90 c\n").unwrap();
        write!(expected_representation, "  dir b\n").unwrap();
        write!(expected_representation, "    34 a\n").unwrap();
        write!(expected_representation, "    78 c\n").unwrap();
        write!(expected_representation, "    dir b\n").unwrap();
        write!(expected_representation, "      56 a\n").unwrap();

        assert_eq!(expected_representation, fs.string_representation());
    }

    #[test]
    fn calculates_sizes_for_each_dir() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_file("a".to_string(), 12).unwrap();
        fs.add_file("b".to_string(), 34).unwrap();
        fs.add_dir("c".to_string()).unwrap();
        fs.add_dir("d".to_string()).unwrap();

        // path: /c
        fs.cd("c").unwrap();
        fs.add_file("a".to_string(), 56).unwrap();
        fs.add_dir("b".to_string()).unwrap();

        // path: /c/b
        fs.cd("b").unwrap();
        fs.add_file("a".to_string(), 78).unwrap();

        let mut expected_items = HashSet::new();
        expected_items.insert(("/".to_string(), 180));
        expected_items.insert(("c".to_string(), 134));
        expected_items.insert(("b".to_string(), 78));
        expected_items.insert(("d".to_string(), 0));

        assert_eq!(expected_items, fs.dir_sizes_iter().collect::<HashSet<_>>());
    }

    #[test]
    fn returns_error_on_invalid_cd() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_dir("a".to_string()).unwrap();

        assert_eq!(Err("dir 'invalid' doesn\'t exist".to_string()), fs.cd("invalid"));
    }

    #[test]
    fn returns_error_on_invalid_add_file() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_file("a".to_string(), 12).unwrap();

        assert_eq!(Err("file 'a' already exists".to_string()), fs.add_file("a".to_string(), 34));

        // check whether fs hasn't been changed
        let mut expected_representation = String::new();
        write!(expected_representation, "dir /\n").unwrap();
        write!(expected_representation, "  12 a\n").unwrap();
        
        assert_eq!(expected_representation, fs.string_representation());
    }

    #[test]
    fn returns_error_on_invalid_add_dir() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_dir("a".to_string()).unwrap();

        // path: /a
        fs.cd("a").unwrap();
        fs.add_dir("a".to_string()).unwrap();

        // path: /
        fs.cd("..").unwrap();
        assert_eq!(Err("dir 'a' already exists".to_string()), fs.add_dir("a".to_string()));

        // check whether fs hasn't been changed
        let mut expected_representation = String::new();
        write!(expected_representation, "dir /\n").unwrap();
        write!(expected_representation, "  dir a\n").unwrap();
        write!(expected_representation, "    dir a\n").unwrap();
        
        assert_eq!(expected_representation, fs.string_representation());
    }
}