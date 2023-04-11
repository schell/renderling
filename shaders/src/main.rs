//! This program builds the rust-gpu shaders and writes the spv files into the
//! main source repo.
//!
//! See the crates/renderling/src/shaders/mod.rs for more info.
use spirv_builder::{CompileResult, MetadataPrintout, ModuleResult, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        //.filter_module("renderling", log::LevelFilter::Trace)
        //.filter_module("naga", log::LevelFilter::Warn)
        //.filter_module("wgpu", log::LevelFilter::Debug)
        //.filter_module("wgpu_hal", log::LevelFilter::Warn)
        .try_init()
        .unwrap();

    let CompileResult {
        entry_points,
        module,
    } = SpirvBuilder::new("shader-crate", "spirv-unknown-spv1.5")
        .print_metadata(MetadataPrintout::None)
        .build()?;

    let dir = std::path::PathBuf::from("../crates/renderling/src/linkage");
    match module {
        ModuleResult::MultiModule(map) => {
            for (entry, filepath) in map.into_iter() {
                let path = dir.join(filepath.file_name().unwrap());
                println!(
                    "copying '{entry}' from {} to {}",
                    filepath.display(),
                    path.display()
                );
                std::fs::copy(filepath, path).unwrap();
            }
        }
        ModuleResult::SingleModule(filepath) => {
            let path = dir.join(filepath.file_name().unwrap());
            println!(
                "copying shader from {} to {} for entry points {}",
                filepath.display(),
                path.display(),
                entry_points.join(", ")
            );
            std::fs::copy(filepath, path).unwrap();
        }
    }

    Ok(())
}
