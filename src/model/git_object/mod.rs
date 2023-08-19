use serde_derive::{Deserialize, Serialize};

pub trait GitObject {}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Blob {
    pub(crate) content: Vec<u8>
}

impl GitObject for Blob {}
