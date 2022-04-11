pub(crate) const LIGHT: [f32; 3] = [
    -1.0, 0.4, 0.9f32
];

pub(crate) fn get_model(zoom: f32) -> [[f32; 4]; 4] {
    [
        [zoom, 0.0,  0.0,  0.0],
        [0.0,  zoom, 0.0,  0.0],
        [0.0,  0.0,  zoom, 0.0],
        [0.0,  0.0,  0.2,  1.0f32]
    ]
}

pub(crate) fn get_perspective(display: &glium::Display) -> [[f32; 4]; 4] {
    let ar = display.get_max_viewport_dimensions();
    let ar = (ar.1 / ar.0) as f32;

    let far = 1024.0;
    let near = 0.1;

    let f = 1.0 / (std::f32::consts::PI / 6.0).tan();

    [
        [f * ar, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 1.0],
        [0.0, 0.0, (far + near) / (far - near), 1.0],
        [0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0]
    ]

}