//! This program builds the rust-gpu shaders and writes the spv files into the
//! main source repo.
//!
//! See the crates/renderling/src/shaders/mod.rs for more info.
use spirv_builder::{CompileResult, MetadataPrintout, ModuleResult, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .try_init()
        .unwrap();

    let CompileResult {
        entry_points,
        module,
    } = SpirvBuilder::new("shader-crate", "spirv-unknown-vulkan1.2")
        .print_metadata(MetadataPrintout::None)
        .multimodule(true)
        .build()?;

    let dir = std::path::PathBuf::from("../crates/renderling/src/linkage");
    match module {
        ModuleResult::MultiModule(modules) => {
            for (entry, filepath) in modules.into_iter() {
                let path = dir.join(filepath.file_name().unwrap());
                println!(
                    "copying '{entry}':\n  from {}\n  to {}\n",
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
