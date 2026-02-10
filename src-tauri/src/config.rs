/// OrcMate configuration constants and settings.

pub const VERSION: &str = "0.2.0";
pub const WORKTREE_DIR: &str = ".worktrees";
pub const BRANCH_PREFIX: &str = "agent";

/// Runtime configuration for OrcMate.
#[derive(Debug, Clone)]
pub struct Config {
    pub version: String,
    pub worktree_dir: String,
    pub branch_prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            worktree_dir: WORKTREE_DIR.to_string(),
            branch_prefix: BRANCH_PREFIX.to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the branch name for a given task.
    pub fn branch_name(&self, task_name: &str) -> String {
        format!("{}/{}", self.branch_prefix, task_name)
    }

    /// Build the worktree path relative to a repo root.
    pub fn worktree_path(&self, repo_root: &str, task_name: &str) -> String {
        format!("{}/{}/{}", repo_root, self.worktree_dir, task_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.version, "0.2.0");
        assert_eq!(config.worktree_dir, ".worktrees");
        assert_eq!(config.branch_prefix, "agent");
    }

    #[test]
    fn test_branch_name() {
        let config = Config::new();
        assert_eq!(config.branch_name("fix-login-bug"), "agent/fix-login-bug");
        assert_eq!(config.branch_name("feature-auth"), "agent/feature-auth");
    }

    #[test]
    fn test_worktree_path() {
        let config = Config::new();
        assert_eq!(
            config.worktree_path("/home/user/project", "fix-login"),
            "/home/user/project/.worktrees/fix-login"
        );
    }

    #[test]
    fn test_config_new_equals_default() {
        let config1 = Config::new();
        let config2 = Config::default();
        assert_eq!(config1.version, config2.version);
        assert_eq!(config1.worktree_dir, config2.worktree_dir);
        assert_eq!(config1.branch_prefix, config2.branch_prefix);
    }

    #[test]
    fn test_constants() {
        assert_eq!(VERSION, "0.2.0");
        assert_eq!(WORKTREE_DIR, ".worktrees");
        assert_eq!(BRANCH_PREFIX, "agent");
    }
}
