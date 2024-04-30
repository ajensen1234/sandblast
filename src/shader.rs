use std::time::Instant;
use std::fs:: File ;
use std::io::Read;
use std::marker::PhantomData;
use crate::buffer::GpuBuffer;
use crate::device::GpuDevice;
use crate::binding_info::stubbed_generate_binding_info_from_wgsl;
use nalgebra::DMatrix;

//TODO: Figure out which things actually need to live in the compute shader vs the things that just need to live as implementations
// This might just need to be a function instead of a struct
// TODO: Remove unwraps and eventually return error codes based on waht we are expecting to pass through
pub struct ComputeShader<'a, T> {
    wgsl_code: String,
    device: &'a GpuDevice,
    bind_group_layout: wgpu::BindGroupLayout,
    compute_pipeline: wgpu::ComputePipeline,
    _marker: PhantomData<T>
}


impl<'a, T> ComputeShader<'a, T>{
    /// This is the code that will house all of the necessary functions to
    /// run a compute shader without any of the necessary boilerplate.
    ///
    /// You should simply be able to call `run()` while passing the correct files, buffers, and devices
    /// into the protocol, and everything boilerplate will be abstracted away.
    /// Voila

    pub fn new(device: &'a GpuDevice, wgsl_file: &str) -> Self {
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

        let binding_info = stubbed_generate_binding_info_from_wgsl(&wgsl_file, &wgsl_code);

        // Create a bindgroup layout in accordance with those parameters
        let entries_vec: Vec<wgpu::BindGroupLayoutEntry> = binding_info.iter().map(|info| wgpu::BindGroupLayoutEntry {
                binding: info.binding as u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: if &*info.buffer_type == "storage" {wgpu::BufferBindingType::Storage { read_only: info.read_only }} else {wgpu::BufferBindingType::Uniform},
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
        }).collect();
        let entries_slice = entries_vec.as_slice();
        println!("{}", entries_vec.len());
        
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

    pub async fn run(&self, buffers: &[&GpuBuffer<T>], dispatch_groups: (u32, u32, u32), output_buffer: &GpuBuffer<T>) -> Vec<f32> {
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

        // TODO: Replace this hardcode
        if buffers.len() == 7 {
            encoder.copy_buffer_to_buffer(&(buffers[3].buffer), 0, &(output_buffer.buffer), 0, output_buffer.buffer.size());
        } else if buffers.len() == 5 {
            encoder.copy_buffer_to_buffer(&(buffers[2].buffer), 0, &(output_buffer.buffer), 0, output_buffer.buffer.size());
        } else if buffers.len() == 3 {
            encoder.copy_buffer_to_buffer(&(buffers[0].buffer), 0, &(output_buffer.buffer), 0, output_buffer.buffer.size());
        }

        self.device.queue.submit(Some(encoder.finish()));

        let buf_slice = output_buffer.buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        println!("pre-poll {:?}", std::time::Instant::now());
        self.device.device.poll(wgpu::Maintain::Wait);
        println!("post-poll {:?}", std::time::Instant::now());
        if let Some(Ok(())) = receiver.receive().await {
            let data_raw = &*buf_slice.get_mapped_range();
            let data: &[f32] = bytemuck::cast_slice(data_raw);
            return (&*data).to_vec()
        }

        return [].to_vec();

    }
}
