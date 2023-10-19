use std::collections::HashMap;

pub struct FileSystem {
    current_dir_id: usize,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            current_dir_id: 0,
            files: Vec::new(),
            dirs: vec![Dir::new("/".to_string())]
        }
    }

    pub fn add_dir(&mut self, name: String) {
        self.dirs.push(Dir::new(name.clone()));

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

    pub fn cd(&mut self, dir: String) {
        if dir == "/" {
            self.current_dir_id = 0;
        } else {
            let current_dir = &self.dirs[self.current_dir_id];
            self.current_dir_id = current_dir.dir_lookup[&dir]; // TODO: handle invalid dirs
        }
    }

    fn string_representation(&self) -> String {
        let mut result = String::new();
        self.dir_string_representation(&mut result, &self.dirs[0], 0);
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
    dir_lookup: HashMap<String, usize>,
    file_ids: Vec<usize>,
}

impl Dir {
    fn new(name: String) -> Self {
        Dir { name, dir_lookup: HashMap::new(), file_ids: Vec::new() }
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
        fs.cd("/".to_string());
        fs.add_file("a".to_string(), 123);
        fs.add_dir("b".to_string());
        fs.cd("b".to_string());
        fs.add_file("a".to_string(), 456);

        let mut expected_representation = String::new();
        expected_representation.push_str("dir /\n");
        expected_representation.push_str("  123 a\n");
        expected_representation.push_str("  dir b\n");
        expected_representation.push_str("    456 a\n");

        assert_eq!(expected_representation, fs.string_representation());
    }
}