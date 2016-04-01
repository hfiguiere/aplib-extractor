
use std::path::PathBuf;

/// Return the testfile path for filename
/// Test files are in the testdata directory in the crate top level.
pub fn get_test_file_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(file!());
    // go up two directories
    path.pop();
    path.pop();
    path.push("testdata");
    path.push(filename);
    path
}

