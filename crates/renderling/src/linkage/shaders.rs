//! Shader linkage.
//!
//! This module is auto-generated.
#![allow(non_snake_case)]
use super::ShaderLinkage;
/// Returns the shader linkage for
/// [convolution::fragment_bloom](crate::convolution::fragment_bloom).
///
/// **spv path**: `crates/renderling/src/linkage/convolution-fragment_bloom.spv`
pub fn convolution__fragment_bloom(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}", stringify!(convolution__fragment_bloom)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("convolution-fragment_bloom.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(convolution__fragment_bloom)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::fragment_bloom",
    }
}
/// Returns the shader linkage for
/// [convolution::fragment_brdf_lut_convolution](crate::convolution::fragment_brdf_lut_convolution).
///
/// **spv path**: `crates/renderling/src/linkage/convolution-fragment_brdf_lut_convolution.spv`
pub fn convolution__fragment_brdf_lut_convolution(
    device: &wgpu::Device,
) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(convolution__fragment_brdf_lut_convolution)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(
            wgpu::include_spirv!("convolution-fragment_brdf_lut_convolution.spv"),
        );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(convolution__fragment_brdf_lut_convolution)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::fragment_brdf_lut_convolution",
    }
}
/// Returns the shader linkage for
/// [convolution::fragment_generate_mipmap](crate::convolution::fragment_generate_mipmap).
///
/// **spv path**: `crates/renderling/src/linkage/convolution-fragment_generate_mipmap.spv`
pub fn convolution__fragment_generate_mipmap(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(convolution__fragment_generate_mipmap)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(
            wgpu::include_spirv!("convolution-fragment_generate_mipmap.spv"),
        );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(convolution__fragment_generate_mipmap)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::fragment_generate_mipmap",
    }
}
/// Returns the shader linkage for
/// [convolution::fragment_prefilter_environment_cubemap](crate::convolution::fragment_prefilter_environment_cubemap).
///
/// **spv path**: `crates/renderling/src/linkage/convolution-fragment_prefilter_environment_cubemap.spv`
pub fn convolution__fragment_prefilter_environment_cubemap(
    device: &wgpu::Device,
) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(convolution__fragment_prefilter_environment_cubemap)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(
            wgpu::include_spirv!(
                "convolution-fragment_prefilter_environment_cubemap.spv"
            ),
        );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(convolution__fragment_prefilter_environment_cubemap)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::fragment_prefilter_environment_cubemap",
    }
}
/// Returns the shader linkage for
/// [convolution::vertex_brdf_lut_convolution](crate::convolution::vertex_brdf_lut_convolution).
///
/// **spv path**: `crates/renderling/src/linkage/convolution-vertex_brdf_lut_convolution.spv`
pub fn convolution__vertex_brdf_lut_convolution(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(convolution__vertex_brdf_lut_convolution)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(
            wgpu::include_spirv!("convolution-vertex_brdf_lut_convolution.spv"),
        );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(convolution__vertex_brdf_lut_convolution)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::vertex_brdf_lut_convolution",
    }
}
/// Returns the shader linkage for
/// [convolution::vertex_generate_mipmap](crate::convolution::vertex_generate_mipmap).
///
/// **spv path**: `crates/renderling/src/linkage/convolution-vertex_generate_mipmap.spv`
pub fn convolution__vertex_generate_mipmap(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}", stringify!(convolution__vertex_generate_mipmap)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(
            wgpu::include_spirv!("convolution-vertex_generate_mipmap.spv"),
        );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(convolution__vertex_generate_mipmap)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::vertex_generate_mipmap",
    }
}
/// Returns the shader linkage for
/// [convolution::vertex_prefilter_environment_cubemap](crate::convolution::vertex_prefilter_environment_cubemap).
///
/// **spv path**: `crates/renderling/src/linkage/convolution-vertex_prefilter_environment_cubemap.spv`
pub fn convolution__vertex_prefilter_environment_cubemap(
    device: &wgpu::Device,
) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(convolution__vertex_prefilter_environment_cubemap)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(
            wgpu::include_spirv!("convolution-vertex_prefilter_environment_cubemap.spv"),
        );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(convolution__vertex_prefilter_environment_cubemap)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::vertex_prefilter_environment_cubemap",
    }
}
/// Returns the shader linkage for
/// [pbr::pbr_fragment](crate::pbr::pbr_fragment).
///
/// **spv path**: `crates/renderling/src/linkage/pbr-pbr_fragment.spv`
pub fn pbr__pbr_fragment(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(pbr__pbr_fragment));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("pbr-pbr_fragment.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}", stringify!(pbr__pbr_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "pbr::pbr_fragment",
    }
}
/// Returns the shader linkage for
/// [sdf::raymarch::raymarch_fragment](crate::sdf::raymarch::raymarch_fragment).
///
/// **spv path**: `crates/renderling/src/linkage/sdf-raymarch-raymarch_fragment.spv`
pub fn sdf__raymarch__raymarch_fragment(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}", stringify!(sdf__raymarch__raymarch_fragment)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(
            wgpu::include_spirv!("sdf-raymarch-raymarch_fragment.spv"),
        );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(sdf__raymarch__raymarch_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::raymarch::raymarch_fragment",
    }
}
/// Returns the shader linkage for
/// [sdf::raymarch::raymarch_rays](crate::sdf::raymarch::raymarch_rays).
///
/// **spv path**: `crates/renderling/src/linkage/sdf-raymarch-raymarch_rays.spv`
pub fn sdf__raymarch__raymarch_rays(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}", stringify!(sdf__raymarch__raymarch_rays)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("sdf-raymarch-raymarch_rays.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(sdf__raymarch__raymarch_rays)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::raymarch::raymarch_rays",
    }
}
/// Returns the shader linkage for
/// [sdf::raymarch::raymarch_vertex](crate::sdf::raymarch::raymarch_vertex).
///
/// **spv path**: `crates/renderling/src/linkage/sdf-raymarch-raymarch_vertex.spv`
pub fn sdf__raymarch__raymarch_vertex(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}", stringify!(sdf__raymarch__raymarch_vertex)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("sdf-raymarch-raymarch_vertex.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(sdf__raymarch__raymarch_vertex)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::raymarch::raymarch_vertex",
    }
}
/// Returns the shader linkage for
/// [sdf::sdf_prim_fragment_test](crate::sdf::sdf_prim_fragment_test).
///
/// **spv path**: `crates/renderling/src/linkage/sdf-sdf_prim_fragment_test.spv`
pub fn sdf__sdf_prim_fragment_test(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}", stringify!(sdf__sdf_prim_fragment_test)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("sdf-sdf_prim_fragment_test.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(sdf__sdf_prim_fragment_test)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::sdf_prim_fragment_test",
    }
}
/// Returns the shader linkage for
/// [sdf::sdf_shape_fragment](crate::sdf::sdf_shape_fragment).
///
/// **spv path**: `crates/renderling/src/linkage/sdf-sdf_shape_fragment.spv`
pub fn sdf__sdf_shape_fragment(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(sdf__sdf_shape_fragment));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("sdf-sdf_shape_fragment.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(sdf__sdf_shape_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::sdf_shape_fragment",
    }
}
/// Returns the shader linkage for
/// [sdf::sdf_shape_vertex](crate::sdf::sdf_shape_vertex).
///
/// **spv path**: `crates/renderling/src/linkage/sdf-sdf_shape_vertex.spv`
pub fn sdf__sdf_shape_vertex(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(sdf__sdf_shape_vertex));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("sdf-sdf_shape_vertex.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(sdf__sdf_shape_vertex)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::sdf_shape_vertex",
    }
}
/// Returns the shader linkage for
/// [shader_test::array_test](crate::shader_test::array_test).
///
/// **spv path**: `crates/renderling/src/linkage/shader_test-array_test.spv`
pub fn shader_test__array_test(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(shader_test__array_test));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("shader_test-array_test.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(shader_test__array_test)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "shader_test::array_test",
    }
}
/// Returns the shader linkage for
/// [skybox::fragment_cubemap](crate::skybox::fragment_cubemap).
///
/// **spv path**: `crates/renderling/src/linkage/skybox-fragment_cubemap.spv`
pub fn skybox__fragment_cubemap(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(skybox__fragment_cubemap));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("skybox-fragment_cubemap.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(skybox__fragment_cubemap)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "skybox::fragment_cubemap",
    }
}
/// Returns the shader linkage for
/// [skybox::fragment_equirectangular](crate::skybox::fragment_equirectangular).
///
/// **spv path**: `crates/renderling/src/linkage/skybox-fragment_equirectangular.spv`
pub fn skybox__fragment_equirectangular(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}", stringify!(skybox__fragment_equirectangular)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(
            wgpu::include_spirv!("skybox-fragment_equirectangular.spv"),
        );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(skybox__fragment_equirectangular)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "skybox::fragment_equirectangular",
    }
}
/// Returns the shader linkage for
/// [skybox::vertex](crate::skybox::vertex).
///
/// **spv path**: `crates/renderling/src/linkage/skybox-vertex.spv`
pub fn skybox__vertex(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(skybox__vertex));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!("skybox-vertex.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}", stringify!(skybox__vertex)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "skybox::vertex",
    }
}
/// Returns the shader linkage for
/// [skybox::vertex_cubemap](crate::skybox::vertex_cubemap).
///
/// **spv path**: `crates/renderling/src/linkage/skybox-vertex_cubemap.spv`
pub fn skybox__vertex_cubemap(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(skybox__vertex_cubemap));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("skybox-vertex_cubemap.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(skybox__vertex_cubemap)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "skybox::vertex_cubemap",
    }
}
/// Returns the shader linkage for
/// [stage::fragment](crate::stage::fragment).
///
/// **spv path**: `crates/renderling/src/linkage/stage-fragment.spv`
pub fn stage__fragment(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(stage__fragment));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!("stage-fragment.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}", stringify!(stage__fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "stage::fragment",
    }
}
/// Returns the shader linkage for
/// [stage::test_i8_i16_extraction](crate::stage::test_i8_i16_extraction).
///
/// **spv path**: `crates/renderling/src/linkage/stage-test_i8_i16_extraction.spv`
pub fn stage__test_i8_i16_extraction(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}", stringify!(stage__test_i8_i16_extraction)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("stage-test_i8_i16_extraction.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(stage__test_i8_i16_extraction)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "stage::test_i8_i16_extraction",
    }
}
/// Returns the shader linkage for
/// [stage::vertex](crate::stage::vertex).
///
/// **spv path**: `crates/renderling/src/linkage/stage-vertex.spv`
pub fn stage__vertex(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(stage__vertex));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!("stage-vertex.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}", stringify!(stage__vertex)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "stage::vertex",
    }
}
/// Returns the shader linkage for
/// [tonemapping::fragment](crate::tonemapping::fragment).
///
/// **spv path**: `crates/renderling/src/linkage/tonemapping-fragment.spv`
pub fn tonemapping__fragment(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(tonemapping__fragment));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("tonemapping-fragment.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(tonemapping__fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "tonemapping::fragment",
    }
}
/// Returns the shader linkage for
/// [tonemapping::vertex](crate::tonemapping::vertex).
///
/// **spv path**: `crates/renderling/src/linkage/tonemapping-vertex.spv`
pub fn tonemapping__vertex(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(tonemapping__vertex));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device
        .create_shader_module(wgpu::include_spirv!("tonemapping-vertex.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(tonemapping__vertex)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "tonemapping::vertex",
    }
}
