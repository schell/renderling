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

            log::info!(
                "Calling `cargo gpu toml {}",
                paths.renderling_crate.display()
            );

            std::process::Command::new("cargo")
                .args(["gpu", "toml"])
                .arg(&paths.renderling_crate)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .output()
                .unwrap();
        }
        Command::GenerateLinkage => {
            log::info!("Generating linkage for shaders");
            let paths = renderling_build::RenderlingPaths::new().unwrap();
            paths.generate_linkage();
        }
    }
}
