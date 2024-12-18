//! Generates linkage for shaders.
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
                    log::info!("creating native linkage for {}", #fn_name);
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
                    log::info!("creating web linkage for {}", #fn_name);
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
    let info = validator.validate(&module).unwrap();
    let wgsl =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let destination = destination.as_ref().with_extension("wgsl");
    std::fs::write(destination, wgsl).unwrap();
}

/// Generate linkage (Rust source) files for each shader in the manifest.
fn generate_linkage() {
    println!("{:#?}", std::env::vars().collect::<Vec<_>>());
    let cargo_manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let shader_dir = cargo_manifest_dir.join("shaders");
    assert!(
        shader_dir.is_dir(),
        "missing directory '{}', you must first compile the shaders",
        shader_dir.display()
    );
    println!("cargo:rerun-if-changed={}", shader_dir.display());

    let shader_manifest = cargo_manifest_dir.join("shaders/manifest.json");
    assert!(
        shader_manifest.is_file(),
        "missing file '{}', you must first compile the shaders",
        shader_manifest.display()
    );
    let linkage_dir = cargo_manifest_dir.join("src/linkage");
    assert!(
        linkage_dir.is_dir(),
        "missing crates/renderling/src/linkage"
    );

    println!("cwd: {:?}", std::env::current_dir().unwrap());

    let manifest_file = std::fs::File::open(&shader_manifest).unwrap();
    let manifest: Vec<Linkage> = serde_json::from_reader(manifest_file).unwrap();
    let mut set = std::collections::HashSet::new();
    for linkage in manifest.into_iter() {
        println!("linkage: {linkage:#?}");
        let fn_name = linkage.fn_name();

        if set.contains(fn_name) {
            panic!("Shader name '{fn_name}' is used for two or more shaders, aborting!");
        }
        set.insert(fn_name.to_string());

        let absolute_source_path = shader_dir.join(linkage.source_path.file_name().unwrap());
        let wgsl_source_path = linkage.source_path.with_extension("wgsl");
        let absolute_wgsl_source_path = shader_dir.join(wgsl_source_path.file_name().unwrap());
        wgsl(absolute_source_path, absolute_wgsl_source_path);

        let filepath = linkage_dir.join(fn_name).with_extension("rs");
        println!("filepath: {}", filepath.display());
        let contents = linkage.to_string();
        std::fs::write(&filepath, contents).unwrap();
        std::process::Command::new("rustfmt")
            .args([&format!("{}", filepath.display())])
            .output()
            .expect("could not format generated code");
    }
}

fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("spirv") {
        generate_linkage();
    }
}
