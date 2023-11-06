use std::collections::VecDeque;
use std::collections::{hash_map::Entry, HashMap};
use std::path::PathBuf;

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
            dirs: vec![Dir::new("/".to_string(), None)],
        }
    }

    pub fn add_dir(&mut self, name: String) -> Result<(), String> {
        let new_entry_index = self.dirs.len();
        let current_dir = &mut self.dirs[self.current_dir_id];
        match current_dir.dir_lookup.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(new_entry_index);
                self.dirs
                    .push(Dir::new(name.clone(), Some(self.current_dir_id)));
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
                self.files.push(File {
                    name: name.clone(),
                    size,
                    parent_id: self.current_dir_id,
                });
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
            }
            ".." => {
                let current_dir = &self.dirs[self.current_dir_id];

                if let Some(id) = current_dir.parent_id {
                    self.current_dir_id = id
                }
            }
            _ => {
                let current_dir = &self.dirs[self.current_dir_id];
                self.current_dir_id = *current_dir
                    .dir_lookup
                    .get(dir)
                    .ok_or_else(|| format!("dir '{}' doesn\'t exist", dir))?;
            }
        }

        Ok(())
    }

    pub fn depth_first_dirs_iter(&self) -> impl Iterator<Item = &Dir> {
        DepthFirstDirs::new(self)
    }

    pub fn dirs_iter(&self) -> impl Iterator<Item = &Dir> {
        self.dirs.iter()
    }

    pub fn depth_first_files_iter(&self) -> impl Iterator<Item = &File> {
        DepthFirstFiles::new(self, DepthFirstDirs::new(self))
    }

    pub fn files_iter(&self) -> impl Iterator<Item = &File> {
        self.files.iter()
    }

    pub fn dir_size(&self, dir: &Dir) -> usize {
        let mut result = 0;

        for id in dir.file_lookup.values() {
            let file = &self.files[*id];
            result += file.size;
        }

        for id in dir.dir_lookup.values() {
            let dir = &self.dirs[*id];
            result += self.dir_size(dir);
        }

        result
    }

    pub fn dir_path(&self, dir: &Dir) -> String {
        let path_buf: PathBuf = self.dir_path_buf(dir);
        path_buf.to_string_lossy().to_string()
    }

    pub fn file_path(&self, file: &File) -> String {
        let parent_dir = &self.dirs[file.parent_id];
        let mut path_buf: PathBuf = self.dir_path_buf(parent_dir);
        path_buf.push(file.name.clone());

        path_buf.to_string_lossy().to_string()
    }

    fn dir_path_buf(&self, dir: &Dir) -> PathBuf {
        let mut parts = vec![dir.name.clone()];

        let mut visited_dir = dir;
        while let Some(dir) = visited_dir.parent_id.map(|x| &self.dirs[x]) {
            parts.push(dir.name.clone());
            visited_dir = dir;
        }

        parts.into_iter().rev().collect()
    }
}

#[derive(PartialEq, Debug)]
pub struct Dir {
    name: String,
    parent_id: Option<usize>,
    dir_lookup: HashMap<String, usize>,
    file_lookup: HashMap<String, usize>,
}

impl Dir {
    fn new(name: String, parent_id: Option<usize>) -> Self {
        Dir {
            name,
            parent_id,
            dir_lookup: HashMap::new(),
            file_lookup: HashMap::new(),
        }
    }
}

#[derive(PartialEq, Debug, Eq)]
pub struct File {
    name: String,
    size: usize,
    parent_id: usize,
}

pub struct DepthFirstDirs<'a> {
    fs: &'a FileSystem,
    dir_stack: Vec<(&'a Dir, VecDeque<&'a Dir>)>,
}

impl<'a> DepthFirstDirs<'a> {
    fn new(fs: &'a FileSystem) -> Self {
        let root_dir = &fs.dirs[ROOT_DIR_ID];
        let children_to_visit = Self::child_dirs(fs, root_dir);
        DepthFirstDirs {
            fs,
            dir_stack: vec![(root_dir, children_to_visit)],
        }
    }

    fn fill_dir_stack(&mut self, dir: &'a Dir) {
        let mut children_to_visit = Self::child_dirs(self.fs, dir);
        let visited_dir_result = children_to_visit.pop_front();

        self.dir_stack.push((dir, children_to_visit));

        if let Some(dir) = visited_dir_result {
            self.fill_dir_stack(dir)
        }
    }

    fn child_dirs(fs: &'a FileSystem, dir: &'a Dir) -> VecDeque<&'a Dir> {
        let mut keys: Vec<_> = dir.dir_lookup.keys().collect();
        keys.sort();

        keys.into_iter()
            .map(|x| &fs.dirs[dir.dir_lookup[x]])
            .collect()
    }
}

impl<'a> Iterator for DepthFirstDirs<'a> {
    type Item = &'a Dir;

    fn next(&mut self) -> Option<Self::Item> {
        let top_item_result = self.dir_stack.last_mut();
        // Stack is empty, meaning we visited all the dirs
        if top_item_result.is_none() {
            return None;
        }

        let top_item = top_item_result.unwrap();
        let (_, children_to_visit) = top_item;

        if let Some(dir) = children_to_visit.pop_front() {
            // After filling the stack, we have guarantee that the top dir has no children to visit
            self.fill_dir_stack(dir);
        }

        // Pop and yield top dir
        let (dir, _) = &self.dir_stack.pop().unwrap();
        Some(dir)
    }
}

pub struct DepthFirstFiles<'a> {
    fs: &'a FileSystem,
    dirs_iter: DepthFirstDirs<'a>,
    files_to_visit: VecDeque<&'a File>,
}

impl<'a> DepthFirstFiles<'a> {
    fn new(fs: &'a FileSystem, dirs_iter: DepthFirstDirs<'a>) -> Self {
        DepthFirstFiles {
            fs,
            dirs_iter,
            files_to_visit: VecDeque::new(),
        }
    }

    fn child_files(&self, dir: &Dir) -> VecDeque<&'a File> {
        let mut keys: Vec<_> = dir.file_lookup.keys().collect();
        keys.sort();

        keys.into_iter()
            .map(|x| &self.fs.files[dir.file_lookup[x]])
            .collect()
    }
}

impl<'a> Iterator for DepthFirstFiles<'a> {
    type Item = &'a File;

    fn next(&mut self) -> Option<Self::Item> {
        let file_to_visit_result = self.files_to_visit.pop_front();
        if file_to_visit_result.is_some() {
            return file_to_visit_result;
        }

        let dir_result = self.dirs_iter.next();
        if dir_result.is_none() {
            return None;
        }

        let mut dir = dir_result.unwrap();
        loop {
            let mut files = self.child_files(dir);
            if let Some(file) = files.pop_front() {
                self.files_to_visit = files;
                break Some(file);
            } else {
                dir = self.dirs_iter.next()?;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day7::file_system::*;

    #[test]
    fn yields_all_dirs() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_dir("a".to_string()).unwrap();
        fs.add_dir("b".to_string()).unwrap();

        // path: /b
        fs.cd("b").unwrap();
        fs.add_dir("c".to_string()).unwrap();

        let expected_items = [
            Dir::new("a".to_string(), Some(0)),
            Dir::new("c".to_string(), Some(2)),
            Dir {
                name: "b".to_string(),
                parent_id: Some(0),
                dir_lookup: [("c".to_string(), 3)].into_iter().collect(),
                file_lookup: HashMap::new(),
            },
            Dir {
                name: "/".to_string(),
                parent_id: None,
                dir_lookup: [("a".to_string(), 1), ("b".to_string(), 2)]
                    .into_iter()
                    .collect(),
                file_lookup: HashMap::new(),
            },
        ];
        let expected_refs: Vec<_> = expected_items.iter().collect();

        // Depth first order
        assert_eq!(
            expected_refs,
            fs.depth_first_dirs_iter().collect::<Vec<_>>()
        );

        // Arbitrary order
        for dir in fs.dirs_iter() {
            assert!(expected_refs.contains(&dir));
        }
    }

    #[test]
    fn yields_all_files() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_file("a".to_string(), 12).unwrap();
        fs.add_dir("b".to_string()).unwrap();
        fs.add_dir("c".to_string()).unwrap();

        // path: /c
        fs.cd("c").unwrap();
        fs.add_file("a".to_string(), 34).unwrap();
        fs.add_dir("b".to_string()).unwrap();

        // path: /c/b
        fs.cd("b").unwrap();
        fs.add_file("a".to_string(), 56).unwrap();

        // path: /c
        fs.cd("..").unwrap();
        fs.add_file("c".to_string(), 78).unwrap();

        // path: /
        fs.cd("/").unwrap();
        fs.add_file("c".to_string(), 90).unwrap();

        let expected_items = [
            File {
                name: "a".to_string(),
                size: 56,
                parent_id: 3,
            },
            File {
                name: "a".to_string(),
                size: 34,
                parent_id: 2,
            },
            File {
                name: "c".to_string(),
                size: 78,
                parent_id: 2,
            },
            File {
                name: "a".to_string(),
                size: 12,
                parent_id: 0,
            },
            File {
                name: "c".to_string(),
                size: 90,
                parent_id: 0,
            },
        ];
        let expected_refs: Vec<_> = expected_items.iter().collect();

        // Depth first order
        assert_eq!(
            expected_refs,
            fs.depth_first_files_iter().collect::<Vec<_>>()
        );

        // Arbitrary order
        for file in fs.files_iter() {
            assert!(expected_refs.contains(&file));
        }
    }

    #[test]
    fn calculates_path_and_size_for_dirs() {
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
        expected_items.insert(("/c".to_string(), 134));
        expected_items.insert(("/c/b".to_string(), 78));
        expected_items.insert(("/d".to_string(), 0));

        let result: HashSet<_> = fs
            .depth_first_dirs_iter()
            .map(|x| (fs.dir_path(x), fs.dir_size(x)))
            .collect();

        assert_eq!(expected_items, result);
    }

    #[test]
    fn calculates_path_and_size_for_files() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_file("a".to_string(), 12).unwrap();
        fs.add_dir("b".to_string()).unwrap();
        fs.add_dir("c".to_string()).unwrap();

        // path: /c
        fs.cd("c").unwrap();
        fs.add_file("a".to_string(), 34).unwrap();
        fs.add_dir("b".to_string()).unwrap();

        // path: /c/b
        fs.cd("b").unwrap();
        fs.add_file("a".to_string(), 56).unwrap();

        // path: /c
        fs.cd("..").unwrap();
        fs.add_file("c".to_string(), 78).unwrap();

        // path: /
        fs.cd("/").unwrap();
        fs.add_file("c".to_string(), 90).unwrap();

        let mut expected_items = HashSet::new();
        expected_items.insert(("/a".to_string(), 12));
        expected_items.insert(("/c".to_string(), 90));
        expected_items.insert(("/c/a".to_string(), 34));
        expected_items.insert(("/c/c".to_string(), 78));
        expected_items.insert(("/c/b/a".to_string(), 56));

        let result: HashSet<_> = fs
            .depth_first_files_iter()
            .map(|x| (fs.file_path(x), x.size))
            .collect();

        assert_eq!(expected_items, result);
    }

    #[test]
    fn returns_error_on_invalid_cd() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_dir("a".to_string()).unwrap();

        assert_eq!(
            Err("dir 'invalid' doesn\'t exist".to_string()),
            fs.cd("invalid")
        );
    }

    #[test]
    fn returns_error_on_invalid_add_file() {
        let mut fs = FileSystem::new();
        // path: /
        fs.add_file("a".to_string(), 12).unwrap();

        assert_eq!(
            Err("file 'a' already exists".to_string()),
            fs.add_file("a".to_string(), 34)
        );

        // check whether fs hasn't been changed
        let mut expected_dirs = HashSet::new();
        expected_dirs.insert("/".to_string());
        let result: HashSet<_> = fs.depth_first_dirs_iter().map(|x| fs.dir_path(x)).collect();
        assert_eq!(expected_dirs, result);

        let mut expected_files = HashSet::new();
        expected_files.insert("/a".to_string());
        let result: HashSet<_> = fs
            .depth_first_files_iter()
            .map(|x| fs.file_path(x))
            .collect();
        assert_eq!(expected_files, result);
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
        assert_eq!(
            Err("dir 'a' already exists".to_string()),
            fs.add_dir("a".to_string())
        );

        // check whether fs hasn't been changed
        let mut expected_dirs = HashSet::new();
        expected_dirs.insert("/".to_string());
        expected_dirs.insert("/a".to_string());
        expected_dirs.insert("/a/a".to_string());
        let result: HashSet<_> = fs.depth_first_dirs_iter().map(|x| fs.dir_path(x)).collect();
        assert_eq!(expected_dirs, result);
    }
}
