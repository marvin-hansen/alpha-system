use std::fs::File;
use std::io::Read;

/// FileManager is the main struct for the file manager.
/// It offers a simple interface to the file system.
/// THe main functionality is to read and write files.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileManager{

}

impl FileManager{
    pub fn new() -> Self {
        Self {}
    }
}

impl FileManager {

/// Reads the contents of the file at the given path as a String.
///
/// # Parameters
///
/// * `path` - The path to the file to read as a &str
///
/// # Returns
///
/// Returns a Result with the file contents as a String if successful, or an io::Error if there was a problem reading the file.
///
/// # Example
///
/// ```
/// let file_manager = file_manager::FileManager::new();
/// let contents = file_manager.read_file("data.txt")?;
/// ```
 fn read_file(path: &str) -> Result<String, std::io::Error> {
   let mut file = File::open(path)?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;
   Ok(contents)
 }
}
