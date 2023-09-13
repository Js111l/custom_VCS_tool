use crate::model::git_object::Blob;
use std::collections::HashMap;

pub fn check_diff(oldBlobs: &mut Vec<Blob>, newBlobs: Vec<Blob>) -> Vec<Blob> {
    let mut oldMap: HashMap<String, &mut Blob> = HashMap::new();
    oldBlobs.into_iter().for_each(|x| {
        oldMap.insert(x.name.clone(), x);
    });
    let mut differences: Vec<Blob> = Vec::new();
    newBlobs.into_iter().for_each(|x| {
        if (oldMap.get(&*x.name).is_none()) {
            differences.push(x);
        } else {
            let blob = oldMap.get(&*x.name).unwrap();
            if (blob.content != x.content) {
                differences.push(x);
            }
        }
    });
    
    differences.iter().for_each(|x| {
        println!("New file: {}", x.name);
    });
    return differences;
}
