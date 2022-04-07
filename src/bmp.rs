use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{ByteOrder, LittleEndian};

pub(crate) struct BitmapImage {
    file: File,
    dimensions: (u32, u32),
    pixel_array_start: u32,
}

impl BitmapImage {
    pub(crate) fn new(mut file: File) -> Result<BitmapImage, std::io::Error> {
        let mut buffer = [0; 4];

        let mut dimensions = (0u32, 0u32);

        // grab width of image
        file.by_ref().seek(SeekFrom::Start(18))?;
        file.by_ref().take(4).read(&mut buffer)?;
        dimensions.0 = LittleEndian::read_u32(buffer.as_slice());

        // height of image
        file.by_ref().take(4).read(&mut buffer)?;
        dimensions.1 = LittleEndian::read_u32(buffer.as_slice());

        // set the reader to the beginning of the pixel array
        file.by_ref().seek(SeekFrom::Start(10))?;
        file.by_ref().take(4).read(&mut buffer)?;
        let pixel_array_start = LittleEndian::read_u32(buffer.as_slice());

        Ok(BitmapImage {
            file,
            dimensions,
            pixel_array_start
        })
    }

    pub(crate) fn view(&mut self, x: u32, y: u32, height: u32, width: u32) -> Result<Vec<f32>, std::io::Error> {
        let mut buffer = [0; 1];
        let mut pixels: Vec<f32> = Vec::new();

        self.file.by_ref().seek(SeekFrom::Start(self.pixel_array_start as u64))?;

        let row = (self.dimensions.1 + self.dimensions.1 % 4) * 3;
        let pre_x = 3 * x;
        let post_x = row  - width * 3;
        // move pointer to beginning of subregion
        self.file.by_ref().seek(SeekFrom::Current((row * y + pre_x) as i64))?;
        for _ in 0..height {
            for _  in 0..width {
                let mut color = [0f32; 3];
                self.file.by_ref().take(1).read(&mut buffer)?;
                color[2] = buffer[0] as f32 / 255f32;
                self.file.by_ref().take(1).read(&mut buffer)?;
                color[1] = buffer[0] as f32 / 255f32;
                self.file.by_ref().take(1).read(&mut buffer)?;
                color[0] = buffer[0] as f32 / 255f32;

                let luminance = 0.2126 * linearize(color[0]) +
                    0.7152 * linearize(color[1]) +
                    0.0722 * linearize(color[2]);

                pixels.push(lightness(luminance));

            }
            self.file.by_ref().seek(SeekFrom::Current(post_x as i64))?;
        }


        Ok(pixels)
    }
}

fn linearize(channel: f32) -> f32 {
    return if channel < 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}


fn lightness(luminance: f32) -> f32 {
    return if luminance <= (216f32 / 24389f32) {
        luminance * (24389f32 / 27f32)
    } else {
        luminance.powf(1f32 / 3f32) * 116f32 - 16f32
    }
}