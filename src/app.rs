use crate::world::World;
use crate::{
    camera_controller::CameraController, frame_timer::FrameTimer, gui::GuiParams,
    light_controller::LightController, renderer::Renderer,
};
use std::{cell::RefCell, rc::Rc, time::Duration};
use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

pub enum WindowEventHandlingResult {
    RequestExit,
    Handled,
}

pub struct App {
    pub renderer: Renderer,
    pub camera_controller: CameraController,
    pub light_controller: LightController,
    pub frame_timer: FrameTimer,
    _gui_params: Rc<RefCell<GuiParams>>,
    world: World,
}

impl App {
    pub async fn new(window: &Window) -> Self {
        let gui_params = Rc::new(RefCell::new(GuiParams::new()));

        let renderer = Renderer::new(window, gui_params.clone()).await;
        let camera_controller = CameraController::new(&renderer, gui_params.clone());
        let light_controller = LightController::new(&renderer.device);
        let world: World = World::new(&renderer).await;

        let frame_timer = FrameTimer::new();

        Self {
            renderer,
            camera_controller,
            light_controller,
            frame_timer,
            _gui_params: gui_params,
            world,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 && new_size != self.renderer.size {
            self.renderer.resize(new_size);
            self.camera_controller
                .resize(new_size.width as f32 / new_size.height as f32)
        }
    }

    pub fn handle_event<'a, T>(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::Event<T>,
    ) {
        self.renderer.handle_event(window, event);
    }

    pub fn handle_device_event(
        &mut self,
        window: &Window,
        device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        self.renderer.handle_event(
            window,
            &winit::event::Event::DeviceEvent::<()> {
                device_id,
                event: event.clone(),
            },
        );

        self.camera_controller.process_device_events(event);
    }

    pub fn handle_window_event(&mut self, event: WindowEvent) -> WindowEventHandlingResult {
        match event {
            WindowEvent::CloseRequested => return WindowEventHandlingResult::RequestExit,

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed
                    && event.physical_key == PhysicalKey::Code(KeyCode::KeyF)
                {
                    self.renderer.toggle_should_draw_gui();
                }
            }

            WindowEvent::Resized(new_size) => {
                self.resize(new_size);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                // self.resize(inner_size_writer); // TODO Handle scale factor change
            }
            WindowEvent::MouseInput { state, button, .. } if button == MouseButton::Right => {
                self.camera_controller
                    .set_is_movement_enabled(state == ElementState::Pressed);
            }
            _ => {}
        };

        WindowEventHandlingResult::Handled
    }

    pub fn request_redraw(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), wgpu::SurfaceError> {
        let delta = self.frame_timer.get_delta_and_reset_timer();
        self.update(delta);
        self.renderer.render(
            window,
            &self.camera_controller,
            &self.light_controller,
            &self.world,
            delta,
        )
    }

    pub fn update(&mut self, delta: Duration) {
        self.camera_controller.update(delta, &self.renderer.queue);

        self.light_controller.update(delta, &self.renderer.queue);
    }
}
