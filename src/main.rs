mod di_parsers;
mod error;
mod project_map;

use clap::Parser;

use di_parsers::boost_di_file_parser::BoostDiFileParser;
use di_parsers::di_parser::DiParser;
use error::Error;

#[derive(Parser, Debug)]
struct CliArgs {
    main_dir: std::path::PathBuf,
    // additional_dir : Vec<Option<std::path::PathBuf>>,
}

fn main() -> Result<(), Error> {
    let args = CliArgs::parse();
    dbg!(&args);

    println!("Parsing {:?}", &args.main_dir);
    let parser =
        BoostDiFileParser::new("D:\\prog\\stilla\\NioReader\\NioReaderServer".to_string())?;
    parser.analyze_dir()
}
