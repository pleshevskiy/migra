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

    #[structopt(name = "list", visible_alias = "ls")]
    List,
}

#[derive(Debug, StructOpt)]
pub(crate) struct ApplyCommandOpt {
    #[structopt(parse(from_str))]
    pub file_name: String,
}
