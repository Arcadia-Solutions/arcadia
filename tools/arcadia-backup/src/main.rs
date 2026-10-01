mod arcadia_config;
mod commands;
mod components;
mod config;
mod ctx;
mod meta;
mod process;
mod random;
mod rest_server;
mod restic;
mod selector;
mod shell;
mod ssh;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Incremental, versioned backups of an Arcadia instance, driven from a backup host over ssh"
)]
struct Cli {
    /// Configuration file [default: ~/.config/arcadia-backup/arcadia-backup.yml]
    #[arg(long, global = true)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create the restic repository
    Init,
    /// Back up the Arcadia host, then apply the retention policy
    Backup,
    /// List the snapshots, the latest first
    Snapshots,
    /// Restore a snapshot on the Arcadia host, replacing its data
    Restore {
        /// `latest`, a number of backups before the latest (`1` is the one before it), or a snapshot id
        #[arg(long, default_value = "latest")]
        snapshot: selector::Selector,
        /// Only restore these components, comma separated
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
        /// Keep the current configuration files
        #[arg(long)]
        skip_config: bool,
        /// Do not ask for confirmation
        #[arg(long)]
        yes: bool,
    },
    /// Extract a snapshot, unencrypted, into a local directory
    Extract {
        /// `latest`, a number of backups before the latest (`1` is the one before it), or a snapshot id
        #[arg(long)]
        snapshot: selector::Selector,
        /// Empty or new directory to extract into
        #[arg(long)]
        target: PathBuf,
    },
    /// Apply the retention policy
    Prune,
    /// Check the integrity of the repository
    Check {
        /// Also read this subset of the data, e.g. `5%`
        #[arg(long)]
        read_data_subset: Option<String>,
    },
}

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let path = match cli.config {
        Some(path) => path,
        None => config::default_path()?,
    };
    let config = config::load(&path)?;
    match cli.command {
        Command::Init => commands::init::run(&config),
        Command::Backup => commands::backup::run(&config),
        Command::Snapshots => commands::snapshots::run(&config),
        Command::Restore {
            snapshot,
            only,
            skip_config,
            yes,
        } => commands::restore::run(
            &config,
            commands::restore::Options {
                snapshot,
                only,
                skip_config,
                yes,
            },
        ),
        Command::Extract { snapshot, target } => {
            commands::extract::run(&config, &snapshot, &target)
        }
        Command::Prune => commands::prune::run(&config),
        Command::Check { read_data_subset } => {
            commands::check::run(&config, read_data_subset.as_deref())
        }
    }
}
