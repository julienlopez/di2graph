mod boost_di_file_parser;
mod di_parser;
mod error;

use clap::Parser;

use crate::boost_di_file_parser::BoostDiFileParser;
use crate::di_parser::DiParser;
use crate::error::Error;

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
