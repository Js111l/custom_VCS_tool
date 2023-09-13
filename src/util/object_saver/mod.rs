use crate::model::git_object::GitObject;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_serializer::ObjectSerializer;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct ObjectSaver;

impl ObjectSaver {
    pub fn save<T: GitObject + Serialize>(&self, git_object: &T, git_path: &str) {
        let byte_array = ObjectSerializer::serialize_object(git_object);
        let hasher = ObjectHasher;
        let hash = hasher.get_hash(byte_array);
        let path_from_hash = hasher.get_path_from_hash(&hash);
        self.create_sub_dir(
            format!(
                "{}{}{}",
                git_path,
                "\\objects\\".to_string(),
                &path_from_hash[0..2]
            )
            .as_str(),
        );
        let path = format!(
            "{}{}{}",
            git_path,
            "\\objects\\".to_string(),
            path_from_hash
        );
        match File::create(&path) {
            Ok(mut file) => {
                file.write_all(&ObjectSerializer::serialize_object(git_object));
            }
            Err(_) => {
                fs::remove_file(&path).unwrap();
                let mut file = File::create(&path).unwrap();
                file.write_all(&ObjectSerializer::serialize_object(git_object));
            }
        };
    }
    fn create_sub_dir(&self, path: &str) {
        println!("{}", path);
        fs::create_dir(path).unwrap();
    }
}
