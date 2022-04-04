use glium::{Display, Frame, glutin, implement_vertex, program, Surface, uniform};
use image::DynamicImage;

use crate::mesh::Mesh;
use crate::mesh::{Vertex, Normal};

struct DisplayHandler<'a> {
    program: glium::Program,
    parameters: glium::DrawParameters<'a>,
    mesh: Mesh<'a>,

}

impl<'a> DisplayHandler<'a> {
    fn new(display: &glium::Display, mesh: Mesh<'a>) -> DisplayHandler<'a> {
        implement_vertex!(Vertex, position);
        implement_vertex!(Normal, normal);

        let parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let program = program!(display, 150 => {
                vertex: &*std::fs::read_to_string("./src/topo.vert.glsl").unwrap(),
                fragment: &*std::fs::read_to_string("./src/topo.frag.glsl").unwrap()
            }).unwrap();

        DisplayHandler {
            program,
            parameters,
            mesh
        }
    }

    fn draw(&self, display: &Display) {
        let mut frame = display.draw();

        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        frame.draw(
            (
                &glium::VertexBuffer::new(display, &self.mesh.positions).unwrap(),
                &glium::VertexBuffer::new(display, &self.mesh.normals).unwrap()),
            &glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &self.mesh.indices).unwrap(),
            &self.program,
            &uniform! {
                model: crate::uniforms::MODEL,
                perspective: crate::uniforms::PERSPECTIVE,
                light: crate::uniforms::LIGHT
            },
            &self.parameters,
        ).unwrap();

        frame.finish().unwrap();
    }
}

pub(crate) fn begin(image: DynamicImage) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = glium::Display::new(
        glutin::window::WindowBuilder::new(),
        glutin::ContextBuilder::new().with_depth_buffer(24),
        &event_loop).unwrap();
    //let image = image::open(file).unwrap();
    let mesh = crate::mesh::Mesh::new(&image, 5, 5);
    let handler = DisplayHandler::new(&display, mesh);

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        handler.draw(&display);

    });
}