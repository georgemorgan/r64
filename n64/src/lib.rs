mod vr4300;
use vr4300::*;

mod rcp;
use rcp::*;

mod pif;
use pif::*;

mod mc;
use mc::*;

pub const N64_ROM_HEADER_SIZE: usize = 0x40;

/* N64 memory sizes. */
pub const N64_IRAM_SIZE: usize = 0x400000;

pub struct N64 {
    cart: Box<[u8]>,
    rdram: Box<[u8]>,
    pub cpu: VR4300,
    rcp: RCP,
    pif: PIF
}

impl N64 {
    pub fn new(cart: Box<[u8]>, pifrom: Box<[u8]>) -> N64 {
        N64 {
            cart: cart,
            rdram: Box::new([0; N64_IRAM_SIZE]),
            cpu: VR4300::new((PIF_ROM_START | KSEG0_START) as u64),
            rcp: RCP::new(),
            pif: PIF::new(pifrom)
        }
    }

    pub fn step(&mut self) {
        let cart = &self.cart;
        let rdram = &self.rdram;
        let rcp = &self.rcp;
        let pif = &self.pif;

        let l = |addr| {
            mc::read(addr, cart, rdram, rcp, pif)
        };

        self.cpu.ic(&l);
    }
}

pub trait Read {
    fn read(&self, addr: u32) -> u32;
}

pub trait Write {
    fn write(&mut self, addr: u32, val: u32);
}

impl Read for N64 {
    fn read(&self, addr: u32) -> u32 {
        mc::read(addr, &self.cart, &self.rdram, &self.rcp, &self.pif)
    }
}

impl Write for N64 {
    fn write(&mut self, addr:u32, val: u32) {
        mc::write(addr, val, &mut self.rdram, &mut self.rcp, &mut self.pif);
    }
}
