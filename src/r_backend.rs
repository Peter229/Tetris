use wgpu::util::DeviceExt;
use std::collections::HashMap;

use crate::texture;
use glob::glob;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrayVertex {
    pos: [f32; 3],
    uv: [f32; 3],
    colour: [f32; 4],
}

impl ArrayVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ArrayVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct TextureVerticesInfo {
    pub verts: Vec<Vertex>,
    pub buf: wgpu::Buffer,
    pub texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
    pub num_sprites_col: i32,
    pub sprite_size: f32,
}

impl TextureVerticesInfo {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, path: &std::path::Path) -> TextureVerticesInfo {

        let verts: Vec<Vertex> = Vec::new();

        let buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let texture = texture::Texture::load(device, queue, path).unwrap();

        let texture_bind_group_layout = texture::Texture::get_bind_group_layout(device);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        TextureVerticesInfo { verts, buf, texture, bind_group, num_sprites_col: 1, sprite_size: 1.0 }
    }

    pub fn new_sprite_sheet(device: &wgpu::Device, queue: &wgpu::Queue, path: &std::path::Path, num_sprites_col: i32) -> TextureVerticesInfo {

        let mut verts: Vec<Vertex> = Vec::new();

        let buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let texture = texture::Texture::load(device, queue, path).unwrap();

        let texture_bind_group_layout = texture::Texture::get_bind_group_layout(device);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        let sprite_size = 1.0 / (num_sprites_col as f32);

        TextureVerticesInfo { verts, buf, texture, bind_group, num_sprites_col, sprite_size }
    }

    pub fn update_buffer(&mut self, device: &wgpu::Device) {

        self.buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.verts),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );
    }
}

pub struct ArrayTextureVerticesInfo {
    pub verts: Vec<ArrayVertex>,
    pub buf: wgpu::Buffer,
    pub texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
    pub num_sprites: i32,
}

impl ArrayTextureVerticesInfo {

    pub fn new_sprite_array(device: &wgpu::Device, queue: &wgpu::Queue, path: &str) -> ArrayTextureVerticesInfo {

        let mut verts: Vec<ArrayVertex> = Vec::new();

        let buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );
        
        let texture = texture::Texture::from_array_to_image(device, queue, path).unwrap();

        let texture_bind_group_layout = texture::Texture::get_array_bind_group_layout(device);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        ArrayTextureVerticesInfo { verts, buf, texture, bind_group, num_sprites: 1 }
    }

    pub fn update_buffer(&mut self, device: &wgpu::Device) {

        self.buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.verts),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );
    }
}

pub struct Renderer {

    pub render_info: HashMap<String, TextureVerticesInfo>,
    pub render_array_info: HashMap<String, ArrayTextureVerticesInfo>,
}

impl Renderer {

    pub fn new(device: &wgpu::Device) -> Renderer {

        let render_info = HashMap::new();
        let render_array_info = HashMap::new();

        Renderer { render_info, render_array_info }
    }

    pub fn load_sprite(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, path: &std::path::Path, name: String) {

        match self.render_info.get(&name) {
            Some(a) => {
                println!("Already loaded texture: {}", name);
            },
            None => {
                println!("Loading texture: {}", name);
                self.render_info.insert(name, TextureVerticesInfo::new(device, queue, path));
            }
        }
    }

    pub fn load_sprite_sheet(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, path: &std::path::Path, name: String, num_sprites_col: i32) {

        match self.render_info.get(&name) {
            Some(a) => {
                println!("Already loaded texture: {}", name);
            },
            None => {
                println!("Loading texture: {}", name);
                self.render_info.insert(name, TextureVerticesInfo::new_sprite_sheet(device, queue, path, num_sprites_col));
            }
        }
    }

    pub fn load_sprite_array(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, path: &str, name: String) {

        //ArrayTextureVerticesInfo::new_sprite_array(device, queue, path);

        match self.render_array_info.get(&name) {
            Some(a) => {
                println!("Already loaded texture: {}", name);
            },
            None => {
                println!("Loading texture: {}", name);
                self.render_array_info.insert(name, ArrayTextureVerticesInfo::new_sprite_array(device, queue, path));
            }
        }
    }

    pub fn render_sprite(&mut self, pos: [f32; 2], size: [f32; 2], depth: f32, name: String) {

        match self.render_info.get_mut(&name) {
            Some(texture_vertices) => {

                texture_vertices.verts.push(Vertex { pos: [pos[0],           pos[1],           depth], uv: [0.0, 1.0] });
                texture_vertices.verts.push(Vertex { pos: [pos[0],           pos[1] + size[1], depth], uv: [0.0, 0.0] });
                texture_vertices.verts.push(Vertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [1.0, 0.0] });

                texture_vertices.verts.push(Vertex { pos: [pos[0],           pos[1],           depth], uv: [0.0, 1.0] });
                texture_vertices.verts.push(Vertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [1.0, 0.0] });
                texture_vertices.verts.push(Vertex { pos: [pos[0] + size[0], pos[1],           depth], uv: [1.0, 1.0] });
            },
            None => {
                println!("Texture not found {}", name);
            }
        }
    }
    
    pub fn render_sprite_part(&mut self, pos: [f32; 2], size: [f32; 2], depth: f32, name: String, image_index: i32) {

        match self.render_info.get_mut(&name) {
            Some(texture_vertices) => {

                let y = ((image_index / texture_vertices.num_sprites_col) as f32) / (texture_vertices.num_sprites_col as f32);
                let x = ((image_index % texture_vertices.num_sprites_col) as f32) / (texture_vertices.num_sprites_col as f32);

                texture_vertices.verts.push(Vertex { pos: [pos[0],           pos[1],           depth], uv: [x,                                y + texture_vertices.sprite_size] });
                texture_vertices.verts.push(Vertex { pos: [pos[0],           pos[1] + size[1], depth], uv: [x,                                y] });
                texture_vertices.verts.push(Vertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [x + texture_vertices.sprite_size, y] });

                texture_vertices.verts.push(Vertex { pos: [pos[0],           pos[1],           depth], uv: [x,                                y + texture_vertices.sprite_size] });
                texture_vertices.verts.push(Vertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [x + texture_vertices.sprite_size, y] });
                texture_vertices.verts.push(Vertex { pos: [pos[0] + size[0], pos[1],           depth], uv: [x + texture_vertices.sprite_size, y + texture_vertices.sprite_size] });
            },
            None => {
                println!("Texture not found {}", name);
            }
        }
    }

    pub fn render_sprite_array(&mut self, pos: [f32; 2], size: [f32; 2], depth: f32, name: String, image_index: i32) {

        match self.render_array_info.get_mut(&name) {
            Some(texture_vertices) => {

                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1],           depth], uv: [0.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1] + size[1], depth], uv: [0.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [1.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });

                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1],           depth], uv: [0.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [1.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1],           depth], uv: [1.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
            },
            None => {
                println!("Texture not found {}", name);
            }
        }
    }

    pub fn render_sprite_transparent(&mut self, pos: [f32; 2], size: [f32; 2], depth: f32, name: String, image_index: i32, alpha: f32) {

        match self.render_array_info.get_mut(&name) {
            Some(texture_vertices) => {

                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1],           depth], uv: [0.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, alpha] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1] + size[1], depth], uv: [0.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, alpha] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [1.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, alpha] });

                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1],           depth], uv: [0.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, alpha] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [1.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, alpha] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1],           depth], uv: [1.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, alpha] });
            },
            None => {
                println!("Texture not found {}", name);
            }
        }
    }

    pub fn render_sprite_array_flip(&mut self, pos: [f32; 2], size: [f32; 2], depth: f32, name: String, image_index: i32) {

        match self.render_array_info.get_mut(&name) {
            Some(texture_vertices) => {

                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1],           depth], uv: [1.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1] + size[1], depth], uv: [1.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [0.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });

                texture_vertices.verts.push(ArrayVertex { pos: [pos[0],           pos[1],           depth], uv: [1.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1] + size[1], depth], uv: [0.0, 0.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
                texture_vertices.verts.push(ArrayVertex { pos: [pos[0] + size[0], pos[1],           depth], uv: [0.0, 1.0, image_index as f32], colour: [1.0, 1.0, 1.0, 1.0] });
            },
            None => {
                println!("Texture not found {}", name);
            }
        }
    }

    pub fn update_buffers(&mut self, device: &wgpu::Device) {

        for texture_vertices in self.render_info.values_mut() {
            texture_vertices.update_buffer(device);
        }

        for texture_vertices in self.render_array_info.values_mut() {
            texture_vertices.update_buffer(device);
        }
    }

    pub fn clear_verts(&mut self) {

        for texture_vertices in self.render_info.values_mut() {
            texture_vertices.verts.clear();
        }

        for texture_vertices in self.render_array_info.values_mut() {
            texture_vertices.verts.clear();
        }
    }
}