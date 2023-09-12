use crate::model::git_object::{Node, Tree};
use std::fs;
use std::fs::ReadDir;

pub fn get_tree_from_dir(read_dir: ReadDir) -> Tree {
    let mut root_node = Node {
        name: "root".to_string(),
        content: vec![],
        children: vec![],
    };

    read_dir.for_each(|element| {
        let entry = element.unwrap();
        let path = entry.path();
        let file_name: String = path.file_name().unwrap().to_str().unwrap().to_string();
        if path.is_file() {
            &root_node.add(Node {
                name: file_name,
                content: fs::read(path).unwrap(),
                children: vec![],
            });
        } else {
            let dir_node = get_node_from_sub_dir(fs::read_dir(path).unwrap(), file_name);
            &root_node.add(dir_node);
        }
    });

    return Tree {
        name: "root".to_string(),
        root: root_node,
    };
}

fn get_node_from_sub_dir(read_dir: ReadDir, dir_name: String) -> Node {
    let mut root = Node {
        name: dir_name,
        content: vec![],
        children: vec![],
    };

    read_dir.for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if path.is_file() {
            &root.add(Node {
                name: file_name,
                content: fs::read(path).unwrap(),
                children: vec![],
            });
        } else {
            &root.add(get_node_from_sub_dir(
                fs::read_dir(path).unwrap(),
                file_name,
            ));
        }
    });

    return root;
}
