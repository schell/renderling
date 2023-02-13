//! A fixed-max-sized array type that can be shared on GPU and CPU.
#[cfg(target_arch = "spirv")]
pub trait ArrayElement: Default + Copy {}
#[cfg(target_arch = "spirv")]
impl<T: Default + Copy> ArrayElement for T {}

#[cfg(not(target_arch = "spirv"))]
pub trait ArrayElement: encase::ShaderType + Default + Copy {}
#[cfg(not(target_arch = "spirv"))]
impl<T: encase::ShaderType + Default + Copy> ArrayElement for T {}

/// A fixed-size array with a length for tracking
//#[cfg_attr(not(target_arch = "spirv"), derive(encase::ShaderType))]
pub struct Array<T: ArrayElement, const S: usize> {
    length: u32,
    pub elements: [T; S],
}

impl<T: ArrayElement, const S: usize> Array<T, S> {
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.length as usize == S {
            Err(item)
        } else {
            self.elements[self.length as usize] = item;
            self.length += 1;
            Ok(())
        }
    }

    pub fn truncate(&mut self) {
        self.length = 0;
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index as usize)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.elements.get_mut(index as usize)
    }
}
