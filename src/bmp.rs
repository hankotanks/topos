use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{ByteOrder, LittleEndian};

pub(crate) struct BitmapImage {
    file: File,
    pub(crate) dimensions: (u32, u32),
    pixel_array_start: u32,
}

impl BitmapImage {
    pub(crate) fn new(mut file: File) -> Result<BitmapImage, std::io::Error> {
        let mut buffer = [0; 4];

        // grab width of image
        file.by_ref().seek(SeekFrom::Start(18))?;
        file.by_ref().take(4).read(&mut buffer)?;
        let width = LittleEndian::read_u32(buffer.as_slice());

        // height of image
        file.by_ref().take(4).read(&mut buffer)?;
        let height = LittleEndian::read_u32(buffer.as_slice());

        // find the offset that specifies the beginning of the pixel array
        file.by_ref().seek(SeekFrom::Start(10))?;
        file.by_ref().take(4).read(&mut buffer)?;
        let pixel_array_start = LittleEndian::read_u32(buffer.as_slice());

        Ok(BitmapImage {
            file,
            dimensions: (width, height),
            pixel_array_start
        })
    }

    pub(crate) fn view(&mut self, x: u32, y: u32, height: u32, width: u32, scale: u32) -> Vec<Vec<f32>> {
        let mut pixels: Vec<Vec<f32>> = Vec::new();

        // define some offsets
        let padding = self.dimensions.0 % 4;
        let row = self.dimensions.0 * 3 + padding;
        let pre = x * 3;
        let post = row - width * 3 - pre;

        // move cursor to beginning of pixel array
        self.file.by_ref().seek(SeekFrom::Start(self.pixel_array_start as u64)).unwrap();

        // skip to the appropriate row
        self.file.by_ref().seek(SeekFrom::Current((row * y) as i64)).unwrap();
        for _ in 0..height {
            // advance cursor to the beginning of the subregion
            self.file.by_ref().seek(SeekFrom::Current(pre as i64)).unwrap();

            // the current row of pixels values, assembled before being appended to the `pixels` Vec
            let mut current: Vec<f32> = Vec::new();
            for _  in 0..width {
                // grab the current pixel
                let mut color = [0u8; 3];
                self.file.by_ref().take(3).read(&mut color).unwrap();

                // normalize and linearize
                let color = color.map(|channel| channel as f32 / 255f32);
                let color = color.map(|channel| linearize(channel));

                // calculate luminance
                // divide by 100 to ensure height value is appropriate
                let luminance = 0.2126 * color[0] + 0.7152 * color[1] + 0.0722 * color[2];
                let luminance = {
                    if luminance <= (216f32 / 24389f32) {
                        luminance * (24389f32 / 27f32)
                    } else {
                        luminance.powf(1f32 / 3f32) * 116f32 - 16f32
                    }
                } / 100f32;

                // add to Vec of current pixels
                current.push(luminance);

            }

            // push row to `pixels` Vec
            pixels.push(current);

            // skip to the end of the current row -- before reading the next
            self.file.by_ref().seek(SeekFrom::Current(post as i64)).unwrap();
        }

        pixels
    }
}

// linearize color channel
// used for perceived lightness calculation
// more information found here: https://stackoverflow.com/questions/596216/formula-to-determine-perceived-brightness-of-rgb-color
fn linearize(channel: f32) -> f32 {
    return if channel < 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}