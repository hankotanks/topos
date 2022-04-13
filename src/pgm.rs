use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub(crate) struct GrayMapImage {
    file: File,
    pub(crate) dimensions: (u32, u32),
    offset: u64
}

impl GrayMapImage {
    pub(crate) fn new(mut file: File) -> Result<GrayMapImage, std::io::Error> {
        seek_until_eol(&mut file)?;
        seek_until_eol(&mut file)?;

        let width = read_until(&mut file, 32)?;
        let width = match std::str::from_utf8(width.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }.parse::<u32>().unwrap();

        let height = read_until(&mut file, 10)?;
        let height = match std::str::from_utf8(height.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }.parse::<u32>().unwrap();

        seek_until_eol(&mut file)?;

        let offset = file.by_ref().seek(SeekFrom::Current(0))?;

        Ok(GrayMapImage {
            file,
            dimensions: (width, height),
            offset
        })
    }

    pub(crate) fn view(&mut self, x: u32, y: u32, width: u32, height: u32, scale: u32) -> Vec<Vec<f32>> {
        let mut heights: Vec<Vec<f32>> = Vec::new();

        self.file.by_ref().seek(SeekFrom::Start(self.offset)).unwrap();
        self.file.by_ref().seek(SeekFrom::Current((self.dimensions.0 * y) as i64)).unwrap();

        for _ in 0..height {
            self.file.by_ref().seek(SeekFrom::Current(x as i64)).unwrap();

            let mut current: Vec<f32> = Vec::new();
            for _ in 0..width {
                let mut color = [0u8; 1];
                self.file.by_ref().take(1).read(&mut color).unwrap();

                current.push(color[0] as f32 / 255f32);

                self.file.by_ref().seek(SeekFrom::Current((scale - 1) as i64)).unwrap();
            }

            heights.push(current);

            self.file.by_ref().seek(SeekFrom::Current((self.dimensions.0 - width * scale - x) as i64)).unwrap();
            self.file.by_ref().seek(SeekFrom::Current((self.dimensions.0 * (scale - 1)) as i64)).unwrap();
        }

        heights
    }
}

fn seek_until_eol(file: &mut File) -> Result<u64, std::io::Error> {
    let mut buffer = [0u8; 1];
    loop {
        file.by_ref().take(1).read(&mut buffer)?;
        if buffer[0] == 10 {
            break;
        }
    }

    file.by_ref().seek(SeekFrom::Current(0))
}

fn read_until(file: &mut File, value: u8) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = [0u8; 1];

    let mut bytes: Vec<u8> = Vec::new();

    loop {
        file.by_ref().take(1).read(&mut buffer)?;
        if buffer[0] == value {
            break;
        }

        bytes.push(buffer[0]);
    }

    Ok(bytes)
}