use std::marker::PhantomData;

use glam::{Vec2, Vec3};
use wgsly_macros::wgsl;
pub use wgsly_macros::Constructable;

pub trait Constructable {
    fn read(slab: &[u32], id: u32) -> Self;
    fn write(self, slab: &mut [u32], id: u32);
}

impl Constructable for f32 {
    fn read(slab: &[u32], id: u32) -> Self {
        f32::from_bits(slab[id as usize])
    }

    fn write(self, slab: &mut [u32], id: u32) {
        slab[id as usize] = self.to_bits();
    }
}

impl Constructable for Vec2 {
    fn read(slab: &[u32], id: u32) -> Self {
        Vec2::new(f32::read(slab, id), f32::read(slab, id + 1))
    }

    fn write(self, slab: &mut [u32], id: u32) {
        self.x.write(slab, id);
        self.y.write(slab, id + 1);
    }
}

pub struct Wgsl<T> {
    pub source: &'static str,
    _phantom: PhantomData<T>,
}

#[derive(Clone, Copy)]
pub struct MyThing {
    pub distance: f32,
    pub direction: Vec3,
    pub is_alive: bool,
}

const AVG_FN: wgpu::ShaderModuleDescriptor<'static> = wgsl!({
    // Function with two parameters, which returns a 'f32' value
    fn average(a: f32, b: f32) -> f32 {
        return (a + b) / 2;
    }
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wgsl_output() {
        let module = naga::front::wgsl::parse_str(AVG_FN).unwrap();
        let mut validator =
            naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
        let info = validator.validate(&module).unwrap();
        panic!(
            "{}",
            naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
                .unwrap()
        );
    }
}
