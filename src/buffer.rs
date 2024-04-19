use bytemuck;
use std::marker::PhantomData;
use wgpu::{self, util::DeviceExt};

pub struct GpuBuffer<T> {
    buffer: wgpu::Buffer,
    size: usize,
    _marker: PhantomData<T>,
}
// TODO: Better error handling (Pretty sure we should be using the `anywho/how(?)` crate for this)
// TODO: Make sure that the generic types passed in are good
// TODO: Validate all the assumptions about the permissions that we give.
// permissions might actually be something that some users might want control over. See if we can overload `new` to make it super permissive with no usage passed in, but fully permissive with nothing passed on

impl<T: bytemuck::Pod + bytemuck::Zeroable> GpuBuffer<T> {
    /// This will create a sandblast structure called a `GpuBuffer`
    /// The types that are allowed inside the templated function are only those
    /// that can be initialized with all zeros (bytemuck::Zeroable) or those
    /// that are 'plain old data' (bytemuck::Pod)
    ///
    /// This will abstract away all the necesity to create different bind groups, as well as abstracting away the need for the different types of data that you want to shove onto a GPU
    pub fn new(device: &wgpu::Device, data: &[T]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::INDEX
                | wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::INDIRECT
                | wgpu::BufferUsages::MAP_READ
                | wgpu::BufferUsages::MAP_WRITE,
        });
        return Self {
            buffer,
            size: data.len() * std::mem::size_of::<T>(), // TODO: Check if we need to change this to the bytemuck representation
            _marker: PhantomData, //TODO: Make sure that this isnt going to mess things up  sdf
        };
    }
}
