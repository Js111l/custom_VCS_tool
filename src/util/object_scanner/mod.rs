use crate::model::git_object::{Blob};
use std::fs;
use std::path::{PathBuf};

pub struct ObjectScanner {}
impl ObjectScanner {
    pub fn new() -> ObjectScanner {
        return Self {};
    }
    pub fn scan_objects(&mut self, path: &str) -> Vec<Blob> {
        let mut blobs: Vec<Blob> = Vec::new();
        let read = fs::read_dir(path).unwrap();
        for entry in read {
            let entryPath = entry.unwrap().path();
            let fileName = entryPath.file_name().unwrap().to_str().unwrap().to_string();
            let isNotGit = !fileName.eq("giter");
            if (entryPath.is_file()) {
                blobs.push(Blob {
                    name: fileName,
                    content: fs::read(&entryPath).unwrap(),
                });
            }
            if (entryPath.is_dir() && isNotGit) {
                processDir(entryPath, &mut blobs);
            }
        }
        return blobs;
    }
}

fn processDir(path: PathBuf, blobs: &mut Vec<Blob>) {
    let readDir = fs::read_dir(path).unwrap();

    for entry in readDir {
        let path = entry.unwrap().path();
        let fileName = path.file_name().unwrap().to_str().unwrap().to_string();
        if (path.is_file()) {
            blobs.push(Blob {
                name: fileName,
                content: fs::read(&path).unwrap(),
            })
        }
        if (path.is_dir()) {
            processDir(path, blobs);
        }
    }
}
