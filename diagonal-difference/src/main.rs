use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::num;

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut diagonal1 = 0;
    let mut diagonal2 = 0;
    let dimension = arr.len();

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if i == j {
                diagonal1 = diagonal1 + arr[i][j];
            }
            if i + j == dimension {
                diagonal2 = diagonal2 + arr[i][j];
            }
        }
    }
    return (diagonal2 - diagonal1).abs();
}

fn main() {
    let lala = [
        [11, 2, 4],
        [4, 5, 6],
        [10, 8, -12]
    ];

    diagonalDifference(&lala);
}
