use crate::model::git_object::Blob;
use crate::util::object_hasher::ObjectHasher;
use crate::util::object_serializer::ObjectSerializer;
use std::fs;
use std::fs::{File, ReadDir};
use std::path::Path;

pub struct ObjectScanner {
    objectHashes: Vec<(String, String)>,
}

impl ObjectScanner {
    pub fn new(objectHashes: Vec<(String, String)>) -> ObjectScanner {
        ObjectScanner { objectHashes }
    }
    pub fn scan_objects(&mut self, path: &str) {}
}
