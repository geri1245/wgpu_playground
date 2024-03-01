use wgpu::{BindGroup, ComputePass, Device};

use crate::pipelines::PostProcessRP;

pub struct PostProcessManager {
    pub pipeline: PostProcessRP,
}

impl PostProcessManager {
    pub async fn new(device: &Device) -> Self {
        let pipeline = PostProcessRP::new(device).await.unwrap();

        Self { pipeline }
    }

    pub fn render<'a>(
        &'a self,
        compute_pass: &mut ComputePass<'a>,
        compute_pass_texture_bind_groups: &'a BindGroup,
        width: u32,
        height: u32,
    ) {
        self.pipeline.run_copmute_pass(
            compute_pass,
            compute_pass_texture_bind_groups,
            width,
            height,
        );
    }
}
