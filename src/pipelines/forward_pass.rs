use crate::{bind_group_layout_descriptors, buffer_content::BufferContent, texture, vertex};

pub struct ForwardPass {
    render_pipeline: wgpu::RenderPipeline,
}

impl ForwardPass {
    pub fn new(device: &wgpu::Device, color_format: wgpu::TextureFormat) -> Self {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Light Pipeline Layout"),
            bind_group_layouts: &[
                &device.create_bind_group_layout(&bind_group_layout_descriptors::LIGHT),
                &device.create_bind_group_layout(&bind_group_layout_descriptors::CAMERA),
            ],
            push_constant_ranges: &[],
        });
        let light_shader_desc = wgpu::ShaderModuleDescriptor {
            label: Some("Light Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/light.wgsl").into()),
        };
        let light_shader = device.create_shader_module(light_shader_desc);

        let render_pipeline = super::render_pipeline::new(
            Some("Light render pipeline"),
            &device,
            &layout,
            color_format,
            Some(texture::Texture::DEPTH_FORMAT),
            &[vertex::VertexRaw::buffer_layout()],
            &light_shader,
        );

        Self { render_pipeline }
    }
}
