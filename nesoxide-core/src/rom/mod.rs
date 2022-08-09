use std::fs::{self, File};
use std::io::Read;
use anyhow::Result;

mod ines;
use ines::*;

#[derive(Debug)]
pub struct Rom {
    data: INes,
}

impl Rom {
    pub fn load(file_name: &str) -> Result<Rom> {
        let file = File::open(&file_name)?;
        let file_size = fs::metadata(&file_name)?.len();

        let mut file_buffer: Vec<u8> = vec![0; file_size as usize];
        let mut file_handle = file.take(file_size);
        file_handle.read(&mut file_buffer)?;

        Ok(
            Rom {
                data: INes::new(file_buffer.to_vec())?,
            }
        )
    }

    pub fn get_data(&self) -> &INes {
        &self.data
    }
}
