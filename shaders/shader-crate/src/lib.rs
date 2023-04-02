#![no_std]
#![feature(lang_items)]
use spirv_std::spirv;

mod ui;
pub use ui::*;

mod pbr;
pub use pbr::*;

#[spirv(compute(threads(1)))]
pub fn compute_test_storage(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)]
    test: &mut renderling_shader::TestStorage
) {
    renderling_shader::compute_test_storage(test)
}
