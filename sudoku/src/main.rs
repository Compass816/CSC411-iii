use array2::Array2;
use csc411_image::{GrayImage, Read};
use std::env;
use std::process;

fn main() {
    // Create an Array2 instance with some sample data.

    let input = env::args().nth(1);
    // assert only one argument is given
    assert!(
        env::args().len() == 2 || env::args().len() == 1,
        "Too many arguments!"
    );

    // set image with read function
    let img = GrayImage::read(input.as_deref()).unwrap();
    let height2 = img.height.try_into().unwrap();
    let width2 = img.width.try_into().unwrap();
    let usize_vec: Vec<usize> = img.pixels.iter().map(|gray| gray.value as usize).collect();
    let try_2 = Array2::from_col_major(width2, height2, usize_vec).unwrap();

    if is_valid_sudoku(&try_2) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}



fn is_valid_sudoku(board: &Array2<usize>) -> bool {
    if board.width() != 9 || board.height() != 9 {
        return false;
    }

    // Check rows and columns
    for i in 0..9 {
        let mut row_set = [false; 9];
        let mut col_set = [false; 9];

        for (x, y, &value) in board.iter_row_major().filter(|&(_, _, &v)| v != 0) {
            if y == i {
                if row_set[value - 1] {
                    return false; // Duplicate value in the same row
                }
                row_set[value - 1] = true;
            }

            if x == i {
                if col_set[value - 1] {
                    return false; // Duplicate value in the same column
                }
                col_set[value - 1] = true;
            }
        }
    }

    // Check 3x3 subgrids
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            if !is_valid_subgrid(board, i, j) {
                return false; // Invalid subgrid
            }
        }
    }

    true // If all checks pass, the Sudoku is valid
}

fn is_valid_subgrid(board: &Array2<usize>, start_row: usize, start_col: usize) -> bool {
    let mut subgrid_set = [false; 9];

    for i in 0..3 {
        for j in 0..3 {
            let value = board.get(start_row + i, start_col + j);

            if *value == 0 {
                continue; // Skip empty cells
            }

            if subgrid_set[*value - 1] {
                return false; // Duplicate value in subgrid
            }

            subgrid_set[*value - 1] = true;
        }
    }

    true // If all checks pass, the subgrid is valid
}
