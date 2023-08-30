//! Ensures our shaders are valid.
//!
//! This module exists just for testing. It doesn't export anything.
mod test {
    #[test]
    // Ensure that the shaders can be converted to WGSL.
    // This is necessary for WASM using WebGPU, because WebGPU only accepts
    // WGSL as a shading language.
    fn validate_shaders() {
        fn validate_src(path: &std::path::PathBuf) {
            let bytes = std::fs::read(path).unwrap();
            let opts = naga::front::spv::Options::default();
            let module = naga::front::spv::parse_u8_slice(&bytes, &opts).unwrap();
            let mut validator =
                naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
            let info = validator.validate(&module).unwrap();
            let wgsl = naga::back::wgsl::write_string(
                &module,
                &info,
                naga::back::wgsl::WriterFlags::empty(),
            )
            .unwrap();
            println!("wgsl: {wgsl}");
            //let (msl, _) = naga::back::msl::write_string(
            //    &module,
            //    &info,
            //    &Default::default(),
            //    &Default::default(),
            //)
            //.unwrap();
            //println!("msl: {msl}");

            let module = naga::front::wgsl::parse_str(&wgsl).unwrap();
            let mut validator =
                naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
            let info = validator.validate(&module).unwrap();
            println!("info: {info:#?}");
        }

        let may_entries = std::fs::read_dir("src/linkage").unwrap();
        for may_entry in may_entries {
            let entry = may_entry.unwrap();
            println!("entry: {}", entry.path().display());
            validate_src(&entry.path());
        }
    }
}
