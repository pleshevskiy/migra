pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub(crate) enum AppOpt {
    Init,
    Apply(ApplyOpt),
}


#[derive(Debug, StructOpt)]
pub(crate) struct ApplyOpt {
    #[structopt(parse(from_str))]
    pub file_name: String
}
