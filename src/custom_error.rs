use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::fmt;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 8;

fn num_docs_created_in_last_minute() -> u8 {
    2
}

#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

// Optional implementation of "description()"
impl Error for DocumentServiceError {
    fn description(&self) -> &str {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => "rate limit exceeded",
            Io(_) => "I/O error",
        }
    }
}

// Implement From trait in order to be able to use "?"
impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(
                f,
                "You have exceeded the allowed number of documents per minute."
            ),
            Io(ref io) => write!(f, "I/O error: {}", io),
        }
    }
}

pub fn create_document(filename: &str) -> Result<File, DocumentServiceError> {
    if num_docs_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

pub fn main() {
    std::fs::remove_file("custom_error.txt").ok(); // Don't mind failure if file does not exist

    println!("First time calling create_document()…"); // Should succeed
    if let Err(e) = create_document("custom_error.txt") {
        eprintln!("An error happened: {:?}", e);
    }
    println!("Second time calling create_document()…"); // Should fail
    if let Err(e) = create_document("custom_error.txt") {
        eprintln!("An error happened: {:?}", e);
    }
}
