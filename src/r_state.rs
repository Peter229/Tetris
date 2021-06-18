#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::texture;
use crate::r_render_pipeline;
use crate::camera;
use crate::uniform;
use crate::r_backend;
use wgpu::util::DeviceExt;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

pub struct State {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    render_array_pipeline: wgpu::RenderPipeline,
    pub renderer: r_backend::Renderer,
    depth_texture: texture::Texture,
    pub camera: camera::Camera,
    projection: camera::Projection,
    uniforms: uniform::Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl State {

    pub async fn new(window: &Window) -> Self {

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        //Fifo or Immediate (vsync on and off)
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let depth_texture = texture::Texture::create_depth_texture(&device, &sc_desc, "depth_texture");

        let renderer = r_backend::Renderer::new(&device);

        let render_pipeline = r_render_pipeline::render_pipeline(&device, &sc_desc, wgpu::include_spirv!("shader.vert.spv"), wgpu::include_spirv!("shader.frag.spv"), r_backend::Vertex::desc(), texture::Texture::get_bind_group_layout(&device));

        let render_array_pipeline = r_render_pipeline::render_pipeline(&device, &sc_desc, wgpu::include_spirv!("array_shader.vert.spv"), wgpu::include_spirv!("array_shader.frag.spv"), r_backend::ArrayVertex::desc(), texture::Texture::get_array_bind_group_layout(&device));

        let camera = camera::Camera::new();

        let projection = camera::Projection::new(0.0, 16.0, 0.0, 9.0, 0.0, 10.0);

        let mut uniforms = uniform::Uniforms::new();
        uniforms.update_view_ortho(&camera, &projection);
    
        let (uniform_buffer, uniform_bind_group) = uniforms.get_buffers(&device);

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            render_array_pipeline,
            renderer,
            depth_texture,
            camera,
            projection,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {

        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        if self.sc_desc.width != 0 && self.sc_desc.height != 0 {
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.sc_desc, "depth_texture");
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {

        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                //self.camera.process_keyboard(*keycode, *state);
                true
            }
            WindowEvent::CursorMoved  { position, .. } => {
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {

        self.camera.update_camera();
        self.uniforms.update_view_ortho(&self.camera, &self.projection);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {

        self.renderer.update_buffers(&self.device);

        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            for texture_vertices in self.renderer.render_info.values_mut() {
                render_pass.set_vertex_buffer(0, texture_vertices.buf.slice(..));
                render_pass.set_bind_group(1, &texture_vertices.bind_group, &[]);
                render_pass.draw(0..(texture_vertices.verts.len() as u32), 0..1);
            }
            
            render_pass.set_pipeline(&self.render_array_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            for texture_vertices in self.renderer.render_array_info.values_mut() {
                render_pass.set_vertex_buffer(0, texture_vertices.buf.slice(..));
                render_pass.set_bind_group(1, &texture_vertices.bind_group, &[]);
                render_pass.draw(0..(texture_vertices.verts.len() as u32), 0..1);
            }

        }
        self.queue.submit(std::iter::once(encoder.finish()));

        self.renderer.clear_verts();

        Ok(())
    }
}