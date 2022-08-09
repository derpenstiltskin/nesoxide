#![allow(dead_code)]
use nesoxide_core::*;

fn main() {
    let nes_rom = Rom::load(".\\bin\\ntf2.nes").unwrap();
    let nes_rom_data = nes_rom.get_data();
    println!("{:?}", &nes_rom_data);
}
