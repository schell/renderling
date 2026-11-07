//! Generates linkage for shaders and sets up cfg aliases.

fn main() {
    cfg_aliases::cfg_aliases! {
        cpu: { not(target_arch = "spirv") },
        gpu: { target_arch = "spirv" },
        gltf: { feature = "gltf" }
    }
}
