#![deny(clippy::all)]

mod opts;

use opts::{AppOpt, StructOpt};

fn main() {
    let opt = AppOpt::from_args();
    dbg!(&opt);

    match opt {
        AppOpt::Init => {
            println!("unimplemented");
        }
    }
}
