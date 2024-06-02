//! Provides convenient wrappers around renderling shader linkage.
use std::sync::Arc;

#[cfg(feature = "bloom_downsample_fragment")]
pub mod bloom_downsample_fragment;
#[cfg(feature = "bloom_mix_fragment")]
pub mod bloom_mix_fragment;
#[cfg(feature = "bloom_upsample_fragment")]
pub mod bloom_upsample_fragment;
#[cfg(feature = "bloom_vertex")]
pub mod bloom_vertex;
#[cfg(feature = "brdf_lut_convolution_fragment")]
pub mod brdf_lut_convolution_fragment;
#[cfg(feature = "brdf_lut_convolution_vertex")]
pub mod brdf_lut_convolution_vertex;
#[cfg(feature = "generate_mipmap_fragment")]
pub mod generate_mipmap_fragment;
#[cfg(feature = "generate_mipmap_vertex")]
pub mod generate_mipmap_vertex;
#[cfg(feature = "prefilter_environment_cubemap_fragment")]
pub mod prefilter_environment_cubemap_fragment;
#[cfg(feature = "prefilter_environment_cubemap_vertex")]
pub mod prefilter_environment_cubemap_vertex;
#[cfg(feature = "renderlet_fragment")]
pub mod renderlet_fragment;
#[cfg(feature = "renderlet_vertex")]
pub mod renderlet_vertex;
#[cfg(feature = "skybox_cubemap_fragment")]
pub mod skybox_cubemap_fragment;
#[cfg(feature = "skybox_cubemap_vertex")]
pub mod skybox_cubemap_vertex;
#[cfg(feature = "skybox_equirectangular_fragment")]
pub mod skybox_equirectangular_fragment;
#[cfg(feature = "skybox_vertex")]
pub mod skybox_vertex;
#[cfg(feature = "tonemapping_fragment")]
pub mod tonemapping_fragment;
#[cfg(feature = "tonemapping_vertex")]
pub mod tonemapping_vertex;
#[cfg(feature = "tutorial_implicit_isosceles_vertex")]
pub mod tutorial_implicit_isosceles_vertex;
#[cfg(feature = "tutorial_passthru_fragment")]
pub mod tutorial_passthru_fragment;
#[cfg(feature = "tutorial_slabbed_renderlet")]
pub mod tutorial_slabbed_renderlet;
#[cfg(feature = "tutorial_slabbed_vertices")]
pub mod tutorial_slabbed_vertices;
#[cfg(feature = "tutorial_slabbed_vertices_no_instance")]
pub mod tutorial_slabbed_vertices_no_instance;

pub struct ShaderLinkage {
    pub module: Arc<wgpu::ShaderModule>,
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
        label: Some("slabs"),
        entries: &entries,
    })
}

pub fn slab_bindgroup(
    device: &wgpu::Device,
    slab_buffer: &wgpu::Buffer,
    bindgroup_layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
    let label = Some("slab");
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: bindgroup_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
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

    let atlas = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2Array,
            multisampled: false,
        },
        count: None,
    };
    let atlas_sampler = wgpu::BindGroupLayoutEntry {
        binding: 1,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        count: None,
    };
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
    atlas: &crate::atlas::Atlas,
    skybox: &crate::skybox::Skybox,
) -> wgpu::BindGroup {
    let label = Some("atlas and skybox");
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&atlas.get_texture().view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&atlas.get_texture().sampler),
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
    use naga::valid::ValidationFlags;

    #[test]
    // Ensure that the shaders can be converted to WGSL.
    // This is necessary for WASM using WebGPU, because WebGPU only accepts
    // WGSL as a shading language.
    fn validate_shaders() {
        fn validate_src(path: &std::path::PathBuf) {
            log::info!("validating source");
            log::info!("  reading '{}'", path.display());
            let bytes = std::fs::read(path).unwrap();
            log::info!("  {:0.2}k bytes read", bytes.len() as f32 / 1000.0);
            let opts = naga::front::spv::Options::default();
            let module = match naga::front::spv::parse_u8_slice(&bytes, &opts) {
                Ok(m) => m,
                Err(e) => {
                    log::error!("{e}");
                    panic!("SPIR-V parse error");
                }
            };
            log::info!("  SPIR-V parsed");
            let mut validator =
                naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
            let is_valid;
            let info = match validator.validate(&module) {
                Ok(i) => {
                    is_valid = true;
                    log::info!("  SPIR-V validated");
                    i
                }
                Err(e) => {
                    log::error!("{}", e.emit_to_string(""));
                    is_valid = false;
                    let mut validator = naga::valid::Validator::new(
                        ValidationFlags::empty(),
                        naga::valid::Capabilities::empty(),
                    );
                    validator.validate(&module).unwrap()
                }
            };
            let wgsl = naga::back::wgsl::write_string(
                &module,
                &info,
                naga::back::wgsl::WriterFlags::empty(),
            )
            .unwrap();
            log::info!("  output WGSL generated");

            let print_var_name = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replace('-', "_");
            let maybe_output_path = if std::env::var("print_wgsl").is_ok() || !is_valid {
                let dir = std::path::PathBuf::from("../../test_output");
                std::fs::create_dir_all(&dir).unwrap();
                let output_path = dir.join(print_var_name).with_extension("wgsl");
                log::info!("writing WGSL to '{}'", output_path.display());
                Some(output_path)
            } else {
                log::info!("  to save the generated WGSL, use an env var 'print_wgsl=1'");
                None
            };

            if let Some(output_path) = maybe_output_path {
                std::fs::write(&output_path, &wgsl).unwrap();
                log::info!("  wrote generated WGSL to {}", output_path.display());
            }

            if !is_valid {
                panic!("SPIR-V validation error");
            }

            let module = match naga::front::wgsl::parse_str(&wgsl) {
                Ok(m) => m,
                Err(e) => {
                    log::error!("{}", e.emit_to_string(&wgsl));
                    panic!("wgsl parse error");
                }
            };
            log::info!("  output WGSL parsed");
            let mut validator =
                naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
            let _info = match validator.validate(&module) {
                Ok(i) => i,
                Err(e) => {
                    log::error!("{}", e.emit_to_string(&wgsl));
                    panic!("wgsl validation error");
                }
            };
            log::info!("  wgsl output validated");
        }

        let may_entries = std::fs::read_dir("src/linkage").unwrap();
        for may_entry in may_entries {
            let entry = may_entry.unwrap();
            let path = entry.path();
            let ext = path.extension().unwrap().to_str().unwrap();
            if let Ok(filename) = std::env::var("only_shader") {
                let stem = path.file_stem().unwrap().to_str().unwrap();
                if filename != stem {
                    log::info!(
                        "  '{}' doesn't match 'only_shader' env '{}', skipping",
                        filename,
                        stem
                    );
                    continue;
                }
            }
            if ext == "spv" {
                validate_src(&path);
            }
        }
    }
}
