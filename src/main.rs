use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

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
	if args.len() < 2 {
		println!("r64: Another Nintendo 64 emulator; this time, written in Rust.\n\nOriginally authored by George Morgan. (george@george-morgan.com)\n\nusage: r64 [rom]");
		return;
	}

	/* Open the file. */
	let path = Path::new(&args[1]);
	let mut file = File::open(path).unwrap();

	/* Create the header. */
	let mut h_data = [0; N64_ROM_HEADER_SIZE];
	file.read_exact(&mut h_data);
	let h_data_p: *const u8 = h_data.as_ptr();
	let h_p: *const N64_ROM_HEADER = h_data_p as *const _;
	let header: &N64_ROM_HEADER = unsafe { &*h_p };

	/* Load the cartridge ROM. */
	let mut file_buf = Vec::new();
	file.read_to_end(&mut file_buf).unwrap();
	let cr = file_buf.into_boxed_slice();

	/* Print the name of the loaded ROM. */
	let name = str::from_utf8(&header.name).unwrap().trim();
	println!("The ROM is {:?}.", name);

	/* Create the N64. */
	let mut n64 = N64::new(cr, &header);
	/* Start emulation. */
	n64.begin();
}
