use crate::di_parsers::di_parser::DiParser;
use crate::error::Error;
use crate::project_map::ProjectMap;
use crate::utils::substring::SubString;

use walkdir::WalkDir;

use regex::Regex;

use std::fs;

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

    fn process_file(&self, file_content: String) -> Result<Vec<String>, Error> {
        let calls = extract_di_calls_from_file(file_content)?;
        let res = calls.into_iter().map(cleanup_whitespaces);
        Ok(res.collect())
    }
}

impl DiParser for BoostDiFileParser {
    fn analyze_dir(&self) -> Result<ProjectMap, Error> {
        for entry in WalkDir::new(&self.main_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();

            if f_name.ends_with(".cpp") {
                println!("{}", f_name);
                let res = self.process_file(fs::read_to_string(&entry.path())?)?;
                res.iter()
                    .filter(|c| !is_tying_a_value(c))
                    .filter(|c| !c.contains("#"))
                    .for_each(|c| {
                        println!("-> {}", c);
                    });
            }
        }
        Ok(ProjectMap::new())
    }
}

fn is_tying_a_value(line: &str) -> bool {
    let re1 = Regex::new(r".to(<[[:alnum:]]*>)*\(.+\)").unwrap();
    let re2 = Regex::new(r".to(<[[:alnum:]]*>)*\(\)").unwrap();
    re1.is_match(line) && !re2.is_match(line)
}

fn is_di_extract_complete(line: &str) -> bool {
    let nb_open_par = line.chars().filter(|c| *c == '(').count();
    let nb_closed_par = line.chars().filter(|c| *c == ')').count();
    nb_open_par <= nb_closed_par
}

fn extract_di_calls_from_file(file_content: String) -> Result<Vec<String>, Error> {
    let re = Regex::new(r"boost::di::bind<.+>[^,]*\)(,|\);)").unwrap();
    let matches = re
        .find_iter(&file_content)
        .map(|mat| -> Result<String, Error> {
            let mut substr = SubString::new(&file_content, mat.start(), mat.end())?;
            assert!(substr.to_str() == mat.as_str());
            while !is_di_extract_complete(substr.to_str()) {
                let next_pos = &file_content[substr.get_end()..]
                    .find(')')
                    .ok_or(Error::Message("Unable to find next position".to_string()))?;
                substr.increase_end(next_pos + 1)?;
            }
            Ok(substr.to_str().to_string())
        });
    matches.collect()
}

fn cleanup_whitespaces(s: String) -> String {
    if s.contains('#') {
        return s;
    }
    s.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .replace('\n', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_di_extract_complete() {
        assert!(is_di_extract_complete("boost::di::bind<IA>.to<A>(),"));
        assert!(is_di_extract_complete("boost::di::bind<IA>.to<A>());"));
        assert!(is_di_extract_complete("boost::di::bind<N::IA>.to<N::A>(),"));
        assert!(is_di_extract_complete(
            "boost::di::bind<N::IA>.to<N::A>());"
        ));
        assert!(is_di_extract_complete("boost::di::bind<IA>.to(my_a_var),"));
        assert!(is_di_extract_complete("boost::di::bind<IA>.to(my_a_var));"));
        assert!(is_di_extract_complete("boost::di::bind<IA>.to(fct(b)),"));
        assert!(is_di_extract_complete("boost::di::bind<IA>.to(fct(b)));"));
        assert!(is_di_extract_complete(
            "boost::di::bind<IA>.to<A>().in(boost::di::singleton),"
        ));
        assert!(is_di_extract_complete(
            "boost::di::bind<IA>.to<A>().in(boost::di::singleton));"
        ));
        assert!(is_di_extract_complete("boost::di::bind<IA>.to(fct(b, c)),"));
        assert!(is_di_extract_complete(
            "boost::di::bind<IA>.to(fct(b, c)));"
        ));
        assert!(is_di_extract_complete("boost::di::bind<IA>.to(A{b, c}),"));
        assert!(is_di_extract_complete("boost::di::bind<IA>.to(A{b, c}));"));
        assert!(is_di_extract_complete(
            "boost::di::bind<IA>.to(A{fct(b), c}),"
        ));
        assert!(is_di_extract_complete(
            "boost::di::bind<IA>.to(A{fct(b), c}));"
        ));

        assert!(!is_di_extract_complete("boost::di::bind<IA>.to(A{fct(b),"));
    }

    #[test]
    fn test_is_tying_a_value() {
        assert!(!is_tying_a_value("boost::di::bind<IA>.to<A>();"));
        assert!(!is_tying_a_value("boost::di::bind<IA>.to<A>(),"));
        assert!(!is_tying_a_value("boost::di::bind<N::IA>.to<N::A>();"));
        assert!(is_tying_a_value("boost::di::bind<IA>.to(my_a_var));"));
        assert!(is_tying_a_value("boost::di::bind<IA>.to(fct(b)));"));
        assert!(!is_tying_a_value(
            "boost::di::bind<IA>.to<A>().in(boost::di::singleton),"
        ));
        assert!(is_tying_a_value("boost::di::bind<IA>.to(fct(b, c)));"));
        assert!(is_tying_a_value("boost::di::bind<IA>.to(A{b, c}));"));
        assert!(is_tying_a_value("boost::di::bind<IA>.to(A{fct(b), c}));"));
        assert!(is_tying_a_value("boost::di::bind<IA>.to(a.b));"));
        // assert!(is_tying_a_value(
        //     "boost::di::bind<IA>.to(A{a,
        //     b})"
        // ));
        assert!(is_tying_a_value("boost::di::bind<IA>.to(A{a, b})"));
    }
}
