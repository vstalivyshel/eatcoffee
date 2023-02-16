use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct Setup {
    event_loop: EventLoop<()>,
    window: Window,
    size: winit::dpi::PhysicalSize<u32>,
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn setup(title: &str) -> Setup {
    log::info!("Creating Event Loop");
    let event_loop = EventLoop::new();

    log::info!("Building Window");
    let window = WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .expect("Failed to build window");

    log::info!("Creating Instance");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    log::info!("Setting up Surface");
    let (surface, size) = unsafe {
        let surface = instance.create_surface(&window).unwrap();
        let size = window.inner_size();
        (surface, size)
    };

    log::info!("Setting up Adapter");
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    log::info!("Creating Device");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: Some("Device"),
            },
            None,
        )
        .await
        .expect("Failed to create Device");

    Setup {
        event_loop,
        window,
        size,
        instance,
        surface,
        adapter,
        device,
        queue,
    }
}

pub trait Init: 'static + Sized {
    fn init(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self;
    fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue);
    fn update(&mut self, event: WindowEvent);
}

fn prepare<I: Init>(
    Setup {
        event_loop,
        window,
        size,
        instance,
        surface,
        adapter,
        device,
        queue,
    }: Setup,
) {
    log::info!("Iitializing...");
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.describe().srgb)
        .unwrap_or(surface_caps.formats[0]);
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    log::info!("Configurating the Surface");
    let mut state = I::init(&device, &config);

    log::info!("Running Event Loop");
    event_loop.run(move |event, _, cf| {
        let _ = (&instance, &adapter);
        *cf = ControlFlow::Poll;
        match event {
            Event::RedrawEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                let frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        log::error!("Failed to get current texture: {e}");
                        surface.configure(&device, &config);
                        surface
                            .get_current_texture()
                            .expect("Failed to acquire next surface texture")
                    }
                };
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                state.render(&view, &device, &queue);
                frame.present();
            }
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::Resized(size)
                | WindowEvent::ScaleFactorChanged {
                    new_inner_size: &mut size,
                    ..
                } => {
                    log::info!("Resizing to {size:?}");
                    config.width = size.width.max(1);
                    config.height = size.height.max(1);
                    surface.configure(&device, &config);
                }
                WindowEvent::CloseRequested => *cf = ControlFlow::Exit,
                _ => state.update(event),
            },
            _ => {}
        }
    });
}

pub fn draw<I: Init>(title: &str) {
    let setup = pollster::block_on(setup(title));
    prepare::<I>(setup);
}
