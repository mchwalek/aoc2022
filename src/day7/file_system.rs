use std::collections::{HashMap, hash_map::Entry};

pub struct FileSystem {
    current_dir_id: usize,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl FileSystem {
    const ROOT_DIR_ID: usize = 0;

    pub fn new() -> Self {
        FileSystem {
            current_dir_id: Self::ROOT_DIR_ID,
            files: Vec::new(),
            dirs: vec![Dir::new("/".to_string(), None)]
        }
    }

    pub fn add_dir(&mut self, name: String) -> Result<(), String> {
        self.dirs.push(Dir::new(name.clone(), Some(self.current_dir_id)));

        let new_entry_index = self.dirs.len() - 1;
        let current_dir = &mut self.dirs[self.current_dir_id];
        match current_dir.dir_lookup.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(new_entry_index);
            }
            Entry::Occupied(_) => {
                return Err(format!("dir '{}' already exists", name.clone()));
            }

        }

        Ok(())
    }

    pub fn add_file(&mut self, name: String, size: usize) -> Result<(), String> {
        self.files.push(File { name: name.clone(), size });

        let new_entry_index = self.files.len() - 1;
        let current_dir = &mut self.dirs[self.current_dir_id];
        match current_dir.file_lookup.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(new_entry_index);
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
                self.current_dir_id = Self::ROOT_DIR_ID;
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
                    .ok_or(format!("dir '{}' doesn\'t exist", dir))?;
            }
        }

        Ok(())
    }

    fn string_representation(&self) -> String {
        let mut result = String::new();
        self.dir_string_representation(&mut result, &self.dirs[Self::ROOT_DIR_ID], 0);
        result
    }

    fn dir_string_representation(&self, result: &mut String, dir: &Dir, indent: usize) {
        result.push_str(&format!("{}dir {}\n", " ".repeat(indent), dir.name));
        for id in Self::sorted_values(&dir.file_lookup) {
            let file = &self.files[id];
            result.push_str(&format!("{}{} {}\n", " ".repeat(indent + 2), file.size, file.name));
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

#[cfg(test)]
mod tests {
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
        expected_representation.push_str("dir /\n");
        expected_representation.push_str("  12 a\n");
        expected_representation.push_str("  90 c\n");
        expected_representation.push_str("  dir b\n");
        expected_representation.push_str("    34 a\n");
        expected_representation.push_str("    78 c\n");
        expected_representation.push_str("    dir b\n");
        expected_representation.push_str("      56 a\n");

        assert_eq!(expected_representation, fs.string_representation());
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
        expected_representation.push_str("dir /\n");
        expected_representation.push_str("  12 a\n");
        
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
        expected_representation.push_str("dir /\n");
        expected_representation.push_str("  dir a\n");
        expected_representation.push_str("    dir a\n");
        
        assert_eq!(expected_representation, fs.string_representation());
    }
}