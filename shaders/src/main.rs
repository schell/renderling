//! This program builds the rust-gpu shaders and writes the spv files into the main source repo.
//!
//! See the crates/renderling/src/shaders/mod.rs for more info.
const SHADERS: [(&'static str, &'static [u8]); 2] = [
    ("ui", include_bytes!(env!("renderling_ui_shader.spv"))),
    ("pbr", include_bytes!(env!("renderling_pbr_shader.spv"))),
];

pub fn main() {
    for (name, src) in SHADERS {
        let path = format!("../crates/renderling/src/linkage/{}.spv", name);
        std::fs::write(&path, src).unwrap();
        println!("wrote shader '{name}' to '{path}'");
    }
}
