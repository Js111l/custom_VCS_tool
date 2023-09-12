use crate::model::git_object::GitObject;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_saver::ObjectSaver;
use crate::util::object_serializer::ObjectSerializer;
use crate::FileCreator;
use serde::Serialize;
use std::fs;

pub(crate) struct ObjectAdder {}

impl ObjectAdder {
    pub fn new() -> ObjectAdder {
        return Self {};
    }
    pub fn save_scanned_objects<T: GitObject + Serialize>(
        &self,
        objects_to_add: Vec<T>,
        git_path: &str,
    ) {
        if (self.create_index_file(git_path).is_err()) {
            println!("index aready exists!");
        };
        let saver = ObjectSaver;
        let mut hashes: Vec<String> = Vec::new();

        objects_to_add.iter().for_each(|object| {
            saver.save(object, git_path);
            let byte_array = ObjectSerializer::serialize_object(object);
            let hasher = ObjectHasher;

            let hash = hasher.get_hash(byte_array);
            hashes.push(hash.clone());

            let pathFromHash = hasher.get_path_from_hash(hash.as_str());
            let path = format!("{}{}{}", git_path, "\\objects\\".to_string(), pathFromHash);
            if let Err(err) = fs::write(
                format!("{}{}", git_path, "\\index"),
                format!("{} {}", hash, path),
            ) {
                println!("{}", format!("{} {}", hash, path));
                println!("{}", format!("{}{}", git_path, "\\index"));
            }
        });
    }

    fn create_index_file(&self, git_path: &str) -> std::io::Result<()> {
        let creator = FileCreator;
        creator.create_file(format!("{}{}", git_path, "\\index").as_str());
        Ok(())
    }
}
