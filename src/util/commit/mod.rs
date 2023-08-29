use crate::util::directory_tree::get_tree_from_dir;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_saver::ObjectSaver;
use crate::util::object_serializer::ObjectSerializer;
use std::fs;

struct Commiter;

impl Commiter {
    fn create_commit(&self, dir_path: String) {
        let read_dir = fs::read_dir(dir_path).unwrap();
        let dir_tree = get_tree_from_dir(read_dir);
        let byte_array = ObjectSerializer::serialize_object(dir_tree);
        let saver = ObjectSaver;
        saver.save(dir_tree);
        // if let Err(err) = fs::write(path, format!("{} {}", hash, path)) {
        //     println!("unexpected error {}", err)
        // }
    }
}
