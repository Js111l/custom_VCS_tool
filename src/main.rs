use crate::model::git_object::{Blob, GitObject};
use crate::util::directory_tree::get_tree_from_dir;
use crate::util::object_adder::ObjectAdder;
use crate::util::object_diff_checker::check_diff;
use crate::util::object_scanner::ObjectScanner;
use crate::util::repo_initializer::RepositoryInitializer;
use sha1::digest::typenum::private::IsEqualPrivate;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::ptr::null;

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
    // while true {
    //     input.clear();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     processCommand(&input);
    // }
    // REPO INIT
    // let repo_init: RepositoryInitializer = RepositoryInitializer;
    // repo_init.init("-p", "C:\\Users\\kubas\\Desktop\\llllll");
    //
    // TREE
    // let xx = fs::read_dir("C:\\Users\\kubas\\Desktop\\llllll").unwrap();
    // let x = get_tree_from_dir(xx);
    // println!("{:?}", x);

    //let readDir = fs::read_dir(Path::new("C:\\Users\\kubas\\Desktop\\llllll")).unwrap();
    let mut scanner = ObjectScanner::new();
    let blobs = scanner.scan_objects("C:\\Users\\kubas\\Desktop\\llllll");
    blobs.iter().for_each(|x| {
        //println!(x.name);
    });
    
    //let adder = ObjectAdder::new();
    //adder.save_scanned_objects(blobs.to_vec(), "C:\\Users\\kubas\\Desktop\\llllll\\giter");

    let newBlobs = scanner.scan_objects("C:\\Users\\kubas\\Desktop\\llllll2");

    check_diff(blobs, newBlobs);
    // let mut files: Vec<File> = Vec::new();

    // TODO
}

fn processCommand(input: &String) {
    let strList = input.split(" ").collect::<Vec<&str>>();
    let command = strList[0];
    let rootDirPath = strList[2];
    match command {
        "init" => {
            let initializer = RepositoryInitializer::new();
            initializer.init(strList[1], strList[2]);
        }
        "add" => {
            let readDir = fs::read_dir(Path::new(rootDirPath)).unwrap();
            let mut scanner = ObjectScanner::new();
            let blobs = scanner.scan_objects(rootDirPath);
            let adder = ObjectAdder::new();
            adder.save_scanned_objects(blobs, rootDirPath);
        }
        "commit" => {}
        _ => {} // TODO PROCESS COMMANDS!
    }
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

//let byte_array = ObjectSerializer::serialize_object(&obj);
// let saver = ObjectSaver;
// let obj2 = Blob {
//     id: 12,
//     key: 12,
//     name: "lalala".to_string(),
// };
// saver.save(obj2);
// let content = fs::
// read(Path::new("C:\\Users\\kubas\\Desktop\\llllll\\git\\objects\\76\\9db3d79abef36b10a316ce58cd85b06b199bc7")).unwrap();
// let blob: Blob = bincode::deserialize(&content).unwrap();
// let scanner = ObjectScanner;
// scanner.scan_objects("C:\\Users\\kubas\\Desktop\\llllll");
