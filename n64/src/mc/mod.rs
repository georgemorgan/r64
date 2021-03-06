/*
    0x0000 0000 to 0x03EF FFFF RDRAM Memory
    0x03F0 0000 to 0x03FF FFFF RDRAM Registers
    0x0400 0000 to 0x040F FFFF SP Registers
    0x0410 0000 to 0x041F FFFF DP Command Registers
    0x0420 0000 to 0x042F FFFF DP Span Registers
    0x0430 0000 to 0x043F FFFF MIPS Interface (MI) Registers
    0x0440 0000 to 0x044F FFFF Video Interface (VI) Registers
    0x0450 0000 to 0x045F FFFF Audio Interface (AI) Registers
    0x0460 0000 to 0x046F FFFF Peripheral Interface (PI) Registers
    0x0470 0000 to 0x047F FFFF RDRAM Interface (RI) Registers
    0x0480 0000 to 0x048F FFFF Serial Interface (SI) Registers
    0x0490 0000 to 0x04FF FFFF Unused
    0x0500 0000 to 0x05FF FFFF Cartridge Domain 2 Address 1
    0x0600 0000 to 0x07FF FFFF Cartridge Domain 1 Address 1
    0x0800 0000 to 0x0FFF FFFF Cartridge Domain 2 Address 2
    0x1000 0000 to 0x1FBF FFFF Cartridge Domain 1 Address 2
    0x1FC0 0000 to 0x1FC0 07BF PIF Boot ROM
    0x1FC0 07C0 to 0x1FC0 07FF PIF RAM
    0x1FC0 0800 to 0x1FCF FFFF Reserved
    0x1FD0 0000 to 0x7FFF FFFF Cartridge Domain 1 Address 3
    0x8000 0000 to 0xFFFF FFFF External SysAD Device
*/

use crate::RCP;
use crate::PIF;

/* RDRAM memory. */
pub const RDRAM_MEM_START:        u32 = 0x0000_0000;
pub const RDRAM_MEM_END:        u32 = 0x003E_FFFF;

/* RDRAM registers. */
pub const RDRAM_REG_START:        u32 = 0x03F0_0000;
pub const RDRAM_REG_END:        u32 = 0x03FF_FFFF;

/* RSP memory. */
pub const SP_DMEM_START:        u32 = 0x0400_0000;
pub const SP_DMEM_END:            u32 = 0x0400_0FFF;
pub const SP_IMEM_START:        u32 = 0x0400_1000;
pub const SP_IMEM_END:            u32 = 0x0400_1FFF;

/* RSP registers. */
pub const SP_REG_START:            u32 = 0x0404_0000;
pub const SP_REG_END:            u32 = 0x0404_0018;

/* RDP registers. */
pub const RDP_CMD_START:        u32 = 0x0410_0000;
pub const RDP_CMD_END:            u32 = 0x041F_FFFF;
pub const RDP_SPAN_START:        u32 = 0x0420_0000;
pub const RDP_SPAN_END:            u32 = 0x042F_FFFF;

/* MI registers. */
pub const MI_REG_START:            u32 = 0x0430_0000;
pub const MI_REG_END:            u32 = 0x043F_FFFF;

/* VI registers. */
pub const VI_REG_START:            u32 = 0x0440_0000;
pub const VI_REG_END:            u32 = 0x044F_FFFF;

/* AI registers. */
pub const AI_REG_START:            u32 = 0x0450_0000;
pub const AI_REG_END:            u32 = 0x045F_FFFF;

/* PI registers. */
pub const PI_REG_START:            u32 = 0x0460_0000;
pub const PI_REG_END:            u32 = 0x046F_FFFF;

/* RI registers. */
pub const RI_REG_START:            u32 = 0x0470_0000;
pub const RI_REG_END:            u32 = 0x047F_FFFF;

/* SI registers. */
pub const SI_REG_START:            u32 = 0x0480_0000;
pub const SI_REG_END:            u32 = 0x048F_FFFF;

/* Unused memory segment    . */
pub const UNUSED_START:            u32 = 0x0490_0000;
pub const UNUSED_END:            u32 = 0x04FF_FFFF;

/* Cartridge memory. */
pub const CART_DOM2_A1_START:    u32 = 0x0500_0000;
pub const CART_DOM2_A1_END:        u32 = 0x05FF_FFFF;
pub const CART_DOM1_A1_START:    u32 = 0x0600_0000;
pub const CART_DOM1_A1_END:        u32 = 0x07FF_FFFF;
pub const CART_DOM2_A2_START:    u32 = 0x0800_0000;
pub const CART_DOM2_A2_END:        u32 = 0x0FFF_FFFF;
pub const CART_DOM1_A2_START:    u32 = 0x1000_0000;
pub const CART_DOM1_A2_END:        u32 = 0x1FBF_FFFF;
pub const CART_DOM1_A3_START:    u32 = 0x1FD0_0000;
pub const CART_DOM1_A3_END:        u32 = 0x7FFF_FFFF;

/* PIF memory. */
pub const PIF_ROM_START:        u32 = 0x1FC0_0000;
pub const PIF_ROM_END:            u32 = 0x1FC0_07BF;
pub const PIF_RAM_START:        u32 = 0x1FC0_07C0;
pub const PIF_RAM_END:            u32 = 0x1FC0_07FF;

/* Reserved memory space. */
pub const RESERVED_START:        u32 = 0x1FC0_0800;
pub const RESERVED_END:            u32 = 0x1FCF_FFFF;

/* External device memory .*/
pub const SYSAD_START:            u32 = 0x8000_0000;
pub const SYSAD_END:            u32 = 0xFFFF_FFFF;

/* Virtual memory spaces. */
pub const KSEG0_START:            u32 = 0x8000_0000;
pub const KSEG0_END:            u32 = 0x9FFF_FFFF;
pub const KSEG1_START:            u32 = 0xA000_0000;
pub const KSEG1_END:            u32 = 0xBFFF_FFFF;

/* Reads a 32-bit word from a boxed slice of u8s. */
fn rmem(addr: u32, mem: &[u8]) -> u32 {
    /* Obtain a slice starting at the read address. */
    let b: &[u8] = &mem[addr as usize .. addr as usize + 4];
    /* Extract each of the word's bytes and use them to create a u32. */
    let w = ((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | b[3] as u32;
    /* Byte swap and adjust the endianness of the read word. */
    u32::from_be(w.swap_bytes())
}

/* Writes a 32-bit word to a boxed slice of u8s. */
fn wmem(addr: u32, val: u32, mem: &mut [u8]) {
    /* Obtain a slice of bytes from the u32. */
    let from: &[u8] = &[(val >> 24) as u8, (val >> 16) as u8, (val >> 8) as u8, val as u8];
    /* Write the slice into memory. */
    mem[addr as usize .. addr as usize + 4].copy_from_slice(from)
}

/* Convers a virtual address to a physical address. */
fn vtop(vaddr: u32) -> u32 {
    match vaddr {
        KSEG0_START ..= KSEG0_END => {
            /* Direct mapped segment KSEG0. */
            vaddr - KSEG0_START
        }, KSEG1_START ..= KSEG1_END => {
            /* Direct mapped segment KSEG1. */
            vaddr - KSEG1_START
        }, _ => panic!("Unrecognized virtual address: address: {:#x}", vaddr)
    }
}

/* Reads a word from the memory map. */
pub fn read(addr: u32, cart: &[u8], rdram: &[u8], rcp: &RCP, pif: &PIF) -> u32 {

    /* Convert the virtual address to a physical address. */
    let paddr = vtop(addr);

    /* Match the memory address to a peripheral address range. */
    match paddr {
        RDRAM_MEM_START ..= RDRAM_MEM_END => {
            return rmem(paddr - RDRAM_MEM_START, rdram);
        }, RDRAM_REG_START ..= RDRAM_REG_END => {
            unimplemented!()
        }, SP_DMEM_START ..= SP_DMEM_END => {
            rmem(paddr - SP_DMEM_START, &rcp.rsp.dmem)
        }, SP_IMEM_START ..= SP_IMEM_END => {
            rmem(paddr - SP_IMEM_START, &rcp.rsp.imem)
        }, SP_REG_START ..= SP_REG_END => {
            rcp.rsp.rreg(paddr)
        }, RDP_CMD_START ..= RDP_CMD_END => {
            rcp.rdp.dpc_rreg(paddr)
        }, RDP_SPAN_START ..= RDP_SPAN_END => {
            rcp.rdp.dps_rreg(paddr)
        }, MI_REG_START ..= MI_REG_END => {
            rcp.mi.rreg(paddr)
        }, VI_REG_START ..= VI_REG_END => {
            rcp.vi.rreg(paddr)
        }, AI_REG_START ..= AI_REG_END => {
            rcp.ai.rreg(paddr)
        }, PI_REG_START ..= PI_REG_END => {
            rcp.pi.rreg(paddr)
        }, RI_REG_START ..= RI_REG_END => {
            rcp.ri.rreg(paddr)
        }, SI_REG_START ..= SI_REG_END => {
            pif.rreg(paddr)
        }, UNUSED_START ..= UNUSED_END => {
            panic!("Attempt to read from unused address space.")
        }, CART_DOM2_A1_START ..= CART_DOM2_A1_END => {
            rmem(paddr - CART_DOM2_A1_START, &cart)
        }, CART_DOM1_A1_START ..= CART_DOM1_A1_END => {
            rmem(paddr - CART_DOM1_A1_START, &cart)
        }, CART_DOM2_A2_START ..= CART_DOM2_A2_END => {
            rmem(paddr - CART_DOM2_A2_START, &cart)
        }, CART_DOM1_A2_START ..= CART_DOM1_A2_END => {
            rmem(paddr - CART_DOM1_A2_START, &cart)
        }, PIF_ROM_START ..= PIF_ROM_END => {
            rmem(paddr - PIF_ROM_START, &pif.prom)
        }, PIF_RAM_START ..= PIF_RAM_END => {
            rmem(paddr - PIF_RAM_START, &pif.pram)
        }, RESERVED_START ..= RESERVED_END => {
            panic!("Attempt to read from a reserved location address: {:#x}.", paddr)
        }, CART_DOM1_A3_START ..= CART_DOM1_A3_END => {
            rmem(paddr - CART_DOM1_A3_START, &cart)
        }, SYSAD_START ..= SYSAD_END => {
            unimplemented!()
        }, _ => panic!("Read from unrecognized physical address: {:#x}", paddr)
    }
}

/* Writes a word to the provided N64's memory map. */
pub fn write(addr: u32, value: u32, rdram: &mut [u8], rcp: &mut RCP, pif: &mut PIF) {

    /* Convert the virtual address to the physical address. */
    let paddr = vtop(addr);

    /* Match the memory address to a peripheral address range. */
    match paddr {
        RDRAM_MEM_START ..= RDRAM_MEM_END => {
            wmem(paddr - RDRAM_MEM_START, value, rdram)
        }, RDRAM_REG_START ..= RDRAM_REG_END => {
            unimplemented!()
        }, SP_DMEM_START ..= SP_DMEM_END => {
            wmem(paddr - SP_DMEM_START, value, &mut rcp.rsp.dmem)
        }, SP_IMEM_START ..= SP_IMEM_END => {
            wmem(paddr - SP_IMEM_START, value, &mut rcp.rsp.imem)
        }, SP_REG_START ..= SP_REG_END => {
            rcp.rsp.wreg(paddr, value)
        }, RDP_CMD_START ..= RDP_CMD_END => {
            rcp.rdp.dpc_wreg(paddr, value)
        }, RDP_SPAN_START ..= RDP_SPAN_END => {
            rcp.rdp.dps_wreg(paddr, value)
        }, MI_REG_START ..= MI_REG_END => {
            rcp.mi.wreg(paddr, value)
        }, VI_REG_START ..= VI_REG_END => {
            rcp.vi.wreg(paddr, value)
        }, AI_REG_START ..= AI_REG_END => {
            rcp.ai.wreg(paddr, value)
        }, PI_REG_START ..= PI_REG_END => {
            rcp.pi.wreg(paddr, value)
        }, RI_REG_START ..= RI_REG_END => {
            rcp.ri.wreg(paddr, value)
        }, SI_REG_START ..= SI_REG_END => {
            pif.wreg(paddr, value)
        }, UNUSED_START ..= UNUSED_END => {
            panic!("Attempt to write to unused address space.")
        }, CART_DOM2_A1_START ..= CART_DOM2_A1_END |
           CART_DOM1_A1_START ..= CART_DOM1_A1_END |
           CART_DOM2_A2_START ..= CART_DOM2_A2_END |
           CART_DOM1_A2_START ..= CART_DOM1_A2_END |
           CART_DOM1_A3_START ..= CART_DOM1_A3_END => {
            panic!("Attempt to write to read-only cartridge memory address: {:#x}.", paddr)
        }, PIF_ROM_START ..= PIF_ROM_END => {
            panic!("Attempt to write to a read-only PIF memory address: {:#x}.", paddr)
        }, PIF_RAM_START ..= PIF_RAM_END => {
            wmem(paddr - PIF_RAM_START, value, &mut pif.pram)
        }, RESERVED_START ..= RESERVED_END => {
            panic!("Attempt to write to a reserved location address: {:#x}.", paddr)
        }, SYSAD_START ..= SYSAD_END => {
            unimplemented!()
        }, _ => panic!("Write to unrecognized physical address: address: {:#x}", paddr)
    }
}
