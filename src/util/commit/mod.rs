use crate::util::directory_tree::get_tree_from_dir;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_saver::ObjectSaver;
use crate::util::object_serializer::ObjectSerializer;
use crate::FileCreator;
use std::fmt::format;
use std::fs;
use std::fs::File;

pub(crate) struct Commiter;

impl Commiter {
    pub fn new() -> Commiter {
        return Self {};
    }
    pub fn create_commit(&self, dir_path: String, gitPath: String) {
        let read_dir = fs::read_dir(&dir_path).unwrap();
        let dir_tree = get_tree_from_dir(read_dir);
        let byte_array = ObjectSerializer::serialize_object(&dir_tree);
        let saver = ObjectSaver;
        saver.save(&dir_tree, &gitPath);

        let hasher = ObjectHasher;
        let hash = hasher.get_hash(byte_array);
        let path = hasher.get_path_from_hash(&hash);

        let branchName = "master";

        writeToIndexFile(&dir_path, &hash, &path);
        writeToRefs(&dir_path, &hash, &path, branchName);
    }
}

fn writeToRefs(dir_path: &String, hash: &String, path: &String, branchName: &str) {
    let filePath = format!(
        "{}{}{}{}{}",
        dir_path, "\\git\\", "refs\\", "heads\\", branchName
    )
    .to_string();
    println!("{}", filePath);

    let file = File::create(&filePath).unwrap();
    println!("{:?}", file);
    if let Err(err) = fs::write(&filePath, format!("{}", hash)) {
        println!("unexpected error {}", err) //todo
    }
}

fn writeToIndexFile(dir_path: &String, hash: &String, path: &String) {
    if let Err(err) = fs::write(
        format!("{}{}{}", dir_path, "\\git\\", "index").to_string(),
        format!("{} {}", hash, &path),
    ) {
        println!("unexpected error {}", err) //todo
    }
}
