mod teapot;

use glium::{Display, Program, Surface, uniform, VertexBuffer};
use glium::glutin::ContextBuilder;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;

fn main() {
    run();
}

fn run() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();
    
    let fragment_shader = include_str!("fragment_shader.glsl");
    let vertex_shader = include_str!("vertex_shader.glsl");

    let positions = VertexBuffer::new(
        &display,
        &teapot::VERTICES).unwrap();

    let normals = VertexBuffer::new(
        &display,
        &teapot::NORMALS
    ).unwrap();

    let indices = glium::index::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES
    ).unwrap();

    let program = Program::from_source(
        &display,
        vertex_shader,
        fragment_shader,
        None
    ).unwrap();

    let matrix = [
        [0.1, 0.0, 0.0, 0.0],
        [0.0, 0.1, 0.0, 0.0],
        [0.0, 0.0, 0.1, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        target.draw(
            (&positions, &normals),
            &indices,
            &program,
            &uniform! {matrix: matrix},
            &Default::default()
        ).unwrap();
        target.finish().unwrap();

        match event {
            Event::WindowEvent {
                window_id: _,
                event
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => ()
            },
            _ => ()
        }
    });
}
