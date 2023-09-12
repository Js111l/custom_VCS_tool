use serde_derive::{Deserialize, Serialize};
pub trait ContentObject: GitObject {
    fn set_content(self, content: Vec<u8>);
    fn get_content(self) -> Vec<u8>;
}

pub trait GitObject {}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Tree {
    pub name: String,
    pub(crate) root: Node,
}
impl GitObject for Tree {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub(crate) name: String,
    pub(crate) content: Vec<u8>,
    pub(crate) children: Vec<Node>,
}
impl Node {
    pub(crate) fn add(&mut self, node: Node) {
        self.children.push(node);
    }
    fn append(&mut self, nodes: &mut Vec<Node>) {
        self.children.append(nodes);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Blob {
    pub name: String,
    pub(crate) content: Vec<u8>,
}

impl GitObject for Blob {}

impl ContentObject for Blob {
    fn set_content(mut self, content: Vec<u8>) {
        self.content = content
    }

    fn get_content(self) -> Vec<u8> {
        self.content
    }
}
