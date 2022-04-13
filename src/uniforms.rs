pub(crate) const LIGHT: [f32; 3] = [
    -1.0, 0.4, 0.9f32
];

pub(crate) fn get_model(zoom: f32) -> [[f32; 4]; 4] {
    [
        [zoom, 0.0,  0.0,  0.0],
        [0.0,  zoom, 0.0,  0.0],
        [0.0,  0.0,  zoom, 0.0],
        [0.0,  0.0,  0.25,  1.0f32]
    ]
}

pub(crate) fn get_perspective(dimensions: (u32, u32)) -> [[f32; 4]; 4] {
    let ar = dimensions.1 as f32 / dimensions.0 as f32;

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