use crate::model::git_object::GitObject;
use serde::Serialize;

pub struct ObjectSerializer {}

impl ObjectSerializer {
    pub fn serialize_object<T: GitObject + Serialize>(git_object: &T) -> Vec<u8> {
        return bincode::serialize(&git_object).unwrap();
    }
}
