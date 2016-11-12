use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

/* Use the N64 module. */
mod n64;
use n64::N64;

/* 'main()' function; loads N64 ROM and initializes emulator context. */
fn main() {
	/* Manage command line arguments using a vector. */
	let args: Vec<_> = env::args().collect();
	/* Ensure the proper arguments were provided, otherwise print usage information. */
	if args.len() < 2 {
		println!("r64: Another Nintendo 64 emulator; this time, written in Rust.\n\nOriginally authored by George Morgan. (george@george-morgan.com)\n\nusage: r64 [rom]");
		return;
	}
	/* Open the ROM file. */
	let rom = load_rom(&args[1]);
	/* Create the N64. */
	let n64 = N64::new(rom);
	/* Start emulation. */
	n64.begin();
}

fn load_rom(p: &String) -> Box<[u8]> {
	let path = Path::new(p);
	let mut file = File::open(path).unwrap();
	let mut file_buf = Vec::new();
	file.read_to_end(&mut file_buf).unwrap();
	file_buf.into_boxed_slice()
}
