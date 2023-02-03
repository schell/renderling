//! Lighting primitives shared by shaders and renderlings.

use wgpu::util::DeviceExt;
#[repr(C)]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLight {
    /// The position of the light in world space.
    pub position: [f32; 3],
    /// The number of all point lights in the scene.
    ///
    /// Because uniform structs need to be 16-byte spaced, we need a float
    /// or uint here, so we use this parameter to tell the shader if it should
    /// continue processing the lights after this one in our array.
    pub num_lights: u32,
    /// Constant, linear and quadratic term of attenuation.
    pub attenuation: [f32; 3],
    /// Just padding to keep everything 16-byte aligned.
    pub _padding: u32,
    /// Ambient color value.
    pub ambient_color: [f32; 4],
    /// Diffuse color value.
    pub diffuse_color: [f32; 4],
    /// Specular color value.
    pub specular_color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpotLight {
    /// The position of the light in world space.
    pub position: [f32; 3],
    /// The number of all spot lights in the scene.
    ///
    /// Because uniform structs need to be 16-byte spaced, we need a float
    /// or uint here, so we use this parameter to tell the shader if it should
    /// continue processing the lights after this one in our array.
    pub num_lights: u32,
    /// The direction the light is pointing in.
    pub direction: [f32; 3],
    /// Inner angular cutoff.
    pub inner_cutoff: f32,
    /// Constant, linear and quadratic terms of attenuation.
    pub attenuation: [f32; 3],
    /// Outer angular cutoff.
    pub outer_cutoff: f32,
    /// Ambient color value.
    pub ambient_color: [f32; 4],
    /// Diffuse color value.
    pub diffuse_color: [f32; 4],
    /// Specular color value.
    pub specular_color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DirectionalLight {
    /// The direction the light is pointing in.
    pub direction: [f32; 3],
    /// The number of all spot lights in the scene.
    ///
    /// Because uniform structs need to be 16-byte spaced, we need a float
    /// or uint here, so we use this parameter to tell the shader if it should
    /// continue processing the lights after this one in our array.
    pub num_lights: u32,
    /// Ambient color value.
    pub ambient_color: [f32; 4],
    /// Diffuse color value.
    pub diffuse_color: [f32; 4],
    /// Specular color value.
    pub specular_color: [f32; 4],
}

pub const MAX_POINT_LIGHTS: usize = 64;
pub const MAX_SPOT_LIGHTS: usize = 32;
pub const MAX_DIRECTIONAL_LIGHTS: usize = 8;

/// The shader only supports a limited number of lights and we need to set some
/// parameters to control the lighting loop in the shader.
trait Light: Default {
    fn set_num_lights(&mut self, num_lights: u32);

    fn process_lights<L: Light>(lights: &mut Vec<L>, max_lights: usize, fill: bool) {
        let len = lights.len();
        let resize_len = if fill { max_lights } else { len.max(1) };
        lights.resize_with(resize_len, L::default);
        lights
            .iter_mut()
            .for_each(|light| light.set_num_lights(len as u32));
    }
}

impl Light for PointLight {
    fn set_num_lights(&mut self, num_lights: u32) {
        self.num_lights = num_lights;
    }
}

impl Light for SpotLight {
    fn set_num_lights(&mut self, num_lights: u32) {
        self.num_lights = num_lights;
    }
}

impl Light for DirectionalLight {
    fn set_num_lights(&mut self, num_lights: u32) {
        self.num_lights = num_lights;
    }
}

pub fn lights_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
        label: Some("forward lights bindgroup layout"),
    })
}

/// Helper struct for managing the light uniforms.
pub struct LightsUniform {
    pub point_buffer: wgpu::Buffer,
    pub spot_buffer: wgpu::Buffer,
    pub directional_buffer: wgpu::Buffer,
    pub bindgroup: wgpu::BindGroup,
}

impl LightsUniform {
    pub fn new(
        device: &wgpu::Device,
        mut point_lights: Vec<PointLight>,
        mut spot_lights: Vec<SpotLight>,
        mut dir_lights: Vec<DirectionalLight>,
    ) -> LightsUniform {
        PointLight::process_lights(&mut point_lights, MAX_POINT_LIGHTS, true);
        let point_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("point-lights-buffer"),
            contents: bytemuck::cast_slice(point_lights.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        SpotLight::process_lights(&mut spot_lights, MAX_SPOT_LIGHTS, true);
        let spot_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("spot-lights-buffer"),
            contents: bytemuck::cast_slice(&spot_lights),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        DirectionalLight::process_lights(&mut dir_lights, MAX_DIRECTIONAL_LIGHTS, true);
        let directional_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("directional-lights-buffer"),
            contents: bytemuck::cast_slice(&dir_lights),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &lights_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: point_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: spot_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: directional_buffer.as_entire_binding(),
                },
            ],
            label: Some("forward light bind group"),
        });

        LightsUniform {
            directional_buffer,
            point_buffer,
            spot_buffer,
            bindgroup,
        }
    }

    pub fn update_point_lights(&self, queue: &wgpu::Queue, mut lights: Vec<PointLight>) {
        PointLight::process_lights(&mut lights, MAX_POINT_LIGHTS, false);
        queue.write_buffer(&self.point_buffer, 0, bytemuck::cast_slice(&lights));
    }

    pub fn update_spot_lights(&self, queue: &wgpu::Queue, mut lights: Vec<SpotLight>) {
        SpotLight::process_lights(&mut lights, MAX_SPOT_LIGHTS, false);
        queue.write_buffer(&self.spot_buffer, 0, bytemuck::cast_slice(&lights));
    }

    pub fn update_directional_lights(
        &self,
        queue: &wgpu::Queue,
        mut lights: Vec<DirectionalLight>,
    ) {
        DirectionalLight::process_lights(&mut lights, MAX_DIRECTIONAL_LIGHTS, false);
        queue.write_buffer(&self.directional_buffer, 0, bytemuck::cast_slice(&lights));
    }
}
