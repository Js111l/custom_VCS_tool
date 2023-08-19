use serde::Serialize;
use sha1::{Digest, Sha1};

pub(crate) struct ObjectHasher;

impl ObjectHasher {
    pub fn get_hash(&self, byte_array: Vec<u8>) -> String {
        let mut hasher = Sha1::new();
        hasher.update(&byte_array);
        let result = hasher.finalize();
        return format!("{:x}", result);
    }
    // pub fn get_hash_string(&self, str: String) -> String {
    //     let mut hasher = Sha1::new();
    //     hasher.update(str);
    //     let result = hasher.finalize();
    //     return format!("{:x}", result);
    // }
    pub fn get_path_from_hash(&self, hash: &str) -> String {
        let dir = &hash[0..2];
        let file_name = &hash[2..];
        return format!("{}\\{}", dir, file_name);
    }
}
