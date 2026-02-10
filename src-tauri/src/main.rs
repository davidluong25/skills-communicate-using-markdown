use clap::{Parser, Subcommand};
use colored::*;

use orcmate_core::config::Config;
use orcmate_core::{git, tmux, env_file};
use orcmate_core::error::OrcError;

#[derive(Parser)]
#[command(
    name = "orcmate",
    version = orcmate_core::config::VERSION,
    about = "OrcMate - Keyboard-First AI Terminal Environment Manager",
    long_about = "A lightweight orchestrator that creates isolated, persistent, and keyboard-first\n\
                  development environments for AI-assisted coding.\n\
                  Built on top of Git Worktrees and Tmux."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new isolated environment (worktree + tmux)
    Start {
        /// Name of the task to start
        task_name: String,
    },
    /// Remove an environment and all its resources
    Clean {
        /// Name of the task to clean up
        task_name: String,
    },
    /// List all active environments
    #[command(alias = "ls")]
    List,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new();

    let result = match cli.command {
        Commands::Start { task_name } => cmd_start(&config, &task_name),
        Commands::Clean { task_name } => cmd_clean(&config, &task_name),
        Commands::List => cmd_list(&config),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "ERROR:".red(), e);
        std::process::exit(1);
    }
}

fn cmd_start(config: &Config, task_name: &str) -> Result<(), OrcError> {
    git::check_git_repo()?;
    check_dependencies()?;

    let repo_root = git::get_repo_root()?;
    let worktree_path = config.worktree_path(&repo_root, task_name);

    println!(
        "{} Creating OrcMate environment for task: {}",
        "→".blue(),
        task_name
    );

    // Create git worktree
    println!("{} Creating git worktree at {}", "→".blue(), worktree_path);
    git::create_worktree(config, &repo_root, task_name)?;
    println!(
        "{} Git worktree created on branch: {}",
        "✓".green(),
        config.branch_name(task_name)
    );

    // Copy .env if it exists
    if env_file::copy_env_file(&repo_root, &worktree_path)? {
        println!("{} .env file copied", "✓".green());
    }

    // Create tmux session
    println!("{} Creating tmux session: {}", "→".blue(), task_name);
    tmux::create_session(task_name, &worktree_path)?;
    println!("{} Tmux session created with 2 panes", "✓".green());
    println!("{} Environment ready! Attaching to session...", "✓".green());

    println!();
    println!("{} Workspace: {}", "→".blue(), worktree_path);
    println!("{} Branch: {}", "→".blue(), config.branch_name(task_name));
    println!();

    // Attach to session
    tmux::attach_session(task_name)?;

    Ok(())
}

fn cmd_clean(config: &Config, task_name: &str) -> Result<(), OrcError> {
    git::check_git_repo()?;

    let repo_root = git::get_repo_root()?;

    println!(
        "{} Cleaning up OrcMate environment: {}",
        "→".blue(),
        task_name
    );

    // Kill tmux session
    if tmux::kill_session(task_name)? {
        println!("{} Tmux session killed", "✓".green());
    } else {
        println!(
            "{} Tmux session '{}' not found",
            "!".yellow(),
            task_name
        );
    }

    // Remove worktree
    if git::remove_worktree(config, &repo_root, task_name)? {
        println!("{} Git worktree removed", "✓".green());
    } else {
        println!(
            "{} Worktree '{}' not found",
            "!".yellow(),
            task_name
        );
    }

    // Delete branch
    let branch_name = config.branch_name(task_name);
    if git::delete_branch(&branch_name, &repo_root)? {
        println!("{} Branch deleted", "✓".green());
    } else {
        println!(
            "{} Branch '{}' not found",
            "!".yellow(),
            branch_name
        );
    }

    println!("{} Environment cleaned up successfully!", "✓".green());

    Ok(())
}

fn cmd_list(config: &Config) -> Result<(), OrcError> {
    git::check_git_repo()?;

    let repo_root = git::get_repo_root()?;

    println!();
    println!("{}", "═══════════════════════════════════════".blue());
    println!("{}", "   OrcMate Active Environments".blue());
    println!("{}", "═══════════════════════════════════════".blue());
    println!();

    // List git worktrees
    println!("{}", "Git Worktrees:".green());
    let worktrees = git::list_worktrees(config, &repo_root)?;
    if worktrees.is_empty() {
        println!("  (none)");
    } else {
        for wt in &worktrees {
            println!("  {}", wt);
        }
    }

    println!();

    // List tmux sessions
    println!("{}", "Tmux Sessions:".green());
    let sessions = tmux::list_sessions()?;
    if sessions.is_empty() {
        println!("  (none)");
    } else {
        for session in &sessions {
            println!("  {}", session);
        }
    }

    println!();
    println!("{}", "═══════════════════════════════════════".blue());
    println!();

    Ok(())
}

fn check_dependencies() -> Result<(), OrcError> {
    let mut missing = Vec::new();

    if !is_command_available("tmux") {
        missing.push("tmux");
    }
    if !is_command_available("git") {
        missing.push("git");
    }

    if !missing.is_empty() {
        return Err(OrcError::MissingDependencies(missing.join(", ")));
    }

    Ok(())
}

fn is_command_available(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
