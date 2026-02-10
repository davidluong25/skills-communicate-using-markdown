/// Environment file (.env) handling for OrcMate.
use std::fs;
use std::path::Path;

use crate::error::Result;

/// Copy the .env file from repo root to worktree if it exists.
/// Returns true if the file was copied, false if no .env was found.
pub fn copy_env_file(repo_root: &str, worktree_path: &str) -> Result<bool> {
    let source = Path::new(repo_root).join(".env");
    let destination = Path::new(worktree_path).join(".env");

    if source.exists() {
        fs::copy(&source, &destination)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_copy_env_file_exists() {
        let source_dir = TempDir::new().unwrap();
        let dest_dir = TempDir::new().unwrap();

        // Create .env in source
        let env_content = "DATABASE_URL=postgres://localhost/test\nSECRET_KEY=abc123\n";
        fs::write(source_dir.path().join(".env"), env_content).unwrap();

        let result = copy_env_file(
            source_dir.path().to_str().unwrap(),
            dest_dir.path().to_str().unwrap(),
        );
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Verify the file was copied
        let copied = fs::read_to_string(dest_dir.path().join(".env")).unwrap();
        assert_eq!(copied, env_content);
    }

    #[test]
    fn test_copy_env_file_not_exists() {
        let source_dir = TempDir::new().unwrap();
        let dest_dir = TempDir::new().unwrap();

        // No .env file in source
        let result = copy_env_file(
            source_dir.path().to_str().unwrap(),
            dest_dir.path().to_str().unwrap(),
        );
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Verify no file was created
        assert!(!dest_dir.path().join(".env").exists());
    }

    #[test]
    fn test_copy_env_preserves_content() {
        let source_dir = TempDir::new().unwrap();
        let dest_dir = TempDir::new().unwrap();

        let env_content = "# Comment\nKEY1=value1\nKEY2=value2\nKEY3=value with spaces\n";
        fs::write(source_dir.path().join(".env"), env_content).unwrap();

        copy_env_file(
            source_dir.path().to_str().unwrap(),
            dest_dir.path().to_str().unwrap(),
        )
        .unwrap();

        let copied = fs::read_to_string(dest_dir.path().join(".env")).unwrap();
        assert_eq!(copied, env_content);
    }
}
