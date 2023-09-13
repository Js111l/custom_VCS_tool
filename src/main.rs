use crate::model::git_object::{Blob, GitObject};
use crate::util::commit::Commiter;
use crate::util::object_adder::ObjectAdder;
use crate::util::object_diff_checker::check_diff;
use crate::util::object_scanner::ObjectScanner;
use crate::util::repo_initializer::RepositoryInitializer;
use sha1::digest::typenum::private::IsEqualPrivate;
use std::fs::File;

mod util {
    pub mod commit;
    pub mod directory_tree;
    pub mod object_adder;
    pub mod object_diff_checker;
    pub mod object_hasher;
    pub mod object_saver;
    pub mod object_scanner;
    pub mod object_serializer;
    pub mod repo_initializer;
}

mod model {
    pub mod git_object;
}

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    let mut rootDirPath: &mut String = &mut "".to_string();
    let mut blobs: Vec<Blob> = Vec::new();
    let mut newBlobsAddedFlag = false;
    while true {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        processCommand(&input, &mut rootDirPath, &mut blobs, &mut newBlobsAddedFlag);
    }
}

fn processCommand(
    input: &String,
    rootDirPath: &mut String,
    blobs: &mut Vec<Blob>,
    newBlobsAddedFlag: &mut bool,
) {
    let strList = input.split(" ").collect::<Vec<&str>>();
    let command = strList[0].trim();
    match command {
        "init" => {
            let initializer = RepositoryInitializer::new();
            initializer.init(strList[1], strList[2]);
            *rootDirPath = strList[2].to_string();
        }
        "add" => {
            if (status(blobs, rootDirPath).is_empty()) {
                println!("nothing to add");
                return;
            }

            let mut scanner = ObjectScanner::new();
            let mut newObjects: Vec<Blob> = Vec::new();
            let adder = ObjectAdder::new();

            if (!&blobs.is_empty()) {
                scanner
                    .scan_objects(&rootDirPath)
                    .into_iter()
                    .for_each(|x| {
                        if (!blobs.contains(&x)) {
                            newObjects.push(x);
                        }
                    });
                adder.save_scanned_objects(
                    &mut newObjects,
                    format!("{}{}", &rootDirPath, "\\git").as_str(),
                );
                blobs.append(&mut newObjects);
                *newBlobsAddedFlag = true;
            } else {
                *blobs = scanner.scan_objects(&rootDirPath);
                adder.save_scanned_objects(blobs, format!("{}{}", &rootDirPath, "\\git").as_str());
                *newBlobsAddedFlag = true;
            }
        }
        "commit" => {
            let commiter = Commiter::new();
            let diffs = status(blobs, rootDirPath);
            if (diffs.is_empty() && !*newBlobsAddedFlag) {
                println!("nothing to commit, working tree clean");
                return;
            } else {
                commiter.create_commit(
                    rootDirPath.to_string(),
                    format!("{}{}", &rootDirPath, "\\git"),
                );
                *newBlobsAddedFlag = false;
            }
        }
        "status" => {
            let diffs = status(blobs, rootDirPath);
            if (diffs.is_empty()) {
                println!("nothing to commit, working tree clean");
            }
        }
        _ => {
            println!("Cannot execute command: {}", command);
        }
    }
}

fn status(blobs: &mut Vec<Blob>, rootDirPath: &mut String) -> Vec<Blob> {
    let mut scanner = ObjectScanner::new();
    let newBlobs = scanner.scan_objects(&rootDirPath);
    return check_diff(blobs, newBlobs);
}

#[derive(Debug, Clone)]
pub struct Node {
    pub content: String,
    pub children: Vec<Node>,
}

struct FileCreator;

impl FileCreator {
    pub fn create_file(&self, path: &str) {
        if let Err(err) = File::create(path) {
            println!("Unexpected error! {}", err);
        }
    }
}
