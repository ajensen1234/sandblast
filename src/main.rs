use sandblast::buffer::GpuBuffer;
use sandblast::device::GpuDevice;
use sandblast::shader::ComputeShader;
use sandblast::matrix_serialization_utils::matrix_to_casted_array;
use nalgebra::DMatrix;


fn main() {
    let device = pollster::block_on(GpuDevice::new());

    // Example element-wise multiplication and division sequence
    let num_rows: usize = 25;
    let num_cols: usize = 25;

    let matrix_base = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    let matrix_to_multiply = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    let matrix_to_divide = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));

    let shader = ComputeShader::<u8>::new(device, "src/element_mul_div.wgsl");

    pollster::block_on(shader.run(&[matrix_base, matrix_to_multiply, matrix_to_divide], (num_rows as u32, num_cols as u32, 1)));

    // A * B * C
    // Example of chained, non-inplace operations

    // let a = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    // let b = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    // let a_b = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    // let c = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    // let a_b_c = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));

    // let matrix_multiply_shader = ComputeShader::<u8>::new(device, "src/matrix_mul.wgsl");

    // pollster::block_on(shader.run(&[a, b, a_b], (num_rows as u32, num_cols as u32, 1)));
    // pollster::block_on(shader.run(&[a_b, c, a_b_c], (num_rows as u32, num_cols as u32, 1)));
}