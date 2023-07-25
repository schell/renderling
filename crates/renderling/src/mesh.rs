//! Sometimes you just need a mesh.
use snafu::prelude::*;
use wgpu::util::DeviceExt;

/// A vertex buffer.
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    vertex_buffer_len: usize,
    vertex_indices: Option<(wgpu::Buffer, usize)>,
}

impl Mesh {
    pub fn new<V: bytemuck::Pod>(
        device: &wgpu::Device,
        label: Option<&str>,
        vertices: impl IntoIterator<Item = V>,
        may_indices: Option<impl IntoIterator<Item = u16>>,
    ) -> Self {
        let vertices = vertices.into_iter().collect::<Vec<_>>();
        let vertex_buffer_len = vertices.len();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let vertex_indices = if let Some(indices) = may_indices {
            let indices = indices.into_iter().collect::<Vec<_>>();
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label,
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            Some((index_buffer, indices.len()))
        } else {
            None
        };
        Self {
            vertex_buffer,
            vertex_buffer_len,
            vertex_indices,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_buffer_len
    }

    pub fn index_count(&self) -> usize {
        self.vertex_indices.map(|(_, len)| len).unwrap_or_default()
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_indices.as_ref().map(|(b, _)| b)
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        match &self.vertex_indices {
            Some((index_buffer, len)) => {
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..*len as u32, 0, 0..1);
            }
            None => {
                render_pass.draw(0..self.vertex_buffer_len as u32, 0..1);
            }
        }
    }

    pub fn update<V: bytemuck::Pod>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        vertices: impl IntoIterator<Item = V>,
        may_indices: Option<impl IntoIterator<Item = u16>>,
    ) {
        log::trace!("updating mesh vertices");
        let vertices = vertices.into_iter().collect::<Vec<_>>();
        self.vertex_buffer_len = vertices.len();
        self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        self.vertex_indices = if let Some(indices) = may_indices {
            let indices = indices.into_iter().collect::<Vec<_>>();
            log::trace!("updating mesh indices");
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label,
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            Some((buffer, indices.len()))
        } else {
            None
        };
    }
}
