use bytemuck::{Pod, cast_slice};
use nalgebra::DMatrix;

pub fn matrix_to_casted_array<U, T>(matrix: &DMatrix<U>) -> &[T]
where
    T: Pod,
    U: Pod
{
    let matrix_array = matrix.as_slice().try_into().unwrap();

    let casted_matrix_array: &[T] = cast_slice(matrix_array);

    return casted_matrix_array
}