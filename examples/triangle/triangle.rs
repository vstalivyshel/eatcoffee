use eatcoffee::{display, pipeline};
use std::borrow::Cow;
use winit::{
    window::Window,
    event_loop::{EventLoop, ControlFlow},
    event::*,
};

fn draw(display: &display::Display, pipeline: &wgpu::RenderPipeline) {
    let frame = display.surface
        .get_current_texture()
        .expect("Failed to acuire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None
    });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(pipeline);
        rpass.draw(0..3, 0..1);
    }
    display.queue.submit(Some(encoder.finish()));
    frame.present();
}


async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let mut display = display::Display::new(window).await.unwrap();

    let pipeline_layout = display.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let vs = wgpu::ShaderModuleDescriptor {
        label: None,
        source:
            wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("vertex.wgsl")))
    };

    let fs = wgpu::ShaderModuleDescriptor {
        label: None,
        source:
            wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("fragment.wgsl")))
    };

    let pipeline = pipeline::RenderPipelineBuilder::new()
        .layout(&pipeline_layout)
        .vertex_shader(vs)
        .fragment_shader(fs)
        .color_solid(display.config.format)
        .build(&display.device)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let _ = (&pipeline_layout, &display);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), ..  } => {
                display.resize(size);
            }
            Event::RedrawRequested(_) => draw(&display, &pipeline),
            _ => {}
        }
    })
}


fn main() {
    pollster::block_on(run());
}
