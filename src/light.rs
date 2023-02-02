use cgmath::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LightData {
    pub position: Vector4<f32>,
    pub color: Vector4<f32>,
}

unsafe impl bytemuck::Pod for LightData {}
unsafe impl bytemuck::Zeroable for LightData {}

pub struct LightUniform {
    data: LightData,
    buffer: wgpu::Buffer,
}

impl LightUniform {
    pub fn new(device: &wgpu::Device, position: Vector3<f32>, color: Vector3<f32>) -> Self {
        let data = LightData {
            position: Vector4::new(position.x, position.y, position.z, 1.0),
            color: Vector4::new(color.x, color.y, color.z, 1.0),
        };
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Light Buffer"),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            contents: bytemuck::cast_slice(&[data]),
        });

        Self { data, buffer }
    }
}

pub struct LightBinding {   
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}
