#![allow(unused_variables)]
#![allow(unused_imports)]

use cgmath::*;
use winit::event::*;
use winit::dpi::PhysicalPosition;
use std::time::Duration;
use std::f32::consts::FRAC_PI_2;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub view: Matrix4<f32>,
}

impl Camera {
    
    pub fn new() -> Self {
        Self {
            position: Point3::new(-9.0, -0.4, 0.0),
            view: Matrix4::look_at_dir(
                Point3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::unit_y(),
            ),
        }
    }

    pub fn update_camera(&mut self) {
        self.view = Matrix4::look_to_rh(self.position, Vector3::new(0.0, 0.0, -1.0), Vector3::unit_y());
    }
}

pub struct Projection {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
}

impl Projection {
    
    pub fn new(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
        }
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * ortho(self.left, self.right, self.bottom, self.top, self.near, self.far)
    }
}