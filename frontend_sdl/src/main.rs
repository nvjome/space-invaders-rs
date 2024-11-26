use core_8080;
use std::{env, process};
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Welcome to Space Invaders!");

    // let path = "/home/nolanjome/Rust/space-invaders-rs/roms/invaders.h";

    let args: Vec<_> = env::args().collect();

    // Open ROM file
    let mut rom_file = match File::open(args[1].clone()) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            process::exit(1);
        },
    };

    let mut rom_buffer = Vec::new();
    if let Err(e) = rom_file.read_to_end(&mut rom_buffer) {
        eprintln!("Error reading ROM: {}", e);
        process::exit(1);
    }

    let mut cpu = core_8080::CPU::new();
    cpu.load_rom(&rom_buffer).unwrap();

    for _i in 0..10000 {
        let _ = cpu.tick().unwrap();
    }
}
