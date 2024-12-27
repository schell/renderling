//! Generates linkage for shaders and sets up cfg aliases.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("spirv") {
        renderling_build::generate_linkage();
    }

    cfg_aliases::cfg_aliases! {
        cpu: { not(target_arch = "spirv") },
        gpu: { target_arch = "spirv" },
    }
}
