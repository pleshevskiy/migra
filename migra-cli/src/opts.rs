pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub(crate) enum AppOpt {
    Init,
}
