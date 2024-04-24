use sandblast;
use sandblast::device;
#[test]
fn init_gpu() {
    let new_dev = device::GpuDevice::new();

    assert_eq!(2, 2);
}
