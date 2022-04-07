mod mesh;
mod display;
mod uniforms;
mod bmp;

use std::fs::File;


fn main() {
    let mut bm = crate::bmp::BitmapImage::new(
        File::open("./images/gebco.bmp").unwrap()).unwrap();
    let bm = bm.view(0, 0, 10, 10);

    println!("{:?}", bm);


    /*
    let mesh = crate::mesh::Mesh::new(
        image::open("./images/medium.png").unwrap(), 64, 64);
    crate::display::begin(mesh);
     */
}
