mod teapot;

use glam::Mat4;
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

    let fov:f32 = 90.0;
    let (window_width, window_height) = display.get_framebuffer_dimensions();
    // First we need a projection matrix which we can calculate from the FoV / screen aspect ratio
    let projection_matrix = Mat4::perspective_rh_gl(
        fov.to_radians(),
        (window_width as f32) / (window_height as f32),
        0.1,
        1000.0,
    );
    // We also need a view matrix to move the object away from the camera, since we would otherwise see the kettle from the inside
    let view_matrix = Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -200.0));
    // Instead one could also send both matrices to the shader and multiply them there
    let matrix =  projection_matrix * view_matrix;
    // And finally we convert the matrix into a format that we can send to the shader
    let matrix = matrix.to_cols_array_2d();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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
    });
}
