pub mod e2e;
pub mod integration;
pub mod unit;

use architecture::chapter2;

#[test]
fn test_sync() {
    let source = "tests/source";
    let target = "tests/target";

    // Prepare test directories
    std::fs::create_dir_all(source).unwrap();
    std::fs::create_dir_all(target).unwrap();

    // Create test files in source
    std::fs::write(format!("{}/file1.txt", source), "Hello World").unwrap();
    std::fs::write(format!("{}/file2.txt", source), "Rust Programming").unwrap();

    // Create test files in target
    std::fs::write(format!("{}/file1.txt", target), "Old Content").unwrap();
    std::fs::write(format!("{}/file3.txt", target), "Rust Programming").unwrap();

    // Perform sync
    chapter2::sync(source, target);

    // Verify results
    assert!(std::path::Path::new(&format!("{}/file1.txt", target)).exists());
    assert!(std::path::Path::new(&format!("{}/file2.txt", target)).exists());
    assert!(!std::path::Path::new(&format!("{}/file3.txt", target)).exists());

    // Clean up test directories
    std::fs::remove_dir_all(source).unwrap();
    std::fs::remove_dir_all(target).unwrap();
}
