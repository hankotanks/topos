mod mesh;
mod display;
mod uniforms;
mod pgm;

use std::fs::File;

fn main() {
    let file = File::open("./images/sample.pgm").unwrap();
    let image = crate::pgm::GrayMapImage::new(file).unwrap();

    let mesh = crate::mesh::Mesh::new(image, 64, 64, false);
    crate::display::begin(mesh);

}

// TODO: Resolve issue where some .pgm files have 3 lines before dimensions, others have 2
// TODO: Flip the image so that the render is right-side up