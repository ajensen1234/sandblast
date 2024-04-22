use std::time::Instant;
use std::fs:: File ;
use std::io::Read;
use std::marker::PhantomData;
use crate::buffer::GpuBuffer;
use crate::device::GpuDevice;

//TODO: Figure out which things actually need to live in the compute shader vs the things that just need to live as implementations
// This might just need to be a function instead of a struct
// TODO: Remove unwraps and eventually return error codes based on waht we are expecting to pass through
pub struct ComputeShader<T> {
    wgsl_code: String,
    device: GpuDevice,
    bind_group_layout: wgpu::BindGroupLayout,
    compute_pipeline: wgpu::ComputePipeline,
    _marker: PhantomData<T>
}


impl<T> ComputeShader<T>{
    /// This is the code that will house all of the necessary functions to
    /// run a compute shader without any of the necessary boilerplate.
    ///
    /// You should simply be able to call `run()` while passing the correct files, buffers, and devices
    /// into the protocol, and everything boilerplate will be abstracted away.
    /// Voila

    pub fn new(device: GpuDevice, wgsl_file: &str) -> Self {
        let mut file = File::open(wgsl_file).unwrap();
        let mut wgsl_code = String::new();
        file.read_to_string(&mut wgsl_code).unwrap();

        let shader_module = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(wgsl_code.clone().into()),
        });

        // Find the number of parameters the wgsl accepts
        // TODO: Validate wgsl bind group binding enumeration schema
        let param_matches: Vec<_> = wgsl_code.match_indices("@binding").collect();
        let num_params = param_matches.len();

        // Create a bindgroup layout in accordance with those parameters
        let entries_vec: Vec<wgpu::BindGroupLayoutEntry> = (0..num_params).map(|idx| wgpu::BindGroupLayoutEntry {
                binding: idx as u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
        }).collect();
        let entries_slice = entries_vec.as_slice();
        
        let bind_group_layout = device.device.create_bind_group_layout(&&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &entries_slice,
        });

        let compute_pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let compute_pipeline = device.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &shader_module,
            entry_point: "main",
        });

        return Self {
            wgsl_code,
            device,
            bind_group_layout,
            compute_pipeline,
            _marker: PhantomData::<T>,
        }
    }

    pub async fn run(&self, buffers: &[GpuBuffer<T>], dispatch_groups: (u32, u32, u32)) {
        let entries_vec: Vec<wgpu::BindGroupEntry> = (0..buffers.len()).map(|idx| wgpu::BindGroupEntry {
            binding: idx as u32,
            resource: buffers[idx].buffer.as_entire_binding(),
        }).collect();
        let entries_slice = entries_vec.as_slice();

        let bind_group = self.device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &entries_slice
        });

        let mut encoder = self.device.device.create_command_encoder(&Default::default());

        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);

            let (x, y, z) = dispatch_groups;
            cpass.dispatch_workgroups(x, y, z);
        }

        self.device.queue.submit(Some(encoder.finish()));
    }
}
