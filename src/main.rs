#![allow(dead_code, unused_variables)]

pub mod common;
pub mod problems;

use std::path::Path;

use common::{args::LcArgs, solutions::Solution};

fn main() {
    let lc_args: LcArgs = argh::from_env();
    println!("{:?}", lc_args);

    match lc_args.problem {
        1 => {
            let results = problems::p1::p1::P1::new(Path::new(&lc_args.input_file)).solve();
            println!("{results:?}");
        },
        56 => {
            let results = problems::p56::p56::P56::new(Path::new(&lc_args.input_file)).solve();
            println!("{results:?}");
        },
        2685 => {
            let results = problems::p2685::p2685::P2685::new(Path::new(&lc_args.input_file)).solve();
            println!("{results:?}");
        },
        _ => {
            println!("Unrecognized problem ID")
        }
    }    
}
