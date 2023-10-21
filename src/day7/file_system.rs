use std::collections::HashMap;

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

    pub fn add_dir(&mut self, name: String) {
        self.dirs.push(Dir::new(name.clone(), Some(self.current_dir_id)));

        let new_entry_index = self.dirs.len() - 1;
        let current_dir = &mut self.dirs[self.current_dir_id];
        current_dir.dir_lookup.insert(name.clone(), new_entry_index);
    }

    pub fn add_file(&mut self, name: String, size: usize) {
        self.files.push(File { name, size });

        let new_entry_index = self.files.len() - 1;
        let current_dir = &mut self.dirs[self.current_dir_id];
        current_dir.file_ids.push(new_entry_index);
    }

    pub fn cd(&mut self, dir: &str) {
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
                self.current_dir_id = current_dir.dir_lookup[dir]; // TODO: handle invalid dirs
            }
        }
    }

    fn string_representation(&self) -> String {
        let mut result = String::new();
        self.dir_string_representation(&mut result, &self.dirs[Self::ROOT_DIR_ID], 0);
        result
    }

    fn dir_string_representation(&self, result: &mut String, dir: &Dir, indent: usize) {
        result.push_str(&format!("{}dir {}\n", " ".repeat(indent), dir.name));
        for id in dir.file_ids.iter() {
            let file = &self.files[*id];
            result.push_str(&format!("{}{} {}\n", " ".repeat(indent + 2), file.size, file.name));
        }

        for id in dir.dir_lookup.values() {
            let dir = &self.dirs[*id];
            self.dir_string_representation(result, dir, indent + 2);
        }
    }

}

struct Dir {
    name: String,
    parent_id: Option<usize>,
    dir_lookup: HashMap<String, usize>,
    file_ids: Vec<usize>,
}

impl Dir {
    fn new(name: String, parent_id: Option<usize>) -> Self {
        Dir { name, parent_id, dir_lookup: HashMap::new(), file_ids: Vec::new() }
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
        fs.add_file("a".to_string(), 12);
        fs.add_dir("b".to_string());

        // path: /b
        fs.cd("b");
        fs.add_file("a".to_string(), 34);
        fs.add_dir("b".to_string());

        // path: /b/b
        fs.cd("b");
        fs.add_file("a".to_string(), 56);

        // path: /b
        fs.cd("..");
        fs.add_file("c".to_string(), 78);

        // path: /
        fs.cd("/");
        fs.add_file("c".to_string(), 90);

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
}