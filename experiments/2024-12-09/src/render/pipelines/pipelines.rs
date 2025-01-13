use std::f32::consts::PI;

use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

use super::Pipeline;

pub struct Pipelines {
    pub triangles: Pipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let aspect_ratio = config.width as f32 / config.height as f32;
        let uniforms =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[Uniforms::from_transform(
                    default_transform(aspect_ratio),
                )]),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let triangles = Pipeline::new(
            device,
            config,
            wgpu::include_wgsl!("shaders/triangles.wgsl"),
            &uniforms,
        );

        Self { triangles }
    }
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub transform: Mat4,
    pub transform_for_normals: Mat4,
}

impl Uniforms {
    pub fn from_transform(transform: Mat4) -> Self {
        let transform_for_normals = transform.inverse().transpose();

        Self {
            transform,
            transform_for_normals,
        }
    }
}

fn default_transform(aspect_ratio: f32) -> Mat4 {
    let fov_y_radians = std::f32::consts::PI / 2.;
    let z_near = 0.1;
    let z_far = 10.;

    Mat4::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far)
        * Mat4::from_translation(Vec3::new(0., 0., -2.))
        * Mat4::from_rotation_x(-PI / 4.)
        * Mat4::from_rotation_z(PI / 4.)
}
