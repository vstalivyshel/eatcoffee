use thiserror::Error;
use winit::dpi::PhysicalSize;
use super::display::Display;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("Invalid shader")]
    InvalidShader,
    #[error("Invalid display format")]
    InvalidDisplayFormat,
}

pub struct SimulationData {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub diffuse_texture: wgpu::Texture,
    pub diffuse_rgba: image::RgbaImage, 
    pub texture_size: wgpu::Extent3d,
}


pub struct MagicCanvas {
    data: SimulationData,
}

impl MagicCanvas {

    pub fn resize(&mut self, display: &mut Display, new_size: PhysicalSize<u32>) { 
        if new_size.width > 0 && new_size.height > 0 {
            display.config.width = new_size.width;
            display.config.height = new_size.height;
            display.surface.configure(&display.device, &display.config);
        }
    }

    // pub fn render(
    //     &mut self,
    //     display: &mut Display,
    //     frame: &wgpu::TextureView,
    // ) {
    //      let mut encoder = display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //             label: Some("Render Encoder"),
    //         });

    //     let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("Shader Canvas Render Pass"),
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view: frame,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 load: wgpu::LoadOp::Load,
    //                 store: true,
    //             },
    //         })],
    //         depth_stencil_attachment: None,
    //     });
    //     pass.set_bind_group(0, &self.simulation_bind_group, &[]);
    //     pass.set_pipeline(&self.pipeline);
    //     pass.draw(0..6, 0..1);
    // }
}

// pub struct CanvasBuilder<'a> {
//     canvas_size: [f32; 2],
//     clear_color: [f32; 4],
//     label: Option<&'a str>,
//     display_format: Option<wgpu::TextureFormat>,
//     shader_src: Option<wgpu::ShaderModuleDescriptor<'a>>,
// }

// impl<'a> CanvasBuilder<'a> {
//     pub fn new() -> Self {
//         Self {
//             canvas_size: [256.0; 2],
//             clear_color: [0.0, 0.0, 0.0, 1.0],
//             label: None,
//             display_format: None,
//             shader_src: Some(wgpu::include_wgsl!("shader.wgsl")),
//         }
//     }

//     pub fn canvas_size(&mut self, width: f32, height: f32) -> &mut Self {
//         self.canvas_size = [width, height];
//         self
//     }

//     pub fn display_format(&mut self, format: wgpu::TextureFormat) -> &mut Self {
//         self.display_format = Some(format);
//         self
//     }

//     pub fn use_swap_chain_desc(&mut self, config: &wgpu::SurfaceConfiguration) -> &mut Self {
//         self.display_format(config.format);
//         self.canvas_size(config.width as f32, config.height as f32)
//     }

//     pub fn shader_src(&mut self, src: wgpu::ShaderModuleDescriptor<'a>) -> &mut Self {
//         self.shader_src = Some(src);
//         self
//     }

//     pub fn build(&mut self, device: &wgpu::Device) -> Result<MagicCanvas, BuildError> {
//         let display_format = self
//             .display_format
//             .ok_or(BuildError::InvalidDisplayFormat)?;

//         let shader = self
//             .shader_src
//             .take()
//             .ok_or(BuildError::InvalidShader)?;

//         let data = SimulationData {
//             canvas_size: self.canvas_size,
//             clear_color: self.clear_color,
//         };

//         let simulation_data_buffer = device.create_buffer_init(&BufferInitDescriptor {
//             label: self.label,
//             contents: bytemuck::cast_slice(&[data]),
//             usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//         });

//         let simulation_bind_group_layout =
//             device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//                 label: self.label,
//                 entries: &[
//                     wgpu::BindGroupLayoutEntry {
//                         binding: 0,
//                         visibility: wgpu::ShaderStages::FRAGMENT,
//                         count: None,
//                         ty: wgpu::BindingType::Buffer {
//                             ty: wgpu::BufferBindingType::Uniform,
//                             has_dynamic_offset: false,
//                             min_binding_size: None,
//                         },
//                     },
//                 ],
//             });
//         let simulation_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: self.label,
//             layout: &simulation_bind_group_layout,
//             entries: &[wgpu::BindGroupEntry {
//                 binding: 0,
//                 resource: simulation_data_buffer.as_entire_binding(),
//             }],
//         });

//         let shader_module = device.create_shader_module(shader);

//         let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: self.label,
//             bind_group_layouts: &[&simulation_bind_group_layout],
//             push_constant_ranges: &[],
//         });
//         let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: self.label,
//             layout: Some(&pipeline_layout),
//             vertex: wgpu::VertexState {
//                 entry_point: "vs_main",
//                 module: &shader_module,
//                 buffers: &[],
//             },
//             fragment: Some(wgpu::FragmentState {
//                 entry_point: "fs_main",
//                 module: &shader_module,
//                 targets: &[Some(wgpu::ColorTargetState {
//                     format: display_format,
//                     blend: Some(wgpu::BlendState {
//                         color: wgpu::BlendComponent::REPLACE,
//                         alpha: wgpu::BlendComponent::REPLACE,
//                     }),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 })],
//             }),
//             primitive: wgpu::PrimitiveState {
//                 topology: wgpu::PrimitiveTopology::TriangleList,
//                 strip_index_format: None,
//                 front_face: wgpu::FrontFace::Ccw,
//                 cull_mode: Some(wgpu::Face::Back),
//                 polygon_mode: wgpu::PolygonMode::Fill,
//                 unclipped_depth: false,
//                 conservative: false,
//             },
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState {
//                 count: 1,
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//             },
//             multiview: None,
//         });
//         
//         Ok(MagicCanvas {
//             pipeline,
//             data,
//             simulation_data_buffer,
//             simulation_bind_group,
//         })
//     }
// }
