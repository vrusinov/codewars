// SPDX-FileCopyrightText: 2025 Vladimir Rusinov <vladimir.rusinov@gmail.com>
// SPDX-License-Identifier: Apache-2.0

// Tic-Tac-Toe Checker
// https://www.codewars.com/kata/525caa5c1bf619d28c000335/train/rust

// Returns the winner of the game if there is one in a given line.
// Otherwise returns -1 if there is an empty cell in the line, or 0 if there are not.
fn check_line(a: u8, b: u8, c: u8) -> i8 {
    if a != 0 && a == b && b == c {
        return i8::try_from(a).unwrap();
    }
    if a == 0 || b == 0 || c == 0 {
        return -1;
    }
    0
}

fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    let mut has_empty: bool = false;
    // Check rows & columns:
    for i in 0..3 {
        let row_check = check_line(board[i][0], board[i][1], board[i][2]);
        if row_check > 0 {
            return row_check;
        }
        if row_check == -1 {
            has_empty = true
        }
        let col_check = check_line(board[0][i], board[1][i], board[2][i]);
        if col_check > 0 {
            return col_check;
        }
        if col_check == -1 {
            has_empty = true
        }
    }
    // Check diagonals:
    let diag1_check = check_line(board[0][0], board[1][1], board[2][2]);
    if diag1_check > 0 {
        return diag1_check;
    }
    if diag1_check == -1 {
        has_empty = true
    }
    let diag2_check = check_line(board[0][2], board[1][1], board[2][0]);
    if diag2_check > 0 {
        return diag2_check;
    }
    if diag2_check == -1 {
        has_empty = true
    }
    if has_empty {
        return -1;
    } else {
        return 0;
    }
}
