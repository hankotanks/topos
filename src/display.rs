use glium;
use glium::{program, Surface};
use glium::glutin;
use glium::glutin::event::{VirtualKeyCode, WindowEvent};

use crate::mesh::Mesh;
use crate::mesh::{Vertex, Normal};

struct DisplayHandler<'a> {
    program: glium::Program,
    parameters: glium::DrawParameters<'a>,
    mesh: Mesh,
    scale: f32
}

impl<'a> DisplayHandler<'a> {
    fn new(display: &glium::Display, mesh: Mesh) -> DisplayHandler<'a> {
        glium::implement_vertex!(Vertex, position);
        glium::implement_vertex!(Normal, normal);

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
            mesh,
            scale: 0.5f32
        }
    }

    fn draw(&self, display: &glium::Display) {
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
            &glium::uniform! {
                model: crate::uniforms::get_model(self.scale),
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
                    let direction: Option<crate::mesh::Direction> = match input.virtual_keycode {
                        Some(key) => {
                            match key {
                                glutin::event::VirtualKeyCode::Up => { Some(crate::mesh::Direction::Up) },
                                glutin::event::VirtualKeyCode::Down => { Some(crate::mesh::Direction::Down) },
                                glutin::event::VirtualKeyCode::Left => { Some(crate::mesh::Direction::Left) },
                                glutin::event::VirtualKeyCode::Right => { Some(crate::mesh::Direction::Right) },
                                other => {
                                    match other {
                                        glutin::event::VirtualKeyCode::PageUp => { handler.scale = (handler.scale + 0.02f32).min(1f32); },
                                        glutin::event::VirtualKeyCode::PageDown => { handler.scale = (handler.scale - 0.02f32).max(0f32); }
                                        _ => {}
                                    };
                                    None
                                }
                            }
                        }

                        None => { None }
                    };

                    match direction {
                        Some(direction) => {
                            handler.mesh.update_view(direction);
                            handler.mesh.update(); },
                        None => {}
                    }
                }
                _ => {}
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