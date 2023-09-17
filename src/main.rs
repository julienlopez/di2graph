mod di_parsers;
mod error;
mod graph_generators;
mod project_map;
mod utils;

use clap::Parser;

use di_parsers::boost_di_file_parser::BoostDiFileParser;
use di_parsers::di_parser::DiParser;
use error::Error;
use graph_generators::mermaid_generator::MermaidGenerator;

use crate::graph_generators::graph_generator::GraphGenerator;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short, long)]
    main_dir: String,

    #[arg(short, long, default_value_t = String::from("./res.mmd"))]
    output_file: String,
    // additional_dir : Vec<Option<std::path::PathBuf>>,
}

fn main() -> Result<(), Error> {
    let args = CliArgs::parse();
    dbg!(&args);

    println!("Parsing {:?}", &args.main_dir);
    let parser = BoostDiFileParser::new(args.main_dir)?;
    let generator = MermaidGenerator::new(args.output_file)?;
    let project = parser.analyze_dir()?;
    generator.generate(&project)
}
