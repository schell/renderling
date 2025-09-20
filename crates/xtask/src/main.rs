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
    /// Perform actions regarding the manual
    Manual(Manual),
}

#[derive(Parser)]
pub struct Manual {
    /// Whether to skip building the docs
    #[clap(long)]
    no_build_docs: bool,

    /// Whether to skip testing the manual
    #[clap(long)]
    no_test: bool,

    /// The URL to the renderling docs
    #[clap(long, default_value = "http://localhost:4000")]
    docs_url: String,

    /// Serve the manual instead of simply building it
    #[clap(long)]
    serve: bool,
}

impl Manual {
    async fn install_deps() {
        const DEPS: &[&str] = &["mdbook", "mdbook-environment"];
        for dep in DEPS {
            if !deps::has_binary(dep).await {
                deps::cargo_install(dep).await;
            }
        }
    }

    async fn build_docs() {
        log::info!("building docs");
        let mut process = tokio::process::Command::new("cargo")
            .args(["doc", "-p", "renderling", "--all-features"])
            .spawn()
            .unwrap();
        let status = process.wait().await.unwrap();
        if !status.success() {
            panic!("Failed building docs");
        }
    }

    async fn test() {
        log::info!("testing the manual snippets");
        let mut process = tokio::process::Command::new("cargo")
            .args(["test", "-p", "test-manual"])
            .spawn()
            .unwrap();
        let status = process.wait().await.unwrap();
        if !status.success() {
            panic!("Failed testing manual");
        }
    }
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
        Command::Manual(Manual {
            no_build_docs,
            no_test,
            docs_url,
            serve,
        }) => {
            log::info!("checking dependencies for the manual");
            Manual::install_deps().await;
            if !no_test {
                Manual::test().await;
            }
            if !no_build_docs {
                Manual::build_docs().await;
            }

            if serve {
                log::info!("serving manual");

                // serve the docs in the meantime
                let _docs_handle = tokio::spawn(server::serve_docs());

                let mut build = tokio::process::Command::new("mdbook")
                    .arg("serve")
                    .current_dir(
                        std::path::PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("manual"),
                    )
                    .env("DOCS_URL", docs_url)
                    .spawn()
                    .unwrap();
                let build_status = build.wait().await.unwrap();
                if !build_status.success() {
                    log::error!("could not build the manual");
                }
            } else {
                log::info!("building manual");

                let mut build = tokio::process::Command::new("mdbook")
                    .arg("build")
                    .env("DOCS_URL", docs_url)
                    .current_dir(
                        std::path::PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("manual"),
                    )
                    .spawn()
                    .unwrap();
                let build_status = build.wait().await.unwrap();
                if !build_status.success() {
                    log::error!("could not build the manual");
                }
            }
        }
    }
}
