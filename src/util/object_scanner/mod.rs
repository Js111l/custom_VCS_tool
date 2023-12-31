use crate::model::git_object::Blob;
use crate::util::object_hasher::ObjectHasher;
use std::fs;
use std::path::Path;

pub struct ObjectScanner {
    objectHashes: Vec<String>,
}

impl ObjectScanner {
    pub fn new(objectHashes: Vec<String>) -> ObjectScanner {
        ObjectScanner { objectHashes }
    }
    pub fn scan_objects(&mut self, path: &str) {
        fs::read_dir(path).unwrap().for_each(|p| {
            let path_string = p.unwrap().path().as_path().to_string_lossy().into_owned();
            let blob = Blob {
                content: fs::read(Path::new(&path_string)).unwrap(),
            };
            let hasher = ObjectHasher;

            //  self.objectHashes
            //   .push(hasher.get_hash(ObjectSerializer::serialize_object(&blob)));
        });

        self.objectHashes.iter().for_each(|f| {
            println!("{}", f);
        })
    }
}
