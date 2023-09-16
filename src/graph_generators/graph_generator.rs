use crate::error::Error;
use crate::project_map::ProjectMap;

pub trait GraphGenerator {
    fn generate(&self, project: &ProjectMap) -> Result<(), Error>;
}
