use crate::model::git_object::Blob;

pub fn check_diff(oldBlobs: Vec<Blob>, newBlobs: Vec<Blob>) {
    let oldNames: Vec<String> = oldBlobs.iter().map(|x| x.name.to_string()).collect();
    newBlobs.iter().for_each(|newBlob| {
        if (!oldNames.contains(&newBlob.name)) {
            println!("New file: {}", newBlob.name);
        }
        oldBlobs.iter().for_each(|oldBlob| {
            if (oldBlob.name.eq(&newBlob.name) && oldBlob.content != newBlob.content) {
                println!("New file: {}", newBlob.name);
            }
        })
    });
}
