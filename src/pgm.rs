use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub(crate) struct GrayMapImage {
    file: File,
    pub(crate) dimensions: (u32, u32),
    maximum: f32,
    offset: u64
}

impl GrayMapImage {
    pub(crate) fn new(mut file: File) -> Result<GrayMapImage, std::io::Error> {
        let mut buffer = [0u8; 1];

        loop {
            seek_until_eol(&mut file)?;
            file.by_ref().take(1).read(&mut buffer)?;

            if (48..58).contains(&(buffer[0] as i32)) {
                file.by_ref().seek(SeekFrom::Current(-1))?;
                break;
            }
        }

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

        let maximum = read_until(&mut file, 10)?;
        let maximum = match std::str::from_utf8(maximum.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }.parse::<u32>().unwrap() as f32;

        let offset = file.by_ref().seek(SeekFrom::Current(0))?;

        Ok(GrayMapImage {
            file,
            dimensions: (width, height),
            maximum,
            offset
        })
    }

    pub(crate) fn view(&mut self, x: u32, y: u32, width: u32, height: u32, scale: u32) -> Vec<Vec<f32>> {
        let mut heights: Vec<Vec<f32>> = Vec::new();

        self.file.by_ref().seek(SeekFrom::Start(self.offset)).unwrap();
        self.file.by_ref().seek(SeekFrom::Current((self.dimensions.0 * y) as i64)).unwrap();

        for _row in 0..height{
            let mut current = capture(&mut self.file, x, width, scale, self.dimensions, self.maximum);

            if scale > 1 {
                self.file.by_ref().seek(SeekFrom::Current((self.dimensions.0 * (scale - 2)) as i64)).unwrap();

                let aux = capture(&mut self.file, x, width, scale, self.dimensions, self.maximum);

                let mut index = 0;
                current = current.iter().map(|h| {
                    index += 1; (h + aux[index - 1]) / 2f32
                } ).collect();
            }

            heights.push(current);
        }

        heights
    }
}

fn capture(file: &mut File, x: u32, width: u32, scale: u32, dimensions: (u32, u32), maximum: f32) -> Vec<f32> {
    // seek to beginning of view
    file.by_ref().seek(SeekFrom::Current(x as i64)).unwrap();

    let mut current: Vec<f32> = Vec::new();

    for _col in 0..width {
        let mut c = [0u8; 1];
        let mut color: Vec<u32> = Vec::new();

        file.by_ref().take(1).read(&mut c).unwrap();
        file.by_ref().seek(SeekFrom::Current((scale as i64 - 2i64).max(0i64))).unwrap();
        color.push(c[0] as u32);

        if scale > 1 {
            file.by_ref().take(1).read(&mut c).unwrap();
        }

        color.push(c[0] as u32);
        let color = (color.iter().sum::<u32>() / 2) as f32 / maximum;
        current.push(color);
    }

    // seek to eol
    file.by_ref().seek(SeekFrom::Current((dimensions.0 - width * scale - x) as i64)).unwrap();

    current


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