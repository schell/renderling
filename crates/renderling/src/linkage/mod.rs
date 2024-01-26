//! Provides convenient wrappers around renderling shader linkage.
mod shaders;
pub use shaders::*;

pub struct ShaderLinkage {
    pub module: wgpu::ShaderModule,
    pub entry_point: &'static str,
}

pub fn slab_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let visibility =
        wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE;
    let slab = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    };
    let entries = vec![slab];
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("slab"),
        entries: &entries,
    })
}

pub fn slab_bindgroup(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
    bindgroup_layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
    let label = Some("slab");
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bindgroup_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
        }],
    })
}

pub fn atlas_and_skybox_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    fn image2d_entry(binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
        let img = wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let sampler = wgpu::BindGroupLayoutEntry {
            binding: binding + 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        };
        (img, sampler)
    }

    fn cubemap_entry(binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
        let img = wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::Cube,
                multisampled: false,
            },
            count: None,
        };
        let sampler = wgpu::BindGroupLayoutEntry {
            binding: binding + 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        };
        (img, sampler)
    }

    let (atlas, atlas_sampler) = image2d_entry(0);
    let (irradiance, irradiance_sampler) = cubemap_entry(2);
    let (prefilter, prefilter_sampler) = cubemap_entry(4);
    let (brdf, brdf_sampler) = image2d_entry(6);
    let (environment, environment_sampler) = cubemap_entry(8);
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("atlas and skybox"),
        entries: &[
            atlas,
            atlas_sampler,
            irradiance,
            irradiance_sampler,
            prefilter,
            prefilter_sampler,
            brdf,
            brdf_sampler,
            environment,
            environment_sampler,
        ],
    })
}

pub fn atlas_and_skybox_bindgroup(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    atlas: &crate::Atlas,
    skybox: &crate::Skybox,
) -> wgpu::BindGroup {
    let label = Some("atlas and skybox");
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&atlas.texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&atlas.texture.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&skybox.irradiance_cubemap.view),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Sampler(&skybox.irradiance_cubemap.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::TextureView(
                    &skybox.prefiltered_environment_cubemap.view,
                ),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::Sampler(
                    &skybox.prefiltered_environment_cubemap.sampler,
                ),
            },
            wgpu::BindGroupEntry {
                binding: 6,
                resource: wgpu::BindingResource::TextureView(&skybox.brdf_lut.view),
            },
            wgpu::BindGroupEntry {
                binding: 7,
                resource: wgpu::BindingResource::Sampler(&skybox.brdf_lut.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 8,
                resource: wgpu::BindingResource::TextureView(&skybox.environment_cubemap.view),
            },
            wgpu::BindGroupEntry {
                binding: 9,
                resource: wgpu::BindingResource::Sampler(&skybox.environment_cubemap.sampler),
            },
        ],
    })
}

#[cfg(test)]
mod test {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    enum SrcError {
        #[snafu(display("{source}"))]
        Read { source: std::io::Error },
        #[snafu(display("{source}"))]
        ParseSpv { source: naga::front::spv::Error },
        #[snafu(display("{source}"))]
        Validate {
            source: naga::WithSpan<naga::valid::ValidationError>,
        },
        #[snafu(display("{source}"))]
        Write { source: naga::back::wgsl::Error },
        #[snafu(display("{source}:\n{wgsl}"))]
        ParseWgsl {
            source: naga::front::wgsl::ParseError,
            wgsl: String,
        },
    }

    #[test]
    // Ensure that the shaders can be converted to WGSL.
    // This is necessary for WASM using WebGPU, because WebGPU only accepts
    // WGSL as a shading language.
    fn validate_shaders() {
        fn validate_src(path: &std::path::PathBuf) -> Result<(), SrcError> {
            let bytes = std::fs::read(path).context(ReadSnafu)?;
            let opts = naga::front::spv::Options::default();
            let module = naga::front::spv::parse_u8_slice(&bytes, &opts).context(ParseSpvSnafu)?;
            let mut validator =
                naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
            let info = validator.validate(&module).context(ValidateSnafu)?;
            let wgsl = naga::back::wgsl::write_string(
                &module,
                &info,
                naga::back::wgsl::WriterFlags::empty(),
            )
            .context(WriteSnafu)?;

            let module = naga::front::wgsl::parse_str(&wgsl).context(ParseWgslSnafu { wgsl })?;
            let mut validator =
                naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
            let _info = validator.validate(&module).context(ValidateSnafu)?;
            Ok(())
        }

        let may_entries = std::fs::read_dir("src/linkage").unwrap();
        for may_entry in may_entries {
            let entry = may_entry.unwrap();
            println!("entry: {}", entry.path().display());
            let path = entry.path();
            let ext = path.extension().unwrap().to_str().unwrap();
            if ext == "spv" {
                if let Err(msg) = validate_src(&path) {
                    panic!("Invalid shader '{}': {msg}", path.display());
                }
            } else {
                println!("  skipping...");
            }
        }
    }
}
