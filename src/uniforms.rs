pub(crate) const MODEL: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.5, 1.0]
];

pub(crate) const LIGHT: [f32; 3] = [
    -1.0, 0.4, 0.9f32
];

pub(crate) const F: f32 = 1.0 / (std::f32::consts::PI / 6.0).tan();

pub(crate) const PERSPECTIVE: [[f32; 4]; 4] = {
    let far = 1024.0;
    let near = 0.1;

    [
        [F, 0.0, 0.0, 0.0],
        [0.0, F, 0.0, 1.0],
        [0.0, 0.0, (far + near) / (far - near), 1.0],
        [0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0]
    ]
};