use crate::di_parsers::di_parser::DiParser;
use crate::error::Error;

use walkdir::WalkDir;

pub struct BoostDiFileParser {
    main_dir: String,
}

impl BoostDiFileParser {
    pub fn new(main_dir: String) -> Result<BoostDiFileParser, Error> {
        if !std::path::Path::new(&main_dir).is_dir() {
            panic!("main dir is not valid.");
        }
        Ok(BoostDiFileParser { main_dir })
    }
}

impl DiParser for BoostDiFileParser {
    fn analyze_dir(&self) -> Result<(), Error> {
        for entry in WalkDir::new(&self.main_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();

            if f_name.ends_with(".cpp") {
                println!("{}", f_name);
            }
        }
        Ok(())
    }
}
