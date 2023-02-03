use anyhow::*;
use std::num::NonZeroU32;

pub struct RenderPipelineBuilder<'a> {
    layout: Option<&'a wgpu::PipelineLayout>,
    vertex_shader: Option<wgpu::ShaderModuleDescriptor<'a>>,
    fragment_shader: Option<wgpu::ShaderModuleDescriptor<'a>>,
    front_face: wgpu::FrontFace,
    cull_mode: Option<wgpu::Face>,
    depth_bias: i32,
    depth_bias_slope_scale: f32,
    depth_bias_clamp: f32,
    primitive_topology: wgpu::PrimitiveTopology,
    color_states: Vec<Option<wgpu::ColorTargetState>>,
    depth_stencil: Option<wgpu::DepthStencilState>,
    index_format: wgpu::IndexFormat,
    vertex_buffers: Vec<wgpu::VertexBufferLayout<'a>>,
    sample_count: u32,
    sample_mask: u64,
    alpha_to_coverage_enabled: bool,
    multiview: Option<NonZeroU32>,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new() -> Self {
        Self {
            layout: None,
            vertex_shader: None,
            fragment_shader: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: Vec::new(),
            depth_stencil: None,
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: Vec::new(),
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
            multiview: None,
        }
    }

    pub fn layout(&mut self, layout: &'a wgpu::PipelineLayout) -> &mut Self {
        self.layout = Some(layout);
        self
    }

    pub fn vertex_shader(&mut self, src: wgpu::ShaderModuleDescriptor<'a>) -> &mut Self {
        self.vertex_shader = Some(src);
        self
    }

    pub fn fragment_shader(&mut self, src: wgpu::ShaderModuleDescriptor<'a>) -> &mut Self {
        self.fragment_shader = Some(src);
        self
    }

    pub fn front_face(&mut self, ff: wgpu::FrontFace) -> &mut Self {
        self.front_face = ff;
        self
    }

    pub fn cull_mode(&mut self, cm: Option<wgpu::Face>) -> &mut Self {
        self.cull_mode = cm;
        self
    }

    pub fn depth_bias(&mut self, db: i32) -> &mut Self {
        self.depth_bias = db;
        self
    }
    
    pub fn depth_bias_slope_scale(&mut self, dbss: f32) -> &mut Self {
        self.depth_bias_slope_scale = dbss;
        self
    }

    pub fn depth_bias_clamp(&mut self, dbc: f32) -> &mut Self {
        self.depth_bias_clamp = dbc;
        self
    }

    pub fn primitive_topology(&mut self, pt: wgpu::PrimitiveTopology) -> &mut Self {
        self.primitive_topology = pt;
        self
    }

    pub fn color_states(&mut self, cs: wgpu::ColorTargetState) -> &mut Self {
        self.color_states.push(Some(cs));
        self
    }

    // Helper [RenderPipelineBuilder::color_state]
    pub fn color_solid(&mut self, format: wgpu::TextureFormat) -> &mut Self {
        self.color_states(wgpu::ColorTargetState {
            format,
            blend: None,
            write_mask: wgpu::ColorWrites::ALL,
        })
    }

    pub fn depth_stencil(&mut self, dss: wgpu::DepthStencilState) -> &mut Self {
        self.depth_stencil = Some(dss);
        self
    }

    // Helper [RenderPipelineBuilder::depth_stencil]
    pub fn depth_no_stencil(
        &mut self,
        format: wgpu::TextureFormat,
        depth_write_enabled: bool,
        depth_compare: wgpu::CompareFunction,
    ) -> &mut Self {
        self.depth_stencil(wgpu::DepthStencilState {
            format, 
            depth_write_enabled,
            depth_compare,
            stencil: Default::default(),
            bias: wgpu::DepthBiasState::default(),
        })
    }

    // Helper [RenderPipelineBuilder::depth_no_stencil]
    pub fn depth_format(&mut self, format: wgpu::TextureFormat) -> &mut Self {
        self.depth_no_stencil(format, true, wgpu::CompareFunction::Less)
    }

    pub fn index_format(&mut self, ifmt: wgpu::IndexFormat) -> &mut Self {
        self.index_format = ifmt;
        self
    }

    pub fn vertex_buffer(&mut self) {}

    pub fn vertex_buffer_desc(&mut self, vb: wgpu::VertexBufferLayout<'a>) -> &mut Self {
        self.vertex_buffers.push(vb);
        self
    }

    pub fn sample_count(&mut self, sc: u32) -> &mut Self {
        self.sample_count = sc;
        self
    }

    pub fn sample_mask(&mut self, sm: u64) -> &mut Self {
        self.sample_mask = sm;
        self
    }

    pub fn alpha_to_coverage_enabled(&mut self, atce: bool) -> &mut Self {
        self.alpha_to_coverage_enabled = atce;
        self
    }

    pub fn multiveiw(&mut self, value: Option<NonZeroU32>) -> &mut Self {
        self.multiview = value;
        self
    }

    pub fn build(&mut self, device: &wgpu::Device) -> Result<wgpu::RenderPipeline> {
        if self.layout.is_none() {
            bail!("No pipeline layout supplied!")
        }
        let layout = self.layout.unwrap();

        if self.vertex_shader.is_none() {
            bail!("No vertex shader suplied!")
        }

        let vs = device.create_shader_module(
            self.vertex_shader
            .take()
            .context("Please include a vertex shader")?,
        );

        let fs = self.fragment_shader
            .take()
            .context("Please include a fragment shader")?;

        let fs = device.create_shader_module(fs);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipelne"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &vs,
                entry_point: "main",
                buffers: &self.vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs,
                entry_point: "main",
                targets: &self.color_states,
            }),
            primitive: wgpu::PrimitiveState {
                topology: self.primitive_topology,
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                strip_index_format: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: self.depth_stencil.clone(),
            multisample: wgpu::MultisampleState {
                count: self.sample_count,
                mask: self.sample_mask,
                alpha_to_coverage_enabled: self.alpha_to_coverage_enabled,
            },
            multiview: self.multiview,
        });

        Ok(pipeline)
    }
}
