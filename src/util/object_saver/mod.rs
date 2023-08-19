use crate::model::git_object::GitObject;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_serializer::ObjectSerializer;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct ObjectSaver;

impl ObjectSaver {
    pub fn save<T: GitObject + Serialize>(&self, git_object: &T) {
        let byte_array = ObjectSerializer::serialize_object(git_object);
        let hasher = ObjectHasher;
        let hash = hasher.get_hash(byte_array);
        let path_from_hash = hasher.get_path_from_hash(&hash);
        self.create_sub_dir(
            format!(
                "C:\\Users\\kubas\\Desktop\\folder\\git\\objects\\{}",
                &path_from_hash[0..2]
            )
            .as_str(),
        );
        let path = format!(
            "C:\\Users\\kubas\\Desktop\\folder\\git\\objects\\{}",
            path_from_hash
        );
        let mut file = File::create(path).unwrap();
        file.write_all(&ObjectSerializer::serialize_object(git_object));
    }
    fn create_sub_dir(&self, path: &str) {
        fs::create_dir(path).unwrap();
    }
}
