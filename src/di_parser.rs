use crate::error::Error;

pub trait DiParser {
    fn analyze_dir(&self) -> Result<(), Error>;
}
