use crate::model::git_object::GitObject;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_saver::ObjectSaver;
use crate::util::object_serializer::ObjectSerializer;
use crate::FileCreator;
use serde::Serialize;
use std::fs;

struct ObjectAdder {}

impl ObjectAdder {
    fn objects_to_add<T: GitObject + Serialize>(&self, objects_to_add: Vec<T>, git_path: &str) {
        let saver = ObjectSaver;
        objects_to_add.iter().for_each(|object| {
            saver.save(object);
            let byte_array = ObjectSerializer::serialize_object(object);
            let hasher = ObjectHasher;
            let hash = hasher.get_hash(byte_array);
            if let Err(err) = fs::write(git_path, format!("{} {}", hash, git_path)) {
                println!("unexpected error {}", err)
            }
        });
    }

    fn create_index_file(&self, git_path: &str) -> std::io::Result<()> {
        let creator = FileCreator;
        creator.create_file(format!("{}{}", git_path, "\\index").as_str());
        Ok(())
    }
}
