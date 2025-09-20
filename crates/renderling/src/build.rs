//! Generates linkage for shaders and sets up cfg aliases.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("spirv") {
        let paths = renderling_build::RenderlingPaths::new();
        if let Some(paths) = paths {
            paths.generate_linkage(true, true, None);
        }
    }

    cfg_aliases::cfg_aliases! {
        cpu: { not(target_arch = "spirv") },
        gpu: { target_arch = "spirv" },
        gltf: { feature = "gltf" }
    }
}
