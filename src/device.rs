pub struct GpuDevice {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

// Boiler-plate device instantiation with a 1:1 device to queue relation
impl GpuDevice{
    pub async fn new() -> Self {
        let instance = wgpu::Instance::default();

        let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await.unwrap();

        let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();

        return Self{
            device,
            queue,
        }
    }
}