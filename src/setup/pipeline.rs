use anyhow::*;
use std::num::NonZeroU32;

pub struct Pipeline<'a> {
    push_constant_ranges: Vec<wgpu::PushConstantRange>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,

    shader: Option<wgpu::ShaderModuleDescriptor<'a>>,
    front_face: wgpu::FrontFace,
    cull_mode: Option<wgpu::Face>,
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

impl<'a> Pipeline<'a> {
    pub fn new() -> Self {
        Self {
            push_constant_ranges: Vec::new(),
            bind_group_layouts: Vec::new(),
            shader: None,

            vertex_buffers: Vec::new(),

            color_states: Vec::new(),

            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,

            depth_stencil: None,

            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            index_format: wgpu::IndexFormat::Uint32,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
            multiview: None,
        }
    }

    pub fn bind_group_layouts(&mut self, bgl: &'a wgpu::BindGroupLayout) -> &mut Self {
        self.bind_group_layouts.push(bgl);
        self
    }

    pub fn push_constant_ranges(&mut self, pcr: wgpu::PushConstantRange) -> &mut Self {
        self.push_constant_ranges.push(pcr);
        self
    }

    pub fn shader(&mut self, src: wgpu::ShaderModuleDescriptor<'a>) -> &mut Self {
        self.shader = Some(src);
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

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &self.bind_group_layouts,
            push_constant_ranges: &self.push_constant_ranges,
        });

        if self.shader.is_none() {
            bail!("No shader suplied!")
        }

        let shader = device.create_shader_module(
            self.shader
            .take()
            .context("Please include a vertex shader")?,
        );

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipelne"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &self.vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
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
