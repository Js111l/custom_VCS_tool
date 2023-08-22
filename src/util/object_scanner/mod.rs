use crate::model::git_object::Blob;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_serializer::ObjectSerializer;
use std::fs;
use std::fs::File;
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
            let mut blob_content: Vec<u8> = vec![];
            match fs::read(Path::new(&path_string)) {
                Ok(content) => {
                    blob_content = content;
                }
                Err(err) => {
                    println!("Unexpected error! {}", err);
                }
            };

            let blob = Blob {
                content: blob_content,
            };
            let hasher = ObjectHasher;

            self.objectHashes
                .push(hasher.get_hash(ObjectSerializer::serialize_object(&blob)));
        });

        self.objectHashes.iter().for_each(|f| {
            println!("{}", f);
        })
    }
}
