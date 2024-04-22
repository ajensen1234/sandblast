use sandblast::buffer::GpuBuffer;
use sandblast::device::GpuDevice;
use sandblast::shader::ComputeShader;
use sandblast::matrix_serialization_utils::matrix_to_casted_array;
use nalgebra::DMatrix;
use bytemuck::cast_slice;



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

    // let a = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    // let b = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    // let a_b = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    // let c = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));
    // let a_b_c = GpuBuffer::<u8>::new(&device, matrix_to_casted_array(&DMatrix::<f32>::new_random(num_rows, num_cols).abs()));

    // let num_rows_matrix_0 = GpuBuffer::<u8>::new(&device, cast_slice(&[num_rows]));
    // let num_cols_matrix_0 = GpuBuffer::<u8>::new(&device, cast_slice(&[num_cols]));
    // let num_cols_matrix_1 = GpuBuffer::<u8>::new(&device, cast_slice(&[num_cols]));

    // let to_tranpose_matrix_0 = GpuBuffer::<u8>::new(&device, cast_slice(&[false]));
    // let to_tranpose_matrix_1 = GpuBuffer::<u8>::new(&device, cast_slice(&[false]));

    // let shader = ComputeShader::<u8>::new(device, "src/matrix_mul.wgsl");

    // pollster::block_on(shader.run(&[matrix_base, matrix_to_multiply, matrix_to_divide, num_rows_matrix_0, num_cols_matrix_0, num_cols_matrix_1, to_tranpose_matrix_0, to_tranpose_matrix_1], (num_rows as u32, num_cols as u32, 1)));
}