use image::{DynamicImage, GenericImage, GenericImageView, Pixel};

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
        let a = a.position;
        let b = b.position;
        let c = c.position;

        let v = Vertex {
            position: [
                b[0] - a[0],
                b[1] - a[1],
                b[2] - a[2]
            ]
        }.position;

        let w = Vertex {
            position: [
                c[0] - a[0],
                c[1] - a[1],
                c[2] - a[2]
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
    image: DynamicImage,
    x: u32,
    y: u32,
    height: u32,
    width: u32,
}

impl Mesh {
    pub(crate) fn new(image: DynamicImage, height: u32, width: u32) -> Mesh {
        let dimensions = image.dimensions();
        // initialize mesh w/o populated position/normal/index vectors
        let mut mesh = Mesh {
            positions: vec![Vertex { position: [0f32; 3] }; (height * width + 1) as usize],
            normals: vec![Normal { normal: [0f32; 3] }; (height * width + 1) as usize],
            indices: vec![0u16; (6 * (height - 1) * (width - 1)) as usize],
            image,
            x: (dimensions.0 / 2) - (width / 2),
            y: (dimensions.1 / 2) - (height / 2),
            height,
            width
        };

        mesh.get_indices();
        mesh.update();
        mesh
    }

    pub(crate) fn update(&mut self) {
        self.get_positions();
        self.get_normals();
        println!("{:?}", self.positions);
        println!("{:?}", self.normals);
    }

    pub(crate) fn update_view(&mut self, direction: Direction) {
        let offset: (isize, isize) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        };

        let ox = self.x as isize + offset.0;
        let oy = self.y as isize + offset.1;

        if ox >= 0 && ox < (self.x + self.width) as isize && oy >= 0 && oy < (self.y + self.height) as isize {
            self.x = ox as u32;
            self.y = oy as u32;
        }
    }

    fn get_positions(&mut self) {
        let view = self.image.sub_image(self.x, self.y, self.width, self.height);
        for y in 0..self.height {
            let y_pos = (y as f32 / self.height as f32) - 0.5f32;
            for x in 0..self.width {
                let intensity = view.get_pixel(x as u32, y as u32).to_luma().0[0];
                let intensity = intensity as f32 / 800f32;
                let x_pos = (x as f32 / self.width as f32) - 0.5f32;
                self.positions[(self.width * y + x) as usize + 1] = Vertex {
                    position: [x_pos, y_pos, intensity]
                };
            }
        }

    }

    fn get_indices(&mut self) {
        for y in 0..(self.height - 1) {
            for x in 0..(self.width - 1) {
                self.indices[y as usize * 6] = (y * self.width + x + 1) as u16;
                self.indices[y as usize * 6 + 1] = ((y + 1) * self.width + x + 1) as u16;
                self.indices[y as usize * 6 + 2] = ((y + 1) * self.width + x + 2) as u16;
                self.indices[y as usize * 6 + 3] = (y * self.width + x + 1) as u16;
                self.indices[y as usize * 6 + 4] = (y * self.width + x + 2) as u16;
                self.indices[y as usize * 6 + 5] = ((y + 1) * self.width + x + 2) as u16;
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
}

