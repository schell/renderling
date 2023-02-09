use spirv_builder::{MetadataPrintout, SpirvBuilder};

const SHADERS: [&'static str; 1] = [
    "renderling-ui-shader"
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for shader in SHADERS {
        SpirvBuilder::new(shader, "spirv-unknown-spv1.5")
            .print_metadata(MetadataPrintout::Full)
            .build()?;
    }
    Ok(())
}
