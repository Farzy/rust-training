use std::fs::File;
use std::fs::OpenOptions;
use std::io;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_docs_created_in_last_minute() -> u8 {
    2
}

quick_error! {
    #[derive(Debug)]
    enum DocumentServiceError {
        RateLimitExceeded {
            display("You have exceeded the allowed number of documents per minute.")
        }
        Io(cause: io::Error) {
            display("I/O error: {}", cause)
            context(filename: &'a str, cause: io::Error)
                -> (filename.to_string(), cause)
        }
    }
}

// Type alias to shorten "Result<T, DocumentServiceError>" to "Result<T>"
use std::result;
use quick_error::ResultExt;

type Result<T> = result::Result<T, DocumentServiceError>;

fn create_document(filename: &str) -> Result<File> {
    if num_docs_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .context(filename)?;

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
        std::fs::remove_file(&format!("my-project-{}", suffix)).ok();
    }

    println!("Simulating project creation…");
    match create_project("my-project") {
        Ok(()) => println!("Project created successfully!"),
        Err(e) => println!("Project creation failed: {}", e),
    }
    println!("Simulating second project creation…");
    match create_project("my-project") {
        Ok(()) => println!("Project created successfully!"),
        Err(e) => println!("Project creation failed: {}", e),
    }
}
