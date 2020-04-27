use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::fmt;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

#[derive(Debug)]
enum DocumentServiceError {
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

// Type alias to shorten "Result<T, DocumentServiceError>" to "Result<T>"
use std::result;
type Result<T> = result::Result<T, DocumentServiceError>;

fn create_document(filename: &str, num_docs_created_in_last_minute: u8) -> Result<File> {
    if num_docs_created_in_last_minute > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name), 2)?;
    create_document(&format!("{}-draft2", project_name), 2)?;
    create_document(&format!("{}-revision1", project_name), 2)?;
    create_document(&format!("{}-revision2", project_name), 2)?;

    Ok(())
}

pub fn main() {
    std::fs::remove_file("custom_error.txt").ok(); // Don't mind failure if file does not exist

    println!("Calling create_document() while exceeding rate limit…");
    if let Err(e) = create_document("custom_error.txt", 200) {
        eprintln!("An error happened!");
        eprintln!("- Short version: {}", e);
        eprintln!("- Long version:  {:?}", e);
    } else {
        eprintln!("Succeeded!");
    }

    std::fs::remove_file("custom_error.txt").ok(); // Don't mind failure if file does not exist

    println!("Calling create_document() after erasing existing file…"); // Should succeed
    if let Err(e) = create_document("custom_error.txt", 2) {
        eprintln!("An error happened: {:?}", e);
    } else {
        eprintln!("Succeeded!");
    }
    println!("Second time calling create_document()…"); // Should fail
    if let Err(e) = create_document("custom_error.txt", 2) {
        eprintln!("An error happened!");
        eprintln!("- Short version: {}", e);
        eprintln!("- Long version:  {:?}", e);
    } else {
        eprintln!("Succeeded!");
    }

    println!("Simulating project creation…");
    match create_project("my-project") {
        Ok(()) => println!("Project created successfully!"),
        Err(e) => println!("Project creation failed: {}", e),
    }
}
