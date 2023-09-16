use crate::error::Error;
use crate::graph_generators::graph_generator::GraphGenerator;
use crate::project_map::ProjectMap;

pub struct MermaidGenerator;

impl GraphGenerator for MermaidGenerator {
    fn generate(&self, project: &ProjectMap) -> Result<(), Error> {
        todo!()
    }
}
