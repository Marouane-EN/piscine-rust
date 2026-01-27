use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new().read(true).create(true).append(true).open(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_create_new_file() {
        let test_file = PathBuf::from("test_create.txt");
        // Clean up if file exists
        let _ = fs::remove_file(&test_file);
        
        open_or_create(&test_file, "Hello, World!");
        
        assert!(test_file.exists());
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "Hello, World!");
        
        // Clean up
        fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_append_to_existing_file() {
        let test_file = PathBuf::from("test_append.txt");
        // Clean up if file exists
        let _ = fs::remove_file(&test_file);
        
        open_or_create(&test_file, "First line\n");
        open_or_create(&test_file, "Second line\n");
        
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "First line\nSecond line\n");
        
        // Clean up
        fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_multiple_appends() {
        let test_file = PathBuf::from("test_multiple.txt");
        let _ = fs::remove_file(&test_file);
        
        open_or_create(&test_file, "A");
        open_or_create(&test_file, "B");
        open_or_create(&test_file, "C");
        
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "ABC");
        
        fs::remove_file(&test_file).unwrap();
    }
}
