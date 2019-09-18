extern crate entropy_rs;

use entropy_rs::{Entropy, Shannon};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    if env::args().len() == 1 {
        println!("Please specify one or more file names.");
        return Ok(());
    }

    const BLOCK_SIZE: usize = 1024;
    env::args().skip(1).for_each(|ref file_name| {
        let mut entropy = Shannon::new();
        let _ = File::open(file_name).map(|mut file| {
            while {
                let mut buffer = vec![0; BLOCK_SIZE];
                file.read(&mut buffer)
                    .map(|len| {
                        entropy.input(&buffer[0..len]);

                        len == BLOCK_SIZE
                    })
                    .or_else(|_| Ok::<bool, std::io::Error>(false))
                    .unwrap()
            } {}
        });
        println!("{}: {}", file_name, entropy.calculate())
    });
    Ok(())
}
