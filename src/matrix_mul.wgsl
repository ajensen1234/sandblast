// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Also licensed under MIT license, at your choice.

struct GPUMatrix {
    data: array<f32>,
    num_rows: u32,
    num_cols: u32,
    to_tranpose: u32
}

@group(0)
@binding(0)
var<storage, read_write> matrix_0: DataBuf;

@group(0)
@binding(1)
var<storage, read_write> matrix_1: DataBuf;

@group(0)
@binding(2)
var<storage, read_write> output_matrix: DataBuf;

@group(0)
@binding(3)
var<storage, read_write> num_rows_matrix_0: u32;

@group(0)
@binding(4)
var<storage, read_write> num_cols_matrix_0: u32;

@group(0)
@binding(5)
var<storage, read_write> num_cols_matrix_1: u32;

@group(0)
@binding(6)
var<storage, read_write> to_transpose_matrix_0: u32;

@group(0)
@binding(7)
var<storage, read_write> to_transpose_matrix_1: u32;



@compute
@workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var num_rows_matrix_0_after_tranpose: i32;
    // if (to_transpose_matrix_0 == 1) {
    //    let num_rows_matrix_0_after_tranpose = num_cols_matrix_0;
    //} else {
    //    let num_rows_matrix_0_after_tranpose = num_rows_matrix_0;
    //};

    //let output_row = global_id.x % num_rows_matrix_0_after_tranpose;
    //let output_col = global_id.x / num_rows_matrix_0_after_tranpose;

    //let new_val: i32 = 0;
    //let current_matrix_0_col: i32 = 0;

    //var matrix_0_col_boundary: i32;
    //if (to_transpose_matrix_0 == 1) {
    //    let matrix_0_col_boundary = num_rows_matrix_0;
    //} else {
    //    let matrix_0_col_boundary = num_cols_matrix_0;
    //};

    //while (current_matrix_0_col < matrix_0_col_boundary) {
        // Adds product of elements at (output_row, current_matrix_0_col) and (current_matrix_0_col, output_col) to new_val
        //var matrix_0_row_elem_idx: i32;
        //var matrix_1_col_elem_idx: i32;
        //if (to_transpose_matrix_0 == 1) {
            //let matrix_0_row_elem_idx = output_row * num_cols_matrix_0 + current_matrix_0_col;
        //} else {
            //let matrix_0_row_elem_idx = current_matrix_0_col * num_rows_matrix_0 + output_row;
        //};

        //if (to_transpose_matrix_1 == 1) {
            //let matrix_1_col_elem_idx = output_col * num_cols_matrix_1 + current_matrix_0_col;
        //} else {
            //let matrix_1_col_elem_idx = current_matrix_0_col * num_cols_matrix_0 + output_col;
        //};

        //new_val += matrix_0[matrix_0_row_elem_idx] * matrix_1[matrix_1_col_elem_idx];
        //current_matrix_0_col += 1;
    //}

    //output_matrix[global_id.x] = new_val;
}
