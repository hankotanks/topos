mod mesh;
mod display;
mod uniforms;

fn main() {
    let image = image::open("./images/medium.png").unwrap();
    crate::display::begin(image);
}
