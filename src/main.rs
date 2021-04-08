extern crate getopts;
extern crate rust_image2pdf;

use std::env;
use std::process;

use rust_image2pdf::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing aurguments: {}", err);
        process::exit(1);
    });


    if let Err(e) = rust_image2pdf::run(config) {
        eprintln!("Problem execute: {}", e);
        process::exit(1);
    }
}
