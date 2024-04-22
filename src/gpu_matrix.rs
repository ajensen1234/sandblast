use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GPUMatrix {
    buffer: &[u8],
    num_rows: u32,
    num_cols: u32
}