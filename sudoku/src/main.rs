use array2::Array2;
use csc411_image::{GrayImage, Read};
use std::env;
use std::process;

fn main() {
    // Create an Array2 instance with some sample data.
    // Set up command line argument
    let input = env::args().nth(1);
    // assert only one argument is given in the command line
    assert!(
        env::args().len() == 2 || env::args().len() == 1,
        "Too many arguments!"
    );
    // set image with read function
    let img = GrayImage::read(input.as_deref()).unwrap();
    // get our values from grayimage to the correct types for our structure
    let height2 = img.height.try_into().unwrap();
    let width2 = img.width.try_into().unwrap();
    let usize_vec: Vec<usize> = img.pixels.iter().map(|gray| gray.value as usize).collect();
    // Set up Array2 instance with the pgm file
    let try_2 = Array2::from_col_major(width2, height2, usize_vec).unwrap();
    // check if pgm file is valid sudoku
    if valid_sudoku(&try_2) {
        process::exit(0);
    } else {
        process::exit(1);
    }

}

fn valid_sudoku(board: &Array2<usize>) -> bool {
    if board.width() != 9 || board.height() != 9 {
        return false;
    }

    // Check rows and columns of pgm file (9 rows x 9 columns)
    for i in 0..9 {
    // set up two arrays for row and column values. They represent numbers 1-9 and are all intilized to false
        let mut row_set = [false; 9];
        let mut col_set = [false; 9];
    // Loop through the board with our iter_row function 
        for (x, y, &value) in board.iter_row_major().filter(|&(_, _, &v)| v != 0) {
            if y == i {
    // if the number in the spot is set to true, that means the number has already been seen, so return false
                if row_set[value - 1] {
                    return false; // Duplicate value in the same row
                }
    // set value to true, because if it is seen again it will get flagged
                row_set[value - 1] = true;
            }
    // repeat above process but with column values
            if x == i {
                if col_set[value - 1] {
                    return false; // Duplicate value in the same column
                }
                col_set[value - 1] = true;
            }
        }
    }

    // Check 3x3 subgrids of pgm file (we step by three so we check the next subgrid after each iteration)
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            if !valid_subgrid(board, i, j) {
                return false; // Invalid subgrid
            }
        }
    }

    true // If all checks pass, the Sudoku is valid and we return true
}

fn valid_subgrid(board: &Array2<usize>, start_row: usize, start_col: usize) -> bool {
    // set the array representing numbers 1-9 all set to false, for each 3x3 subgrid
    let mut subgrid_set = [false; 9];

    for i in 0..3 {
        for j in 0..3 {
    // get the value in the specified indices from array2 get function
            let value = board.get(start_row + i, start_col + j);

            if *value == 0 {
                continue; // Skip empty cells that are zero
            }

            if subgrid_set[*value - 1] {
                return false; // Duplicate value in subgrid because if that value has already been set to true; it has been seen before
            }

            subgrid_set[*value - 1] = true;
        }
    }

    true // If all checks pass, the subgrid is valid so return true
}
