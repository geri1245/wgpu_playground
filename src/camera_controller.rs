use glam::{Mat4, Vec4};
use std::time;
use wgpu::util::DeviceExt;
use winit::event::DeviceEvent;

use crate::camera::Camera;

/// Contains the rendering-related concepts of the camera
pub struct CameraController {
    camera: Camera,
    pub binding_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    is_movement_enabled: bool,
}

impl CameraController {
    pub fn new(
        aspect_ratio: f32,
        render_device: &wgpu::Device,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> CameraController {
        let camera = Camera::new(aspect_ratio);

        let binding_buffer = render_device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[Self::get_raw(&camera)]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: binding_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            camera,
            binding_buffer,
            bind_group,
            is_movement_enabled: false,
        }
    }

    pub fn resize(&mut self, aspect: f32) {
        self.camera.resize(aspect);
    }

    pub fn update(&mut self, delta_time: time::Duration, render_queue: &wgpu::Queue) {
        self.camera.update(delta_time);

        render_queue.write_buffer(
            &self.binding_buffer,
            0,
            bytemuck::cast_slice(&[self.to_raw()]),
        );
    }

    pub fn set_is_movement_enabled(&mut self, value: bool) {
        self.is_movement_enabled = value;

        if !self.is_movement_enabled {
            self.camera.stop_movement();
        }
    }

    pub fn process_device_events(&mut self, event: DeviceEvent) {
        if self.is_movement_enabled {
            self.camera.process_device_events(event);
        }
    }

    pub fn to_raw(&self) -> CameraRaw {
        Self::get_raw(&self.camera)
    }

    fn get_raw(camera: &Camera) -> CameraRaw {
        let view = Mat4::look_at_rh(camera.position, camera.get_target(), camera.up);
        let proj = Mat4::perspective_rh(camera.fovy, camera.aspect, camera.znear, camera.zfar);

        let pos = camera.get_position();
        let pos_homogenous = Vec4::new(pos.x, pos.y, pos.z, 1.0_f32);

        CameraRaw {
            view_proj: (proj * view).to_cols_array_2d(),
            camera_pos: pos_homogenous.to_array(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraRaw {
    view_proj: [[f32; 4]; 4],
    camera_pos: [f32; 4],
}
