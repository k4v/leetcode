#![allow(dead_code)]

pub mod args;

use args::args::LcArgs;

fn main() {
    let lc_args: LcArgs = argh::from_env();
    println!("{:?}", lc_args);
}
