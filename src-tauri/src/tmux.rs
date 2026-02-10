/// Tmux session management for OrcMate.
use std::process::Command;

use crate::error::{OrcError, Result};

/// Check if tmux is available on the system.
pub fn check_tmux() -> Result<bool> {
    let output = Command::new("which")
        .arg("tmux")
        .output()?;
    Ok(output.status.success())
}

/// Check if a tmux session with the given name already exists.
pub fn session_exists(session_name: &str) -> Result<bool> {
    let output = Command::new("tmux")
        .args(["has-session", "-t", session_name])
        .output()?;

    Ok(output.status.success())
}

/// Create a new tmux session with split panes for OrcMate workflow.
pub fn create_session(task_name: &str, worktree_path: &str) -> Result<()> {
    if session_exists(task_name)? {
        return Err(OrcError::TmuxSessionExists(task_name.to_string()));
    }

    // Create new detached session
    let output = Command::new("tmux")
        .args([
            "new-session", "-d", "-s", task_name, "-c", worktree_path,
        ])
        .output()?;

    if !output.status.success() {
        return Err(OrcError::TmuxError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Split window horizontally (top 70%, bottom 30%)
    let output = Command::new("tmux")
        .args([
            "split-window", "-v", "-p", "30",
            "-t", &format!("{}:0", task_name),
            "-c", worktree_path,
        ])
        .output()?;

    if !output.status.success() {
        return Err(OrcError::TmuxError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Select the top pane
    let _ = Command::new("tmux")
        .args(["select-pane", "-t", &format!("{}:0.0", task_name)])
        .output()?;

    // Send message to top pane
    let claude_available = Command::new("which")
        .arg("claude")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let top_cmd = if claude_available {
        "claude"
    } else {
        "echo 'OrcMate Action Pane - Start your AI assistant here (e.g., claude)'"
    };

    let _ = Command::new("tmux")
        .args([
            "send-keys", "-t", &format!("{}:0.0", task_name),
            top_cmd, "C-m",
        ])
        .output()?;

    // Send message to bottom pane
    let _ = Command::new("tmux")
        .args([
            "send-keys", "-t", &format!("{}:0.1", task_name),
            "echo 'OrcMate Monitor Pane - Run tests, logs, and commands here'",
            "C-m",
        ])
        .output()?;

    Ok(())
}

/// Kill a tmux session.
pub fn kill_session(session_name: &str) -> Result<bool> {
    if !session_exists(session_name)? {
        return Ok(false);
    }

    let output = Command::new("tmux")
        .args(["kill-session", "-t", session_name])
        .output()?;

    if !output.status.success() {
        return Err(OrcError::TmuxError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(true)
}

/// List all active tmux sessions.
pub fn list_sessions() -> Result<Vec<String>> {
    let output = Command::new("tmux")
        .args(["list-sessions"])
        .output()?;

    if !output.status.success() {
        // No sessions exist (tmux returns error)
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let sessions: Vec<String> = stdout
        .lines()
        .map(|line| line.to_string())
        .collect();

    Ok(sessions)
}

/// Attach to an existing tmux session (interactive).
pub fn attach_session(session_name: &str) -> Result<()> {
    let status = Command::new("tmux")
        .args(["attach-session", "-t", session_name])
        .status()?;

    if !status.success() {
        return Err(OrcError::TmuxError(format!(
            "Failed to attach to session '{}'",
            session_name
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_tmux_availability() {
        // tmux may or may not be installed in test environment
        let result = check_tmux();
        assert!(result.is_ok());
    }

    #[test]
    fn test_session_exists_nonexistent() {
        // A random session name should not exist
        let result = session_exists("orcmate_test_nonexistent_session_12345");
        // This may fail if tmux server is not running, which is expected
        if let Ok(exists) = result {
            assert!(!exists);
        }
    }

    #[test]
    fn test_list_sessions_no_server() {
        // When no tmux server is running, list_sessions should return empty vec
        let result = list_sessions();
        assert!(result.is_ok());
    }
}
