extern crate entropy;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use entropy::calculate;

fn main() {
    if env::args().len() == 1 {
        println!("Please specify one or more file names.");
        return;
    }

    for i in 1..env::args().len() {
        let file_name = env::args().nth(i).unwrap();
        let mut buffer = Vec::new();
        let file = File::open(file_name.clone());
        match file {
            Ok(mut f) => {
                match f.read_to_end(&mut buffer) {
                    Ok(_) => {
                        println!("{}: {}", file_name.clone(), calculate(buffer));
                    },
                    Err(_) => {
                    }
                }
            },
            Err(e) => {
                println!("Can't open {}", e);
            }
        }
    }
}
