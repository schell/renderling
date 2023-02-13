//! Math helpers for writing shader code.
use glam::{Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};

pub trait Vec3ColorSwizzles {
    fn r(&self) -> f32;
    fn g(&self) -> f32;
    fn b(&self) -> f32;
    fn rgb(&self) -> Vec3;
}

pub trait Vec4ColorSwizzles {
    fn a(&self) -> f32;
    fn rgba(&self) -> Vec4;
}

impl Vec3ColorSwizzles for Vec4 {
    fn r(&self) -> f32 {
        self.x
    }

    fn g(&self) -> f32 {
        self.y
    }

    fn b(&self) -> f32 {
        self.z
    }

    fn rgb(&self) -> Vec3 {
        self.xyz()
    }
}

impl Vec4ColorSwizzles for Vec4 {
    fn a(&self) -> f32 {
        self.w
    }

    fn rgba(&self) -> Vec4 {
        *self
    }
}

impl Vec3ColorSwizzles for Vec3 {
    fn r(&self) -> f32 {
        self.x
    }

    fn g(&self) -> f32 {
        self.y
    }

    fn b(&self) -> f32 {
        self.z
    }

    fn rgb(&self) -> Vec3 {
        (*self).xyz()
    }
}
