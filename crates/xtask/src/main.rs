//! A build helper for the `renderling` project.
use clap::{Parser, Subcommand};

mod deps;
mod server;

#[derive(Subcommand)]
enum Command {
    /// Compile the `renderling` library into multiple SPIR-V shader entry points.
    CompileShaders,
    /// Generate Rust files linking `wgpu` shaders from SPIR-V shader entry points.
    GenerateLinkage {
        /// Whether to generate WGSL shaders.
        #[clap(long)]
        wgsl: bool,

        /// Only generate linkage for the given shader function.
        #[clap(long)]
        only_fn_with_name: Option<String>,

        /// Print cargo output as if called from cargo (this is for testing).
        #[clap(long)]
        from_cargo: bool,
    },
    /// Run the WASM test server
    WasmServer,
    /// Compile for WASM and run headless browser tests
    TestWasm {
        /// Cargo args.
        #[clap(last = true)]
        args: Vec<String>,
        /// Set to use chrome, otherwise firefox will be used.
        #[clap(long)]
        chrome: bool,
    },
    /// Build the manual
    BuildManual,
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    /// The subcommand to run
    subcommand: Command,
}

#[tokio::main]
async fn main() {
    env_logger::builder().init();

    log::info!("running xtask");

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
        Command::GenerateLinkage {
            wgsl,
            from_cargo,
            only_fn_with_name,
        } => {
            log::info!("Generating linkage for shaders");
            let paths = renderling_build::RenderlingPaths::new().unwrap();
            paths.generate_linkage(from_cargo, wgsl, only_fn_with_name);
        }
        Command::TestWasm { args, chrome } => {
            log::info!("testing WASM");
            let _proxy_handle = tokio::spawn(server::serve());
            let mut test_handle = tokio::process::Command::new("wasm-pack");
            test_handle.args([
                "test",
                "--headless",
                if chrome { "--chrome" } else { "--firefox" },
                "crates/renderling",
                "--features",
                "wasm",
            ]);
            if !args.is_empty() {
                test_handle.arg("--").args(args);
            }
            let mut test_handle = test_handle.spawn().unwrap();
            let status = test_handle.wait().await.unwrap();
            if !status.success() {
                panic!("Testing WASM failed :(");
            }
        }
        Command::WasmServer => {
            server::serve().await;
        }
        Command::BuildManual => {
            log::info!("building manual");
            if !deps::has_binary("mdbook").await {
                deps::cargo_install("mdbook").await;
            }
        }
    }
}
