use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "migra", name = "Migra")]
pub(crate) struct AppOpt {
    #[structopt(short, long)]
    pub config: Option<PathBuf>,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub(crate) enum Command {
    Init,

    Apply(ApplyCommandOpt),

    Make(MakeCommandOpt),

    #[structopt(name = "list", visible_alias = "ls")]
    List,

    #[structopt(name = "upgrade", visible_alias = "up")]
    Upgrade,

    #[structopt(name = "downgrade", visible_alias = "down")]
    Downgrade,
}

#[derive(Debug, StructOpt)]
pub(crate) struct ApplyCommandOpt {
    #[structopt(parse(from_str))]
    pub file_name: String,
}

#[derive(Debug, StructOpt)]
pub(crate) struct MakeCommandOpt {
    #[structopt(parse(from_str))]
    pub migration_name: String,
}
