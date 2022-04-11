#[derive(Debug, Copy, Clone)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3]
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Normal {
    pub(crate) normal: [f32; 3]
}

impl Normal {
    fn new(a: &Vertex, b: &Vertex, c: &Vertex) -> Normal {
        let v = Vertex {
            position: [
                b.position[0] - a.position[0],
                b.position[1] - a.position[1],
                b.position[2] - a.position[2]
            ]
        }.position;

        let w = Vertex {
            position: [
                c.position[0] - a.position[0],
                c.position[1] - a.position[1],
                c.position[2] - a.position[2]
            ]
        }.position;

        Normal {
            normal: [
                v[1] * w[2] - v[2] * w[1],
                v[2] * w[0] - v[0] * w[2],
                v[0] * w[1] - v[1] * w[0]
            ]
        }
    }
}

pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) struct Mesh {
    pub(crate) positions: Vec<Vertex>,
    pub(crate) normals: Vec<Normal>,
    pub(crate) indices: Vec<u16>,
    image: crate::bmp::BitmapImage,
    scale: u32,
    smoothing: bool,
    x: u32,
    y: u32,
    height: u32,
    width: u32,
}

impl Mesh {
    pub(crate) fn new(image: crate::bmp::BitmapImage, height: u32, width: u32, smoothing: bool) -> Mesh {
        let dimensions = image.dimensions;

        // initialize mesh w/o populated position/normal/index vectors
        let mut mesh = Mesh {
            positions: vec![Vertex { position: [0f32; 3] }; (height * width + 1) as usize],
            normals: vec![Normal { normal: [0f32; 3] }; (height * width + 1) as usize],
            indices: vec![0u16; (6 * (height - 1) * (width - 1)) as usize],
            image,
            scale: 1u32,
            smoothing,
            x: (dimensions.0 / 2) - (width / 2),
            y: (dimensions.1 / 2) - (height / 2),
            height,
            width
        };

        mesh.get_indices(); // indices only need to be calculated once
        mesh.update();
        mesh
    }

    pub(crate) fn update(&mut self) {
        self.get_positions();
        self.get_normals();
    }

    pub(crate) fn update_view(&mut self, direction: Direction) {
        let offset: (isize, isize) = match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        };

        let ox = self.x as isize + offset.0;
        let oy = self.y as isize + offset.1;

        if ox >= 0 && (ox + (self.width * self.scale) as isize) < self.image.dimensions.0 as isize && oy >= 0 &&
            (oy + (self.height * self.scale) as isize) < self.image.dimensions.1 as isize {
            self.x = ox as u32;
            self.y = oy as u32;
        }
    }

    pub(crate) fn scale(&mut self, offset: isize) {
        let os = (self.scale as isize + offset) as u32;

        if os == 0 { return; }

        if self.x + os * self.width < self.image.dimensions.0 && self.y + os * self.height < self.image.dimensions.1 {
            self.scale = os;
            if offset > 0 {
                self.x -= offset.abs() as u32 * self.width / 2;
                self.y -= offset.abs() as u32 * self.height / 2;
            } else if offset < 0 {
                self.x += offset.abs() as u32 * self.width / 2;
                self.y += offset.abs() as u32 * self.height / 2;
            }

            self.update();
        }


    }

    fn get_positions(&mut self) {
        let view = self.image.view(self.x, self.y, self.width, self.height, self.scale);

        for y in 0..self.height {
            let y_pos = (y as f32 / self.height as f32) - 0.5f32;
            for x in 0..self.width {
                let x_pos = (x as f32 / self.width as f32) - 0.5f32;

                let intensity = view[y as usize][x as usize];

                self.positions[(self.width * y + x) as usize + 1] = Vertex {
                    position: [x_pos, y_pos, intensity]
                };
            }
        }

        if self.smoothing {
            loop {
                self.flatten();
                if self.deviation() < 0.1f32 {
                    break;
                }
            }
        }
    }

    fn get_indices(&mut self) {
        let mut index = 0;
        for y in 0..(self.height - 1) {
            for x in 0..(self.width - 1) {
                self.indices[index] = (y * self.width + x + 1) as u16;
                self.indices[index + 1] = ((y + 1) * self.width + x + 1) as u16;
                self.indices[index + 2] = ((y + 1) * self.width + x + 2) as u16;
                self.indices[index + 3] = (y * self.width + x + 1) as u16;
                self.indices[index + 4] = (y * self.width + x + 2) as u16;
                self.indices[index + 5] = ((y + 1) * self.width + x + 2) as u16;

                index += 6;
            }
        }
    }

    fn get_normals(&mut self) {
        for index in (0..(self.indices.len() - 2)).step_by(3) {
            let a = &self.positions[self.indices[index] as usize];
            let b = &self.positions[self.indices[index + 1] as usize];
            let c = &self.positions[self.indices[index + 2] as usize];

            self.normals[self.indices[index] as usize] = Normal::new(a, b, c);
            self.normals[self.indices[index + 1] as usize] = Normal::new(b, a, c);
            self.normals[self.indices[index + 2] as usize] = Normal::new(c, a, b);
        }
    }

    fn deviation(&self) -> f32 {
        let heights: Vec<f32> = self.positions.iter().map(|v| v.position[2]).collect();

        let average: f32 = heights.iter().sum();
        let average: f32 = average / self.height as f32 / self.width as f32;

        let mut deviation = 0.0f32;
        for intensity in heights.iter() {
            deviation += (average - intensity.abs()).abs();
        }

        deviation /= self.height as f32 * self.width as f32;
        deviation
    }

    fn flatten(&mut self) {
        let intensity = self.positions.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut adj = 0;
                let mut average = 0.0f32;

                if x as isize - 1 >= 0 { adj += 1; average += intensity[(y * self.width + x) as usize].position[2] };
                if x + 1 < self.width { adj += 1; average += intensity[(y * self.width + x + 2) as usize].position[2] };
                if y as isize - 1 >= 0 { adj += 1; average += intensity[((y - 1) * self.width + x + 1) as usize].position[2] };
                if y + 1 < self.height { adj += 1; average += intensity[((y + 1) * self.width + x + 1) as usize].position[2] };

                average /= adj as f32;
                self.positions[(y * self.width + x + 1) as usize].position[2] += average;
                self.positions[(y * self.width + x + 1) as usize].position[2] /= 2f32;
            }
        }
    }
}

