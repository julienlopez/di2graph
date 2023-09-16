#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(std::io::Error),
}
