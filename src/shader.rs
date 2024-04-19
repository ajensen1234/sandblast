use std::fs::{self, File };
use std::io::Read;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::str;
use crate::buffer::GpuBuffer;

//TODO: Figure out which things actually need to live in the compute shader vs the things that just need to live as implementations
// This might just need to be a function instead of a struct
// TODO: Remove unwraps and eventually return error codes based on waht we are expecting to pass through
pub struct ComputeShader<T> {
    _marker: PhantomData<T>
}


impl<T> ComputeShader<T>{
    pub async fn run(device: wgpu::Device, wgsl_file: String, buffers: &[GpuBuffer<T>], dispatch_groups: (usize, usize, usize)) {
        let mut file = File::open(wgsl_file).unwrap();
        let mut wgsl_code = String::new();
        file.read_to_string(&mut wgsl_code).unwrap();
        let cs_mod = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(wgsl_code.into()),
        });
    }
}
