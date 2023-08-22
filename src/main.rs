use crate::model::git_object::Blob;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_saver::ObjectSaver;
use crate::util::object_scanner::ObjectScanner;
use crate::util::object_serializer::ObjectSerializer;
use sha1::digest::typenum::private::IsEqualPrivate;
use std::fs;
use std::fs::File;
use std::path::Path;

mod util {
    pub mod object_adder;
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
    // let repo_init: RepositoryInitializer = RepositoryInitializer;
    // repo_init.init("-p", "C:\\Users\\kubas\\Desktop\\llllll");
    //
    fs::read(Path::new("C:\\Users\\kubas\\Desktop\\llllll")).unwrap();

    // let mut staged_files: Vec<String> = Vec::new();
    // let mut untracked_files: Vec<&str> = Vec::new();
    // let xx = staged_files.to_owned();
    // let mut scanner = ObjectScanner::new(staged_files);
    // scanner.scan_objects("C:\\Users\\kubas\\Desktop\\llllll");
    // println!("{:?}", xx);
    // let obj = Blob {
    //   id: 12,
    //    key: 12,
    //     name: "lalala".to_string(),
    // };

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
}

struct FileCreator;

impl FileCreator {
    pub fn create_file(&self, path: &str) {
        if let Err(err) = File::create(path) {
            println!("Unexpected error! {}", err)
        }
    }
}
