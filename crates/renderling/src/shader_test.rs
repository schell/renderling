use glam::Vec4;
use spirv_std::spirv;

#[cfg(feature = "array_test")]
#[spirv(fragment)]
pub fn array_test(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(flat)] in_instance: u32,

    frag_color: &mut Vec4,
) {
    let index = in_instance as usize;
    let r = slab[index as usize];
    *frag_color = Vec4::new(f32::from_bits(r), 0.0, 0.0, 1.0);
}
