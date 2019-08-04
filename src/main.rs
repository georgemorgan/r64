extern crate rustyline;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use rustyline::error::ReadlineError;
use rustyline::Editor;

/* Use the N64 module. */
mod n64;
use n64::N64;
use n64::N64_ROM_HEADER;
use n64::N64_ROM_HEADER_SIZE;

use std::str;

/* 'main()' function; loads N64 ROM and initializes emulator context. */
fn main() {
    /* Manage command line arguments using a vector. */
    let args: Vec<_> = env::args().collect();
    /* Ensure the proper arguments were provided, otherwise print usage information. */
    if args.len() < 3 {
        println!("r64: Another Nintendo 64 emulator; this time, written in Rust.\n\nOriginally authored by George Morgan. (george@george-morgan.com)\n\nusage: r64 [rom]");
        return;
    }

    /* Open the ROM file. */
    let path = Path::new(&args[1]);
    let mut rom_file = File::open(path).unwrap();

    /* Create the header. */
    let mut h_data = [0; N64_ROM_HEADER_SIZE];
    rom_file.read_exact(&mut h_data).unwrap();
    let h_data_p: *const u8 = h_data.as_ptr();
    let h_p: *const N64_ROM_HEADER = h_data_p as *const _;
    let header: &N64_ROM_HEADER = unsafe { &*h_p };

    /* Load the cartridge ROM. */
    let mut rom_buf = Vec::new();
    rom_file.read_to_end(&mut rom_buf).unwrap();
    let crom = rom_buf.into_boxed_slice();

    /* Open the PIF ROM file. */
    let path = Path::new(&args[2]);
    let mut pif_file = File::open(path).unwrap();

    /* Load the PIF ROM. */
    let mut pif_data_buf = Vec::new();
    pif_file.read_to_end(&mut pif_data_buf).unwrap();
    let prom = pif_data_buf.into_boxed_slice();

    /* Print the name of the loaded ROM. */
    let name = str::from_utf8(&header.name).unwrap().trim();
    println!("The ROM is {:?}.", name);

    /* Create the N64. */
    let mut n64 = N64::new(crom, prom);

    // loop {
    //     n64.cycle();
    // }

    let mut rl = Editor::<()>::new();
    'main_loop: loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                match line.as_ref() {
                    // /* Prints the CPU state. */
                    "print" | "p" => {
                        println!("{:?}", n64.cpu);
                    },
                    "quit" | "q" => {
                        break 'main_loop;
                    },
                    /* Steps into a single instruction. */
                    "step" | "s" | _ => {
                        n64.cycle();
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
