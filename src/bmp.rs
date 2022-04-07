use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{ByteOrder, LittleEndian};

pub(crate) fn read(mut file: File, x: u32, y: u32, h: u32, w: u32) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = [0; 4];

    let height: u32;
    let width: u32;

    // grab width of image
    file.by_ref().seek(SeekFrom::Start(18))?;
    file.by_ref().take(4).read(&mut buffer)?;
    width = LittleEndian::read_u32(buffer.as_slice());

    // height of image
    file.by_ref().take(4).read(&mut buffer)?;
    height = LittleEndian::read_u32(buffer.as_slice());

    // set the reader to the beginning of the pixel array
    file.by_ref().seek(SeekFrom::Start(10))?;
    file.by_ref().take(4).read(&mut buffer)?;
    file.by_ref().seek(SeekFrom::Start(LittleEndian::read_u32(buffer.as_slice()) as u64))?;

    let pixels: Vec<u8> = Vec::new();

    Ok(pixels)
}