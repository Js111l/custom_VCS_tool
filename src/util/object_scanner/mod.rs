use crate::model::git_object::Blob;
use std::fs;

pub struct ObjectScanner {}
impl ObjectScanner {
    pub fn new() -> ObjectScanner {
        return Self {};
    }
    pub fn scan_objects(&mut self, path: &str) -> Vec<Blob> {
        let mut blobs: Vec<Blob> = Vec::new();
        let read = fs::read_dir(path).unwrap();
        let strList = path.split("\\").collect::<Vec<&str>>();
        let rootDirIndex = strList.len() - 1;

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
                processDir(
                    entryPath.to_str().unwrap().to_string(),
                    &mut blobs,
                    rootDirIndex,
                );
            }
        }
        return blobs;
    }
}

fn processDir(path: String, blobs: &mut Vec<Blob>, rootDirIndex: usize) {
    let strList = path.split("\\").collect::<Vec<&str>>();
    let currDir = getCurrentDir(strList, rootDirIndex);

    let readDir = fs::read_dir(&path).unwrap();
    for entry in readDir {
        let entryPath = entry.unwrap().path();
        let fileName = entryPath.file_name().unwrap().to_str().unwrap().to_string();

        if (entryPath.is_file()) {
            blobs.push(Blob {
                name: format!("{}{}{}", currDir.clone(), "\\", fileName),
                content: fs::read(&entryPath).unwrap(),
            })
        }
        if (entryPath.is_dir()) {
            processDir(entryPath.to_str().unwrap().to_string(), blobs, rootDirIndex);
        }
    }
}

fn getCurrentDir(strList: Vec<&str>, rootDirIndex: usize) -> String {
    let mut index: usize = rootDirIndex;
    let mut result: Vec<String> = Vec::new();
    index = index + 1;
    while index < strList.len() {
        result.push(strList[index].to_string());
        index = index + 1;
    }
    return result.join("\\");
}
