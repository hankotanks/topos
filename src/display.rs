use glium::{Display, glutin, implement_vertex, program, Surface, uniform};
use glium::glutin::event::VirtualKeyCode;

use crate::mesh::Mesh;
use crate::mesh::{Vertex, Normal};

struct DisplayHandler<'a> {
    program: glium::Program,
    parameters: glium::DrawParameters<'a>,
    mesh: Mesh,
}

impl<'a> DisplayHandler<'a> {
    fn new(display: &glium::Display, mesh: Mesh) -> DisplayHandler<'a> {
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
                vertex: &*std::fs::read_to_string("./src/topo.vert.gl").unwrap(),
                fragment: &*std::fs::read_to_string("./src/topo.frag.gl").unwrap()
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
                model: crate::uniforms::get_model(),
                perspective: crate::uniforms::get_perspective(&display),
                light: crate::uniforms::LIGHT
            },
            &self.parameters,
        ).unwrap();

        frame.finish().unwrap();
    }
}

pub(crate) fn begin(mesh: Mesh) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = glium::Display::new(
        glutin::window::WindowBuilder::new(),
        glutin::ContextBuilder::new().with_depth_buffer(24),
        &event_loop).unwrap();
    let mut handler = DisplayHandler::new(&display, mesh);

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    let direction: Option<crate::mesh::Direction> = match input.virtual_keycode.unwrap() {
                        VirtualKeyCode::W => { Some(crate::mesh::Direction::Up) },
                        VirtualKeyCode::S => { Some(crate::mesh::Direction::Down) },
                        VirtualKeyCode::A => { Some(crate::mesh::Direction::Left) },
                        VirtualKeyCode::D => { Some(crate::mesh::Direction::Right) },
                        _ => { None },
                    };

                    match direction {
                        Some(direction) => {
                            handler.mesh.update_view(direction);
                            handler.mesh.update(); },
                        None => {  }
                    }
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