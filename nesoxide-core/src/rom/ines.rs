use anyhow::{bail, Result};
use crate::is_bit_offset_set;

#[derive(Debug)]
#[repr(u8)]
pub enum HardWiredMirroringType {
    HorizontalOrMapper = 0x0,
    Vertical = 0x1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Battery {
    NotPresent = 0x0,
    Present = 0x1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum HardWiredFourScreenMode {
    No = 0x0,
    Yes = 0x1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum ConsoleType {
    NesOrFamicom = 0x0,
    NintendoVsSystem = 0x1,
    NintendoPlaychoice10 = 0x2,
}

#[derive(Debug)]
#[repr(u8)]
pub enum TvSystem {
    Ntsc = 0x0,
    Pal = 0x1,
}

#[derive(Debug)]
pub struct INes {
    size: usize,
    header: Vec<u8>,
    trainer: Vec<u8>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    chr_ram_size: usize,
    prg_ram_size: usize,
    pc_inst_rom: Vec<u8>,
    pc_prom_data: Vec<u8>,
    pc_prom_counterout: Vec<u8>,
    title: Vec<u8>,
    mapper: u8,
    hard_wired_mirroring_type: HardWiredMirroringType,
    battery: Battery,
    hard_wired_four_screen_mode: HardWiredFourScreenMode,
    console_type: ConsoleType,
    tv_system: TvSystem,
}

impl INes {
    pub fn new(bytes: Vec<u8>) -> Result<INes> {
        if bytes[0..4] != [0x4E, 0x45, 0x53, 0x1A] {
            bail!("Invalid INES ROM.")
        }

        let mut byte_offset = 0;

        let size = bytes.len();

        let header = bytes[byte_offset..16].to_vec();
        byte_offset += 16;

        let trainer = if is_bit_offset_set!(bytes[6], 2) {
            let pre_byte_offset = byte_offset;
            byte_offset += 512;
            bytes[pre_byte_offset..byte_offset].to_vec()
        } else {
            Vec::new()
        };

        let prg_rom = if bytes[3] > 0 {
            let pre_byte_offset = byte_offset;
            byte_offset += 16384*bytes[4] as usize;
            bytes[pre_byte_offset..byte_offset].to_vec()
        } else {
            Vec::new()
        };

        let chr_rom = if bytes[5] > 0 {
            let pre_byte_offset = byte_offset;
            byte_offset += 8192*bytes[5] as usize;
            bytes[pre_byte_offset..byte_offset].to_vec()
        } else {
            Vec::new()
        };

        let chr_ram_size = 0;

        let prg_ram_size = if is_bit_offset_set!(bytes[9], 4) {
            0
        } else {
            8192*bytes[8] as usize
        };

        let pc_inst_rom = if is_bit_offset_set!(bytes[6], 1) {
            let pre_byte_offset = byte_offset;
            byte_offset += 8192;
            bytes[pre_byte_offset..byte_offset].to_vec()
        } else {
            Vec::new()
        };

        let pc_prom_data = if is_bit_offset_set!(bytes[6], 1) {
            let pre_byte_offset = byte_offset;
            byte_offset += 16384;
            bytes[pre_byte_offset..byte_offset].to_vec()
        } else {
            Vec::new()
        };

        let pc_prom_counterout = if is_bit_offset_set!(bytes[6], 1) {
            let pre_byte_offset = byte_offset;
            byte_offset += 16384;
            bytes[pre_byte_offset..byte_offset].to_vec()
        } else {
            Vec::new()
        };

        let title = if byte_offset < bytes.len() {
            bytes[byte_offset..bytes.len()].to_vec()
        } else {
            Vec::new()
        };

        let mapper = 0;

        let hard_wired_mirroring_type = if is_bit_offset_set!(bytes[5], 0) {
            HardWiredMirroringType::Vertical
        } else {
            HardWiredMirroringType::HorizontalOrMapper
        };

        let battery = if is_bit_offset_set!(bytes[5], 1) {
            Battery::Present
        } else {
            Battery::NotPresent
        };

        let hard_wired_four_screen_mode = if is_bit_offset_set!(bytes[5], 3) {
            HardWiredFourScreenMode::Yes
        } else {
            HardWiredFourScreenMode::No
        };

        let console_type = if is_bit_offset_set!(bytes[6], 0) {
            ConsoleType::NintendoVsSystem
        } else if is_bit_offset_set!(bytes[6], 1) {
            ConsoleType::NintendoPlaychoice10
        } else {
            ConsoleType::NesOrFamicom
        };

        let tv_system = if is_bit_offset_set!(bytes[8], 0) {
            TvSystem::Pal
        } else {
            TvSystem::Ntsc
        };

        Ok(INes {
            size,
            header,
            trainer,
            prg_rom,
            chr_rom,
            chr_ram_size,
            prg_ram_size,
            pc_inst_rom,
            pc_prom_data,
            pc_prom_counterout,
            title,
            mapper,
            hard_wired_mirroring_type,
            battery,
            hard_wired_four_screen_mode,
            console_type,
            tv_system,
        })
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_header(&self) -> Vec<u8> {
        self.header.to_vec()
    }

    fn get_trainer(&self) -> Vec<u8> {
        self.trainer.to_vec()
    }

    fn is_trainer_present(&self) -> bool {
        if self.trainer.len() == 0 { false } else { true }
    }

    fn get_trainer_size(&self) -> usize {
        self.trainer.len()
    }

    fn get_prg_rom(&self) -> Vec<u8> {
        self.prg_rom.to_vec()
    }

    fn get_prg_rom_size(&self) -> usize {
        self.prg_rom.len()
    }

    fn get_chr_rom(&self) -> Vec<u8> {
        self.chr_rom.to_vec()
    }

    fn get_chr_rom_size(&self) -> usize {
        self.chr_rom.len()
    }

    fn is_chr_rom_present(&self) -> bool {
        if self.chr_rom.len() == 0 { false } else { true }
    }

    fn get_chr_ram_size(&self) -> usize {
        self.chr_ram_size
    }

    fn is_chr_ram_present(&self) -> bool {
        if self.chr_ram_size == 0 { false } else { true }
    }

    fn get_prg_ram_size(&self) -> usize {
        self.prg_ram_size
    }

    fn is_prg_ram_present(&self) -> bool {
        if self.prg_ram_size == 0 { false } else { true }
    }

    fn get_title(&self) -> Vec<u8> {
        self.title.to_vec()
    }

    fn is_title_present(&self) -> bool {
        if self.title.len() == 0 { false } else { true }
    }

    fn get_mapper(&self) -> u8 {
        self.mapper
    }

    fn get_hard_wired_mirroring_type(&self) -> &HardWiredMirroringType {
        &self.hard_wired_mirroring_type
    }

    fn get_battery(&self) -> &Battery {
        &self.battery
    }

    fn get_hard_wired_four_screen_mode(&self) -> &HardWiredFourScreenMode {
        &self.hard_wired_four_screen_mode
    }

    fn get_console_type(&self) -> &ConsoleType {
        &self.console_type
    }

    fn get_tv_system(&self) -> &TvSystem {
        &self.tv_system
    }
}