//! Cubemap utilities.
//!
//! Shaders, render pipelines and layouts for creating and sampling cubemaps.
//!
//! For more info see:
//! * <https://github.com/markpmlim/MetalCubemapping>

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

pub mod shader;

#[cfg(test)]
mod test {
    use glam::{Vec2, Vec3};

    use crate::cubemap::shader::{CubemapDescriptor, CubemapFaceDirection};

    #[test]
    fn cubemap_right() {
        assert_eq!(Vec3::NEG_Z, CubemapFaceDirection::X.right());
        assert_eq!(Vec3::Z, CubemapFaceDirection::NEG_X.right());
        assert_eq!(Vec3::X, CubemapFaceDirection::Y.right());
        assert_eq!(Vec3::X, CubemapFaceDirection::NEG_Y.right());
        assert_eq!(Vec3::X, CubemapFaceDirection::Z.right());
        assert_eq!(Vec3::NEG_X, CubemapFaceDirection::NEG_Z.right());

        assert_eq!(
            (1, Vec2::new(0.0, 1.0)),
            CubemapDescriptor::get_face_index_and_uv(Vec3::NEG_ONE)
        );
    }

    #[test]
    fn cubemap_face_index() {
        let center = Vec2::splat(0.5);
        let data = [
            (Vec3::X, 0, center),
            (Vec3::NEG_X, 1, center),
            (Vec3::Y, 2, center),
            (Vec3::NEG_Y, 3, center),
            (Vec3::Z, 4, center),
            (Vec3::NEG_Z, 5, center),
        ];
        for (coord, expected_face_index, expected_uv) in data {
            let (seen_face_index, seen_uv) = CubemapDescriptor::get_face_index_and_uv(coord);
            dbg!((coord, seen_face_index, seen_uv));
            assert_eq!(expected_face_index, seen_face_index);
            assert_eq!(expected_uv, seen_uv);
        }
    }
}
