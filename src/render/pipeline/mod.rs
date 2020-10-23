mod fill;

pub(crate) trait Draw {
    fn draw(&self, render_pass: &mut wgpu::RenderPass);
}
