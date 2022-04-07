mod mesh;
mod display;
mod uniforms;
mod bmp;

use std::fs::File;


fn main() {
    bmp::read(
        File::open("./images/gebco.bmp").unwrap(),
        0, 0, 10, 10).unwrap();

    /*
    let mesh = crate::mesh::Mesh::new(
        image::open("./images/medium.png").unwrap(), 64, 64);
    crate::display::begin(mesh);
     */
}
