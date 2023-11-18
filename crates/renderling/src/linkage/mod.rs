//! Ensures our shaders are valid.
//!
//! This module exists just for testing. It doesn't export anything.
#[cfg(test)]
mod test {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    enum SrcError {
        #[snafu(display("{source}"))]
        Read { source: std::io::Error },
        #[snafu(display("{source}"))]
        ParseSpv { source: naga::front::spv::Error },
        #[snafu(display("{source}"))]
        Validate {
            source: naga::WithSpan<naga::valid::ValidationError>,
        },
        #[snafu(display("{source}"))]
        Write { source: naga::back::wgsl::Error },
        #[snafu(display("{source}:\n{wgsl}"))]
        ParseWgsl {
            source: naga::front::wgsl::ParseError,
            wgsl: String,
        },
    }

    #[test]
    // Ensure that the shaders can be converted to WGSL.
    // This is necessary for WASM using WebGPU, because WebGPU only accepts
    // WGSL as a shading language.:w
    fn validate_shaders() {
        fn validate_src(path: &std::path::PathBuf) -> Result<(), SrcError> {
            let bytes = std::fs::read(path).context(ReadSnafu)?;
            let opts = naga::front::spv::Options::default();
            let module = naga::front::spv::parse_u8_slice(&bytes, &opts).context(ParseSpvSnafu)?;
            let mut validator =
                naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
            let info = validator.validate(&module).context(ValidateSnafu)?;
            let wgsl = naga::back::wgsl::write_string(
                &module,
                &info,
                naga::back::wgsl::WriterFlags::empty(),
            )
            .context(WriteSnafu)?;

            let module = naga::front::wgsl::parse_str(&wgsl).context(ParseWgslSnafu { wgsl })?;
            let mut validator =
                naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
            let _info = validator.validate(&module).context(ValidateSnafu)?;
            Ok(())
        }

        let may_entries = std::fs::read_dir("src/linkage").unwrap();
        for may_entry in may_entries {
            let entry = may_entry.unwrap();
            println!("entry: {}", entry.path().display());
            let path = entry.path();
            let ext = path.extension().unwrap().to_str().unwrap();
            if ext == "spv" {
                if let Err(msg) = validate_src(&path) {
                    panic!("Invalid shader '{}': {msg}", path.display());
                }
            } else {
                println!("  skipping...");
            }
        }
    }
}
