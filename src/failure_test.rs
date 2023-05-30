use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use thiserror::Error;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;
const PROJECT_NAME: &str = "my-project";

fn num_docs_created_in_last_minute() -> u8 {
    2
}

#[derive(Debug, Error)]
enum DocumentServiceError {
    #[error("You have exceeded the allowed number of documents per minute")]
    RateLimitExceeded,
    #[error("I/O error: {0}")]
    Io(io::Error),
}

// Implement From trait in order to be able to use "?"
impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

// Type alias to shorten "Result<T, DocumentServiceError>" to "Result<T>"
use std::result;

type Result<T> = result::Result<T, DocumentServiceError>;

fn create_document(filename: &str) -> Result<File> {
    if num_docs_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name))?;
    create_document(&format!("{}-draft2", project_name))?;
    create_document(&format!("{}-revision1", project_name))?;
    create_document(&format!("{}-revision2", project_name))?;

    Ok(())
}

pub fn main() {
    for suffix in [ "draft1", "draft2", "revision1", "revision2"].iter() {
        std::fs::remove_file(&format!("{}-{}", PROJECT_NAME, suffix)).ok();
    }

    println!("Simulating project creation…");
    match create_project(PROJECT_NAME) {
        Ok(()) => println!("Project created successfully!"),
        Err(e) => println!("Project creation failed: {}", e),
    }
    println!("Simulating second project creation…");
    match create_project(PROJECT_NAME) {
        Ok(()) => println!("Project created successfully!"),
        Err(e) => {
            println!("Project creation failed: {}", e);
            // FIXME: Backtrace available in "thiserror"?
            // if let Some(backtrace) = e.backtrace() {
            //     if !backtrace.to_string().trim().is_empty() {
            //         println!("Backtrace: {:?}", backtrace);
            //     }
            // } else {
            //     println!("IN ORDER TO SHOW BACKTRACE RUN WITH 'RUST_BACKTRACE=1'.");
            // }

        },
    }
}
