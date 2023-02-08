use crate::setup::*;
use std::future::Future;
use winit::{
    event::{self, *},
    event_loop::ControlFlow,
};

#[derive(Default)]
pub struct Spawner<'a> {
    executor: async_executor::LocalExecutor<'a>,
}

impl<'a> Spawner<'a> {
    pub fn new() -> Self {
        Self {
            executor: async_executor::LocalExecutor::new(),
        }
    }

    pub fn spawn_local(&self, future: impl Future<Output = ()> + 'a) {
        self.executor.spawn(future).detach();
    }

    pub fn run_until_stalled(&self) {
        while self.executor.try_tick() {}
    }
}


pub trait Example: 'static + Sized {
    fn optional_features() -> wgpu::Features {
        wgpu::Features::empty()
    }

    fn required_features() -> wgpu::Features {
        wgpu::Features::empty()
    }

    // For WebGPU
    fn required_downlevel_capabilities() -> wgpu::DownlevelCapabilities {
        wgpu::DownlevelCapabilities {
            flags: wgpu::DownlevelFlags::empty(),
            shader_model: wgpu::ShaderModel::Sm5,
            ..Default::default()
        }
    }
        
    // Limits to let run code on any hardware
    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::downlevel_webgl2_defaults()
    }

    fn init(
        config: &wgpu::SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self;

    fn resize(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    );

    fn update(&mut self, event: winit::event::WindowEvent);

    fn render(
        &mut self,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        spawner: &Spawner,
    );
}

pub fn start<E: Example>( 
    Setup{
        window,
        event_loop,
        instance,
        size,
        surface,
        adapter,
        device,
        queue 
    }: Setup
    ) {
    let spawner = Spawner::new();
    log::info!("Configurating Surface");
    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .expect("Surface isn't supported by the adapter");
    surface.configure(&device, &config);

    log::info!("Setting Example");
    let mut state = E::init(&config, &adapter, &device, &queue);

    let mut last_frame_inst = std::time::Instant::now();
    let (mut frame_count, mut accum_time) = (0, 0.0);

    log::info!("Entering render loop");
    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter);
        *control_flow = ControlFlow::Poll;
        match event {
            Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size) 
                    | WindowEvent::ScaleFactorChanged {
                        new_inner_size: &mut size, ..
                    }, ..
            } => {
                log::info!("Resizing to {:?}", size);
                config.width = size.width.max(1);
                config.height = size.height.max(1);
                state.resize(&config, &device, &queue);
                surface.configure(&device, &config);
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { input, .. } => match input {
                    event::KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::R),
                        state: ElementState::Pressed,
                        ..
                    } => println!("{:#?}", instance.generate_report()),

                    event::KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {},
                }
                _ => {}
            }
            Event::RedrawRequested(_) => {
                {
                    accum_time += last_frame_inst.elapsed().as_secs_f32();
                    last_frame_inst = std::time::Instant::now();
                    frame_count += 1;
                    if frame_count == 100 {
                        println!(
                            "Avg frame time {}ms",
                            accum_time * 1000.0 / frame_count as f32
                        );
                        accum_time = 0.0;
                        frame_count = 0;
                    }
                }

                let frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        log::error!("Failed to get current texture: {}", e);
                        surface.configure(&device, &config);
                        surface
                            .get_current_texture()
                            .expect("Failed to acquire next surface texture!")
                    }
                };
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

                state.render(&view, &device, &queue, &spawner);

                frame.present();
            }
            _ => {}
        }
    });
}

