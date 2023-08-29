use crate::util::directory_tree::get_tree_from_dir;
use sha1::digest::typenum::private::IsEqualPrivate;
use std::fs;
use std::fs::File;

mod util {
    pub mod commit;
    pub mod directory_tree;
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
    // REPO INIT
    // let repo_init: RepositoryInitializer = RepositoryInitializer;
    // repo_init.init("-p", "C:\\Users\\kubas\\Desktop\\llllll");
    //
    // TREE
    // let xx = fs::read_dir("C:\\Users\\kubas\\Desktop\\llllll").unwrap();
    // let x = get_tree_from_dir(xx);
    // println!("{:?}", x);

    // let mut files: Vec<File> = Vec::new();
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
            println!("Unexpected error! {}", err)
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
