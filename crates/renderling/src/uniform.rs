//! Uniforms share data across CPU/GPU.

use std::ops::{Deref, DerefMut};

use wgpu::util::DeviceExt;

/// Wraps `T`, allowing it to live on the CPU and the GPU as a shader uniform.
///
/// Provides a [`wgpu::Buffer`] and [`wgpu::BindGroup`].
///
/// Access the underlying `T` using [`Deref::deref`] and
/// [`DerefMut::deref_mut`]. When accessed with [`DerefMut::deref_mut`] the will
/// be marked "dirty" and the new value will be uploaded to the GPU the next
/// time [`SharedGpuData::update`] is called.
pub struct Uniform<T> {
    inner: T,
    inner_updated: bool,
    bindgroup: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl<T> Uniform<T>
where
    T: Clone + bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new_and_layout(
        device: &wgpu::Device,
        inner: T,
        mut usage: wgpu::BufferUsages,
        visibility: wgpu::ShaderStages,
    ) -> (Self, wgpu::BindGroupLayout) {
        usage |=
            // We use this buffer as a uniform
            wgpu::BufferUsages::UNIFORM
            // We copy data to this buffer on update
            | wgpu::BufferUsages::COPY_DST;

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(std::any::type_name::<Self>()),
            contents: bytemuck::cast_slice(&[inner.clone()]),
            usage,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(std::any::type_name::<Self>()),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let desc = wgpu::BindGroupDescriptor {
            label: Some(std::any::type_name::<Self>()),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
            }],
        };
        let bindgroup = device.create_bind_group(&desc);
        (
            Self {
                inner,
                inner_updated: false,
                bindgroup,
                buffer,
            },
            layout,
        )
    }

    pub fn new(
        device: &wgpu::Device,
        inner: T,
        usage: wgpu::BufferUsages,
        visibility: wgpu::ShaderStages,
    ) -> Self {
        Self::new_and_layout(device, inner, usage, visibility).0
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        if self.inner_updated {
            log::trace!("updating {}", std::any::type_name::<T>());
            self.inner_updated = false;
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.inner.clone()]));
        }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn bindgroup(&self) -> &wgpu::BindGroup {
        &self.bindgroup
    }
}

impl<T> Deref for Uniform<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Uniform<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_updated = true;
        &mut self.inner
    }
}
