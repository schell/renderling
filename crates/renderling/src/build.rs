//! Generates linkage for shaders and sets up cfg aliases.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("spirv") {
        if let Some(paths) = renderling_build::RenderlingPaths::new() {
            paths.generate_linkage(true, true, None);
        }
    }

    cfg_aliases::cfg_aliases! {
        cpu: { not(target_arch = "spirv") },
        gpu: { target_arch = "spirv" },
    }
}
