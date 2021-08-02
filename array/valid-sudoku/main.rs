// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/769/

// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
//
// Each row must contain the digits 1-9 without repetition.
// Each column must contain the digits 1-9 without repetition.
// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
//
// Note:
// A Sudoku board (partially filled) could be valid but is not necessarily solvable.
// Only the filled cells need to be validated according to the mentioned rules.

use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let result = true;

        let mut horizontal = HashSet::new();
        let mut vertical = HashSet::new();
        let mut square = HashSet::new();

        for i in 0..9 {
            for j in 0..9 {
                let square_row = i - (i % 3);
                let square_col = j - (j % 3);
                for r in 0..3 {
                    for c in 0..3 {
                        let curr = board[r + square_row][c + square_col];
                        if curr != '.' {
                            if square.contains(&curr) {
                                return false;
                            } else {
                                square.insert(curr);
                            }
                        }
                    }
                }
                square.clear();

                if board[i][j] != '.' {
                    if horizontal.contains(&board[i][j]) {
                        return false;
                    } else {
                        horizontal.insert(board[i][j]);
                    }
                }

                if board[j][i] != '.' {
                    if vertical.contains(&board[j][i]) {
                        return false;
                    } else {
                        vertical.insert(board[j][i]);
                    }
                }
            }

            vertical.clear();
            horizontal.clear();
        }

        result
    }
}