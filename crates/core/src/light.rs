//! Lighting primitives shared by shaders and renderlings.
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
