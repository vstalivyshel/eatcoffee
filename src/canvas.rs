use anyhow::*;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
    event::*,
};

pub struct Canvas {
    event_loop: EventLoop,
    window: Window,
    surface: wgpu::Surface,
    instance: wgpu::Instance,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Canvas {
    pub async fn new() -> Result<Self, Error> {
        env_logger::init();

        let event_loop = EventLoop::new();

        let window = WindowBuilder::new().build(&event_loop)
            .expect("Failed to create window");

        let size = window.inner_size();

        // Instance is a handle to GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.expect("Failed to create surface");

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
        }, 
        None,
        ).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let suface_format = surface_capabilities.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(Self {
            event_loop,
            window, 
            device,
            instance,
            surface,
            config,
            queue,
        })
    }

    pub fn input_event() -> bool {
        false
    }

    pub async fn draw(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { window_id, event } if window_id == self.window.id() => 
                    match event{
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
                _ => {}
            }
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) { 
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
