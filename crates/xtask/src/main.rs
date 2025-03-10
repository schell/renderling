//! A build helper for the `renderling` project.
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    /// Compile the `renderling` library into multiple SPIR-V shader entry points.
    CompileShaders,
    /// Generate Rust files linking `wgpu` shaders from SPIR-V shader entry points.
    GenerateLinkage,
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    /// The subcommand to run
    subcommand: Command,
}

fn main() {
    env_logger::builder().init();

    let cli = Cli::parse();
    match cli.subcommand {
        Command::CompileShaders => {
            let paths = renderling_build::RenderlingPaths::new().unwrap();

            log::info!("Calling `cargo gpu {}", paths.renderling_crate.display());

            let output = std::process::Command::new("cargo")
                .args(["gpu", "build", "--shader-crate"])
                .arg(&paths.renderling_crate)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .output()
                .unwrap();
            if !output.status.success() {
                panic!("Building shaders failed :(");
            }
        }
        Command::GenerateLinkage => {
            log::info!("Generating linkage for shaders");
            let paths = renderling_build::RenderlingPaths::new().unwrap();
            paths.generate_linkage();
        }
    }
}
