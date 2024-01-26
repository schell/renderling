use crabslab::SlabItem;

#[derive(Default, Debug, Clone, Copy, PartialEq, SlabItem)]
pub struct DrawIndirect {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}
