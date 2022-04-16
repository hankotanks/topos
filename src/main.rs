mod mesh;
mod display;
mod uniforms;
mod pgm;

use std::fs::File;

fn main() {
    let file = File::open("./images/two.pgm").unwrap();
    let image = crate::pgm::GrayMapImage::new(file).unwrap();

    let mesh = crate::mesh::Mesh::new(image, 64, 64, true);
    crate::display::begin(mesh);

}

// TODO: Flip the image so that the render is right-side up