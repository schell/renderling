// SPDX-FileCopyrightText: 2024 Schell Scivally <efsubenovex@gmail.com>>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crabslab::SlabItem;

#[derive(Default, Debug, Clone, Copy, PartialEq, SlabItem)]
pub struct DrawIndirect {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}
