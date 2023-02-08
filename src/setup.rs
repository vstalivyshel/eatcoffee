use crate::init::Example;
use winit::{
    event_loop::EventLoop ,
    window::{Window,WindowBuilder},
    dpi::PhysicalSize,
};

pub struct Setup {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub size: PhysicalSize<u32>,
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub async fn setup<E: Example>(title: &str) -> Setup {
    {
        env_logger::init(); 
    };

    log::info!("Initializing the surface");

    log::info!("Creating EventLoop");
    let event_loop = EventLoop::new();

    log::info!("Creating Window");
    let window = WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
    let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();

    log::info!("Creating Instace");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        dx12_shader_compiler,
    });

    log::info!("Setting up Surface");
    let ( size, surface ) = unsafe { 
        let size = window.inner_size();
        let surface = instance.create_surface(&window).unwrap();

        (size, surface)
    };
    log::debug!("Inner Size: {:?}", size);

    log::info!("Setting up Adapter");
    let adapter = wgpu::util::initialize_adapter_from_env_or_default(
        &instance,
        backends,
        Some(&surface),
    ).await.expect("No suitable GPU adapter found on the system");

    {
        let adapter_info = adapter.get_info();
        log::info!("Using Adapter: {} ({:?})", adapter_info.name, adapter_info.backend);
    }

    let opt_features = E::optional_features();
    let req_features = E::required_features();
    let adapter_features = adapter.features();

    assert!(
        adapter_features.contains(req_features),
        "Adapter does not suport required features: {req_features:?}",
    );

    let req_downlevel_caps = E::required_downlevel_capabilities();
    let downlevel_caps = adapter.get_downlevel_capabilities();
    
    log::info!("Checking minimum shader model requirenments");
    assert!(
        downlevel_caps.shader_model >= req_downlevel_caps.shader_model,
        "Adapter does nto support this minimum shader model requirenments {:?}",
        req_downlevel_caps.shader_model
    );
    log::info!("Ok");

    log::info!("Checking downlevel capabilities support");
    assert!(
        downlevel_caps.flags.contains(req_downlevel_caps.flags),
        "Adapter does not support this downlevel capabilities: {:?}",
        req_downlevel_caps.flags - downlevel_caps.flags
    );
    log::info!("Ok");

    let needed_limits = E::required_limits().using_resolution(adapter.limits());

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Device Descriptor"),
            features: (opt_features & adapter_features) | req_features,
            limits: needed_limits,
        },
        None
    ).await.expect("Unable to find a suitable GPU adapter!");

    Setup {
        event_loop,
        window,
        size,
        instance,
        adapter,
        surface,
        device,
        queue,
    }
}

