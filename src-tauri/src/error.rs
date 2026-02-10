use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrcError {
    #[error("Not in a git repository. Please run this command from within a git repository.")]
    NotInGitRepo,

    #[error("Missing dependencies: {0}")]
    MissingDependencies(String),

    #[error("Task name is required. Usage: orcmate start <task-name>")]
    TaskNameRequired,

    #[error("Worktree '{0}' already exists at {1}")]
    WorktreeAlreadyExists(String, String),

    #[error("Tmux session '{0}' already exists. Use 'tmux attach -t {0}' or clean it first.")]
    TmuxSessionExists(String),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("Tmux error: {0}")]
    TmuxError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown command: {0}. Run 'orcmate help' for usage.")]
    UnknownCommand(String),
}

pub type Result<T> = std::result::Result<T, OrcError>;
