// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

// Matrix Determinant
// https://www.codewars.com/kata/52a382ee44408cea2500074c/train/rust

fn determinant(matrix: &[Vec<i64>]) -> i64 {
    if matrix.len() == 1 {
        return matrix[0][0];
    }
    if matrix.len() == 2 {
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    }
    let mut det = 0;
    for (i, &value) in matrix[0].iter().enumerate() {
        let sub_matrix: Vec<Vec<i64>> = matrix[1..]
            .iter()
            .map(|row| {
                let mut new_row = row.to_vec();
                new_row.remove(i);
                new_row
            })
            .collect();
        det += if i % 2 == 0 {
            value * determinant(&sub_matrix)
        } else {
            -value * determinant(&sub_matrix)
        };
    }
    return det;
}
