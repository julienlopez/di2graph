use crate::error::Error;
use crate::project_map::ProjectMap;

pub trait DiParser {
    fn analyze_dir(&self) -> Result<ProjectMap, Error>;
}
