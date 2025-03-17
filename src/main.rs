#![allow(dead_code, unused_variables)]

pub mod args;
pub mod problems;

use std::path::Path;

use args::args::LcArgs;

fn main() {
    let lc_args: LcArgs = argh::from_env();
    println!("{:?}", lc_args);

    let p56 = problems::p56::p56::P56::new(Path::new("./README.md"));
    p56.run();
}
