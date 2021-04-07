use std::path::PathBuf;
use structopt::clap;
pub use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(bin_name = "migra", name = "Migra")]
pub(crate) struct AppOpt {
    #[structopt(name = "config", short, long)]
    pub config_path: Option<PathBuf>,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt, Clone)]
pub(crate) enum Command {
    Init,

    Apply(ApplyCommandOpt),

    Make(MakeCommandOpt),

    #[structopt(name = "list", visible_alias = "ls")]
    List,

    #[structopt(name = "upgrade", visible_alias = "up")]
    Upgrade(UpgradeCommandOpt),

    #[structopt(name = "downgrade", visible_alias = "down")]
    Downgrade(DowngradeCommandOpt),

    Completions(CompletionsShell),
}

#[derive(Debug, StructOpt, Clone)]
pub(crate) struct ApplyCommandOpt {
    #[structopt(parse(from_str))]
    pub file_name: String,
}

#[derive(Debug, StructOpt, Clone)]
pub(crate) struct MakeCommandOpt {
    /// Name of the migration to create in specify directory.
    #[structopt(parse(from_str))]
    pub migration_name: String,
}

#[derive(Debug, StructOpt, Clone)]
pub(crate) struct UpgradeCommandOpt {
    /// Name of the existing migration that will update the schema
    /// in the database.
    #[structopt(long = "name")]
    pub migration_name: Option<String>,

    /// How many existing migrations do we have to update.
    #[structopt(long = "number", short = "n")]
    pub migrations_number: Option<usize>,
}

#[derive(Debug, StructOpt, Clone)]
pub(crate) struct DowngradeCommandOpt {
    /// How many applied migrations do we have to rollback.
    #[structopt(long = "number", short = "n", default_value = "1")]
    pub migrations_number: usize,

    /// Rolls back all applied migrations. Ignores --number option.
    #[structopt(long = "all")]
    pub all_migrations: bool,
}

#[derive(Debug, StructOpt, Clone)]
pub(crate) enum CompletionsShell {
    Bash,
    Fish,
    Zsh,
    PowerShell,
    Elvish,
}

impl From<CompletionsShell> for clap::Shell {
    fn from(shell: CompletionsShell) -> Self {
        match shell {
            CompletionsShell::Bash => Self::Bash,
            CompletionsShell::Fish => Self::Fish,
            CompletionsShell::Zsh => Self::Zsh,
            CompletionsShell::PowerShell => Self::PowerShell,
            CompletionsShell::Elvish => Self::Elvish,
        }
    }
}
