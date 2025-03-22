#![allow(dead_code, unused_variables)]

pub mod common;
pub mod problems;

use std::path::Path;

use common::{args::LcArgs, solutions::Solution};

fn main() {
    let lc_args: LcArgs = argh::from_env();
    println!("{:?}", lc_args);

    let p56 = problems::p56::p56::P56::new(Path::new(&lc_args.input_file));
    match p56.solve() {
        Ok(outputs) => println!("Problem {} solved", lc_args.problem),
        Err(err) => println!("Error solving problem {}: {:?}", lc_args.problem, err)
    }
}
