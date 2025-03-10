#![allow(unexpected_cfgs)]
use quote::quote;

#[derive(Debug, serde::Deserialize)]
struct Linkage {
    source_path: std::path::PathBuf,
    entry_point: String,
    wgsl_entry_point: String,
}

impl core::fmt::Display for Linkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spv_source_path = self.source_path.clone();
        let spv_source_filename = spv_source_path.file_name().unwrap().to_str().unwrap();
        let spv_include_source_path = format!("../../shaders/{spv_source_filename}");

        let wgsl_source_path = self.source_path.with_extension("wgsl");
        let wgsl_source_filename = wgsl_source_path.file_name().unwrap().to_str().unwrap();
        let wgsl_include_source_path = format!("../../shaders/{wgsl_source_filename}");

        let Linkage {
            source_path: _,
            entry_point,
            wgsl_entry_point,
        } = self;

        let fn_name = self.fn_name();

        let quote = quote! {
            use crate::linkage::ShaderLinkage;

            #[cfg(not(target_arch = "wasm32"))]
            mod target {
                pub const ENTRY_POINT: &str = #entry_point;

                pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
                    wgpu::include_spirv!(#spv_include_source_path)
                }

                pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
                    log::debug!("creating native linkage for {}", #fn_name);
                    super::ShaderLinkage {
                        entry_point: ENTRY_POINT,
                        module: device.create_shader_module(descriptor()).into()
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            mod target {
                pub const ENTRY_POINT: &str = #wgsl_entry_point;

                pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
                    wgpu::include_wgsl!(#wgsl_include_source_path)
                }

                pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
                    log::debug!("creating web linkage for {}", #fn_name);
                    super::ShaderLinkage {
                        entry_point: ENTRY_POINT,
                        module: device.create_shader_module(descriptor()).into()
                    }
                }
            }

            pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
                target::linkage(device)
            }
        };

        f.write_fmt(format_args!(
            r#"#![allow(dead_code)]
            //! Automatically generated by Renderling's `build.rs`.
            {quote}
            "#,
        ))
    }
}

impl Linkage {
    pub fn fn_name(&self) -> &str {
        self.entry_point.split("::").last().unwrap()
    }
}

fn wgsl(spv_filepath: impl AsRef<std::path::Path>, destination: impl AsRef<std::path::Path>) {
    let bytes = std::fs::read(spv_filepath.as_ref()).unwrap_or_else(|e| {
        panic!(
            "could not read spv filepath '{}' while attempting to translate to wgsl: {e}",
            spv_filepath.as_ref().display()
        );
    });
    let opts = naga::front::spv::Options::default();
    let module = naga::front::spv::parse_u8_slice(&bytes, &opts).unwrap();
    let mut validator =
        naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
    let info = validator.validate(&module).unwrap_or_else(|e| {
        panic!(
            "Could not validate '{}': {e}",
            spv_filepath.as_ref().display(),
        )
    });
    let wgsl =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let destination = destination.as_ref().with_extension("wgsl");
    std::fs::write(destination, wgsl).unwrap();
}

pub struct RenderlingPaths {
    pub cargo_workspace: std::path::PathBuf,
    pub renderling_crate: std::path::PathBuf,
    pub shader_dir: std::path::PathBuf,
    pub shader_manifest: std::path::PathBuf,
    pub linkage_dir: std::path::PathBuf,
}

impl RenderlingPaths {
    /// Create a new `RenderlingPaths`.
    ///
    /// If the `CARGO_WORKSPACE_DIR` is _not_ available, this most likely means we're building renderling
    /// outside of its own source tree, which means we **don't want to compile shaders or generate linkage**.
    ///
    /// For this reason we return `Option`.
    pub fn new() -> Option<Self> {
        let cargo_workspace = std::path::PathBuf::from(std::env::var("CARGO_WORKSPACE_DIR").ok()?);
        let renderling_crate = cargo_workspace.join("crates").join("renderling");
        log::debug!("cargo_manifest_dir: {renderling_crate:#?}");
        let shader_dir = renderling_crate.join("shaders");

        let shader_manifest = shader_dir.join("manifest.json");
        let linkage_dir = renderling_crate.join("src").join("linkage");

        Some(Self {
            cargo_workspace,
            renderling_crate,
            shader_dir,
            shader_manifest,
            linkage_dir,
        })
    }

    /// Generate linkage (Rust source) files for each shader in the manifest.
    pub fn generate_linkage(&self) {
        log::trace!("{:#?}", std::env::vars().collect::<Vec<_>>());
        assert!(
            self.shader_manifest.is_file(),
            "missing file '{}', you must first compile the shaders",
            self.shader_manifest.display()
        );

        if !self.linkage_dir.is_dir() {
            log::info!("creating linkage directory");
            std::fs::create_dir_all(&self.linkage_dir).unwrap();
        }

        log::debug!("cwd: {:?}", std::env::current_dir().unwrap());

        let manifest_file = std::fs::File::open(&self.shader_manifest).unwrap();
        let manifest: Vec<Linkage> = serde_json::from_reader(manifest_file).unwrap();
        let mut set = std::collections::HashSet::new();
        for linkage in manifest.into_iter() {
            log::debug!("linkage: {linkage:#?}");
            let fn_name = linkage.fn_name();

            if set.contains(fn_name) {
                panic!("Shader name '{fn_name}' is used for two or more shaders, aborting!");
            }
            set.insert(fn_name.to_string());

            let absolute_source_path = self
                .shader_dir
                .join(linkage.source_path.file_name().unwrap());
            let wgsl_source_path = linkage.source_path.with_extension("wgsl");
            let absolute_wgsl_source_path =
                self.shader_dir.join(wgsl_source_path.file_name().unwrap());
            wgsl(absolute_source_path, absolute_wgsl_source_path);

            let filepath = self.linkage_dir.join(fn_name).with_extension("rs");
            log::info!("generating: {}", linkage.entry_point,);

            let contents = linkage.to_string();
            std::fs::write(&filepath, contents).unwrap();
            std::process::Command::new("rustfmt")
                .args([&format!("{}", filepath.display())])
                .output()
                .expect("could not format generated code");
        }
        log::info!("...done!")
    }
}
