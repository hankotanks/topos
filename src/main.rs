mod mesh;
mod display;
mod uniforms;

fn main() {
    let mesh = crate::mesh::Mesh::new(
        image::open("./images/medium.png").unwrap(), 64, 64);
    crate::display::begin(mesh);
}
