use eatcoffee::{pipeline, canvas};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    event::*,
};

pub async fn draw() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut display = canvas::Display::new(window).await;

    let layout = display.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let pipeline = pipeline::RenderPipelineBuilder::new()
        .layout(&layout)
        .shader_source(wgpu::ShaderSource::Wgsl(include_str!("res/shader.wgsl").into()))
        .color_states(wgpu::ColorTargetState {
            format: display.config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })
        .build(&display.device);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(wid) 
                if wid == display.window.id() => match display.render(&pipeline) {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => display.resize(display.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                            Err(e) => eprint!("{e}"),
                }

            Event::MainEventsCleared => {
                display.window.request_redraw();
                }

            Event::WindowEvent { window_id, event } 
            if window_id == display.window.id() && !display.input_event(&event) => 
                match event{
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        display.resize(physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        display.resize(*new_inner_size);
                    }
                    _ => {}
            }
            _ => {}
        }
    })
}


fn main() {
    pollster::block_on(draw());
}
