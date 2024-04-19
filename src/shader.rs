use std::fs:: File ;
use std::io::Read;
use std::marker::PhantomData;
use crate::buffer::GpuBuffer;

//TODO: Figure out which things actually need to live in the compute shader vs the things that just need to live as implementations
// This might just need to be a function instead of a struct
// TODO: Remove unwraps and eventually return error codes based on waht we are expecting to pass through
pub struct ComputeShader<T> {
    _marker: PhantomData<T>
}


impl<T> ComputeShader<T>{
    /// This is the code that will house all of the necessary functions to
    /// run a compute shader without any of the necessary boilerplate.
    ///
    /// You should simply be able to call `run()` while passing the correct files, buffers, and devices
    /// into the protocol, and everything boilerplate will be abstracted away.
    /// Voila
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
