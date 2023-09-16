use crate::error::Error;
use crate::graph_generators::graph_generator::GraphGenerator;
use crate::project_map::ProjectMap;

pub struct MermaidGenerator {
    output_file: String,
}

impl MermaidGenerator {
    pub fn new(output_file: String) -> Result<Self, Error> {
        Ok(MermaidGenerator { output_file })
    }
}

impl GraphGenerator for MermaidGenerator {
    fn generate(&self, project: &ProjectMap) -> Result<(), Error> {
        todo!()
    }
}
