#![deny(clippy::all)]

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum AppOpt {
    Init,
}


fn main() {
    let opt = AppOpt::from_args();
    dbg!(&opt);

    match opt {
        AppOpt::Init => {
            println!("unimplemented");
        }
    }
}
