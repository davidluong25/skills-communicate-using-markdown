/// Git worktree management for OrcMate.
use std::path::Path;
use std::process::Command;

use crate::error::{OrcError, Result};
use crate::config::Config;

/// Check if the current directory is inside a git repository.
pub fn check_git_repo() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()?;

    if !output.status.success() {
        return Err(OrcError::NotInGitRepo);
    }
    Ok(())
}

/// Get the root directory of the current git repository.
pub fn get_repo_root() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        return Err(OrcError::NotInGitRepo);
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Check if a git branch exists.
pub fn branch_exists(branch_name: &str, repo_root: &str) -> Result<bool> {
    let output = Command::new("git")
        .args(["show-ref", "--verify", "--quiet", &format!("refs/heads/{}", branch_name)])
        .current_dir(repo_root)
        .output()?;

    Ok(output.status.success())
}

/// Create a git worktree with a new branch.
pub fn create_worktree(config: &Config, repo_root: &str, task_name: &str) -> Result<()> {
    let worktree_path = config.worktree_path(repo_root, task_name);
    let branch_name = config.branch_name(task_name);

    // Check if worktree directory already exists
    if Path::new(&worktree_path).exists() {
        return Err(OrcError::WorktreeAlreadyExists(
            task_name.to_string(),
            worktree_path,
        ));
    }

    // Create worktree directory parent
    let worktree_parent = format!("{}/{}", repo_root, config.worktree_dir);
    std::fs::create_dir_all(&worktree_parent)?;

    // Try creating worktree with a new branch
    let output = Command::new("git")
        .args(["worktree", "add", "-b", &branch_name, &worktree_path])
        .current_dir(repo_root)
        .output()?;

    if !output.status.success() {
        // If branch already exists, try checking it out
        if branch_exists(&branch_name, repo_root)? {
            let output = Command::new("git")
                .args(["worktree", "add", &worktree_path, &branch_name])
                .current_dir(repo_root)
                .output()?;

            if !output.status.success() {
                return Err(OrcError::GitError(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ));
            }
        } else {
            return Err(OrcError::GitError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
    }

    Ok(())
}

/// Remove a git worktree.
pub fn remove_worktree(config: &Config, repo_root: &str, task_name: &str) -> Result<bool> {
    let worktree_path = config.worktree_path(repo_root, task_name);

    if !Path::new(&worktree_path).exists() {
        return Ok(false);
    }

    let output = Command::new("git")
        .args(["worktree", "remove", &worktree_path, "--force"])
        .current_dir(repo_root)
        .output()?;

    if !output.status.success() {
        return Err(OrcError::GitError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(true)
}

/// Delete a git branch.
pub fn delete_branch(branch_name: &str, repo_root: &str) -> Result<bool> {
    if !branch_exists(branch_name, repo_root)? {
        return Ok(false);
    }

    let output = Command::new("git")
        .args(["branch", "-D", branch_name])
        .current_dir(repo_root)
        .output()?;

    if !output.status.success() {
        return Err(OrcError::GitError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(true)
}

/// List all git worktrees, optionally filtering by worktree directory.
pub fn list_worktrees(config: &Config, repo_root: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["worktree", "list"])
        .current_dir(repo_root)
        .output()?;

    if !output.status.success() {
        return Err(OrcError::GitError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let worktrees: Vec<String> = stdout
        .lines()
        .filter(|line| line.contains(&config.worktree_dir))
        .map(|line| line.to_string())
        .collect();

    Ok(worktrees)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_git_repo() -> TempDir {
        let dir = TempDir::new().unwrap();
        Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        // Create initial commit so branches work
        std::fs::write(dir.path().join("README.md"), "# Test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        dir
    }

    #[test]
    fn test_create_worktree() {
        let dir = setup_git_repo();
        let config = Config::new();
        let repo_root = dir.path().to_str().unwrap();

        let result = create_worktree(&config, repo_root, "test-task");
        assert!(result.is_ok(), "Failed to create worktree: {:?}", result);

        let worktree_path = config.worktree_path(repo_root, "test-task");
        assert!(Path::new(&worktree_path).exists());
    }

    #[test]
    fn test_create_worktree_already_exists() {
        let dir = setup_git_repo();
        let config = Config::new();
        let repo_root = dir.path().to_str().unwrap();

        create_worktree(&config, repo_root, "dup-task").unwrap();
        let result = create_worktree(&config, repo_root, "dup-task");
        assert!(result.is_err());

        match result.unwrap_err() {
            OrcError::WorktreeAlreadyExists(name, _) => {
                assert_eq!(name, "dup-task");
            }
            other => panic!("Expected WorktreeAlreadyExists, got: {:?}", other),
        }
    }

    #[test]
    fn test_remove_worktree() {
        let dir = setup_git_repo();
        let config = Config::new();
        let repo_root = dir.path().to_str().unwrap();

        create_worktree(&config, repo_root, "remove-task").unwrap();
        let result = remove_worktree(&config, repo_root, "remove-task");
        assert!(result.is_ok());
        assert!(result.unwrap());

        let worktree_path = config.worktree_path(repo_root, "remove-task");
        assert!(!Path::new(&worktree_path).exists());
    }

    #[test]
    fn test_remove_nonexistent_worktree() {
        let dir = setup_git_repo();
        let config = Config::new();
        let repo_root = dir.path().to_str().unwrap();

        let result = remove_worktree(&config, repo_root, "no-such-task");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_branch_exists_after_create() {
        let dir = setup_git_repo();
        let config = Config::new();
        let repo_root = dir.path().to_str().unwrap();

        create_worktree(&config, repo_root, "branch-test").unwrap();
        let exists = branch_exists(&config.branch_name("branch-test"), repo_root).unwrap();
        assert!(exists);
    }

    #[test]
    fn test_delete_branch() {
        let dir = setup_git_repo();
        let config = Config::new();
        let repo_root = dir.path().to_str().unwrap();

        create_worktree(&config, repo_root, "del-branch").unwrap();
        // Remove worktree first, then delete branch
        remove_worktree(&config, repo_root, "del-branch").unwrap();
        let result = delete_branch(&config.branch_name("del-branch"), repo_root);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_delete_nonexistent_branch() {
        let dir = setup_git_repo();
        let repo_root = dir.path().to_str().unwrap();

        let result = delete_branch("agent/no-such-branch", repo_root);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_list_worktrees() {
        let dir = setup_git_repo();
        let config = Config::new();
        let repo_root = dir.path().to_str().unwrap();

        create_worktree(&config, repo_root, "list-task-1").unwrap();
        create_worktree(&config, repo_root, "list-task-2").unwrap();

        let worktrees = list_worktrees(&config, repo_root).unwrap();
        assert_eq!(worktrees.len(), 2);
        assert!(worktrees.iter().any(|w| w.contains("list-task-1")));
        assert!(worktrees.iter().any(|w| w.contains("list-task-2")));
    }
}
