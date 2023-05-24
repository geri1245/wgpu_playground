use std::{cell::RefCell, iter, rc::Rc, time::Instant};

use egui::FontDefinitions;
use egui_wgpu::{renderer::ScreenDescriptor, Renderer};

use crate::{
    color,
    egui_winit_platform::{Platform, PlatformDescriptor},
    texture,
};

#[derive(Default)]
pub struct GuiParams {
    pub clear_color: [f32; 4],
    pub fov_x: f32,
    pub fov_y: f32,
}

impl GuiParams {
    pub fn new() -> Self {
        GuiParams {
            clear_color: color::wgpu_color_to_f32_array_rgba(crate::CLEAR_COLOR),
            fov_x: 90.0,
            fov_y: 45.0,
        }
    }
}

pub struct Gui {
    renderer: Renderer,
    platform: Platform,
    start_time: Instant,
    is_open: bool,
    width: u32,
    height: u32,
}

impl Gui {
    pub fn new(
        window: &winit::window::Window,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let renderer =
            egui_wgpu::Renderer::new(device, format, Some(texture::Texture::DEPTH_FORMAT), 1);

        Gui {
            platform,
            renderer,
            start_time: Instant::now(),
            is_open: true,
            width,
            height,
        }
    }

    pub fn render<'a>(
        &'a mut self,
        window: &winit::window::Window,
        render_pass: &mut wgpu::RenderPass<'a>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        params: Rc<RefCell<GuiParams>>,
    ) {
        self.platform
            .update_time(Instant::now().duration_since(self.start_time).as_secs_f64());

        // Begin to draw the UI frame.
        self.platform.begin_frame();

        let egui_window = egui::Window::new("Window with Panels")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(&mut self.is_open);
        let res = egui_window.show(&self.platform.context(), |ui| {
            ui.add(egui::Slider::new(&mut params.borrow_mut().fov_x, 40.0..=50.0).text("age"));
        });

        // Draw the demo application.
        // self.demo.ui(&self.platform.context());

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let full_output = self.platform.end_frame(Some(&window));
        let paint_jobs = self.platform.context().tessellate(full_output.shapes);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.width, self.height],
            pixels_per_point: window.scale_factor() as f32,
        };
        self.renderer.update_buffers(
            &device,
            &queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        // Record all render passes.
        self.renderer
            .render(render_pass, &paint_jobs, &screen_descriptor);

        // Submit the commands.
        queue.submit(iter::once(encoder.finish()));
    }

    pub fn handle_event<'a, T>(&mut self, event: &winit::event::Event<T>) {
        self.platform.handle_event(event);
    }
}
