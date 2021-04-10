extern crate getopts;
extern crate pdfpackman;

use std::env;
use std::process;

use pdfpackman::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing aurguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = pdfpackman::run(config) {
        eprintln!("Problem execute: {}", e);
        process::exit(1);
    }
}
