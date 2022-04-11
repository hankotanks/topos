mod mesh;
mod display;
mod uniforms;
mod bmp;

use std::fs::File;

fn main() {
    let file = File::open("./images/large.bmp").unwrap();
    let bm = crate::bmp::BitmapImage::new(file).unwrap();

    let mesh = crate::mesh::Mesh::new(bm, 64, 64, true);

    crate::display::begin(mesh);

}
