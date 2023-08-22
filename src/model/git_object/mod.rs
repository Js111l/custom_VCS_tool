use serde_derive::{Deserialize, Serialize};

pub trait GitObject {
    fn set_content(self, content: Vec<u8>);
    fn get_content(self) -> Vec<u8>;
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Blob {
    pub(crate) content: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Tree {
    pub(crate) content: Vec<u8>,
}

impl GitObject for Blob {
    fn set_content(mut self, content: Vec<u8>) {
        self.content = content
    }

    fn get_content(self) -> Vec<u8> {
        self.content
    }
}

impl GitObject for Tree {
    fn set_content(mut self, content: Vec<u8>) {
        self.content = content;
    }

    fn get_content(self) -> Vec<u8> {
        self.content
    }
}
