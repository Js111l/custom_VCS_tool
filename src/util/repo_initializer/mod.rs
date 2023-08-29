use crate::FileCreator;
use std::fs;
use std::path::Path;

pub struct RepositoryInitializer;

impl RepositoryInitializer {
    pub fn init(self, option: &str, option_argument: &str) {
        let mut path = "";
        if option.eq("-p") {
            path = option_argument;
        }
        let repo_path = Path::new(path);
        if fs::create_dir(repo_path).is_err() {
            println!("The repo already exists")
        }
        if self.create_root_git_dir(path.to_string()).is_err() {
            println!("The repo already exists")
        }
        self.setup_directories(format!("{}{}", path, "\\git").as_str());
    }

    fn create_root_git_dir(&self, path: String) -> std::io::Result<()> {
        let git_path = path + "\\git";
        fs::create_dir(Path::new(git_path.as_str()))?;
        Ok(())
    }
    fn create_head_file(&self, git_path: &str) -> std::io::Result<()> {
        let creator = FileCreator;
        creator.create_file(format!("{}{}", git_path, "\\HEAD").as_str());
        Ok(())
    }

    fn create_objects_dir(&self, git_path: &str) -> std::io::Result<()> {
        fs::create_dir(Path::new(format!("{}{}", git_path, "\\objects").as_str()))?;
        fs::create_dir(Path::new(
            format!("{}{}", git_path, "\\objects\\info").as_str(),
        ))?;
        fs::create_dir(Path::new(
            format!("{}{}", git_path, "\\objects\\pack").as_str(),
        ))?;
        Ok(())
    }
    fn create_config_dir(&self, git_path: &str) -> std::io::Result<()> {
        fs::create_dir(Path::new(format!("{}{}", git_path, "\\config").as_str()))?;
        Ok(())
    }
    fn create_refs_dir(&self, git_path: &str) -> std::io::Result<()> {
        fs::create_dir(Path::new(format!("{}{}", git_path, "\\refs").as_str()))?;
        fs::create_dir(Path::new(
            format!("{}{}", git_path, "\\refs\\heads").as_str(),
        ))?;
        fs::create_dir(Path::new(
            format!("{}{}", git_path, "\\refs\\tags").as_str(),
        ))?;
        Ok(())
    }
    fn setup_directories(&self, git_path: &str) {
        if let Err(err) = self.create_head_file(git_path) {
            println!("{}", err); // TODO
        }
        if let Err(err) = self.create_config_dir(git_path) {
            println!("{}", err);
        }
        if let Err(err) = self.create_objects_dir(git_path) {
            println!("{}", err);
        }
        if let Err(err) = self.create_refs_dir(git_path) {
            println!("{}", err);
        }
    }
}
