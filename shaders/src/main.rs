//! This program builds the rust-gpu shaders and writes the spv files into the main source repo.
//!
//! See the crates/renderling/src/shaders/mod.rs for more info.
use spirv_builder::{ModuleResult, MetadataPrintout, SpirvBuilder, CompileResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let CompileResult{ entry_points:_, module } = SpirvBuilder::new("shader-crate", "spirv-unknown-spv1.5")
        .print_metadata(MetadataPrintout::None)
        .multimodule(true)
        .build()?;

    match module {
        ModuleResult::MultiModule(map) => {
            let dir = std::path::PathBuf::from("../crates/renderling/src/linkage");
            for (entry, filepath) in map.into_iter() {
                let path = dir.join(filepath.file_name().unwrap());
                println!("copying '{entry}' from {} to {}", filepath.display(), path.display());
                std::fs::copy(filepath, path).unwrap();
            }
        }
        _ => {}
    }

    Ok(())
}
