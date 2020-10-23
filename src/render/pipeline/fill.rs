use super::Draw;

pub(crate) struct Fill {}

impl Draw for Fill {
    fn draw(&self, render_pass: &mut wgpu::RenderPass) {}
}
