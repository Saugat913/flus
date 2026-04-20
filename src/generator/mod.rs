use crate::error::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GeneratorContext {
    project_name: String,
    base_path: PathBuf,
}

impl GeneratorContext {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            project_name: name,
            base_path: path,
        }
    }
}

pub trait Generator {
    fn run(&self, context: &GeneratorContext) -> Result<()>;
}

mod project;
pub use project::ProjectScaffolder;
