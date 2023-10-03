use std::vec::Vec;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`
    /// * `arr`: the 2D array from which to map into the vector
    pub fn new(width: usize, height: usize, val: T) -> Array2<T> {
        // error checking for at least the dimensions of the 2D array matching the provided width and height
        // assign the width and height attributes to the parameters given
        // take the width * height to find the length of the vector
        // create the vec (data)
        // return an Array2 instance with the initialized data vector
        Array2 {
            width,
            height,
            data: vec![val; width * height],
        }
    }

    /// Takes the elements from the 2D array in row-major order and maps to the data vec
    ///
    /// # Arguments
    ///
    /// * `arr`: the 2D array from which to map into the vector
    pub fn from_row_major(arr: &[&[T]]) -> Self {
        // read through 'arr' and map each element to the vector

        // use this formula: row * width + col, where row and col are indices
        // and width is the width of the 2D array

        /// Invariant satisfied:
        /// every element in the 2D array will be mapped to an element in our
        /// data vector based on its row and column indexes.

        Array2 {
            width,
            height,
            data,
        }
    }

    /// Takes the elements from the 2D array in column-major order and maps to the data vec
    ///
    /// # Arguments
    ///
    /// * `arr`: the 2D array from which to map into the vector
    pub fn from_col_major(arr: &[&[T]]) -> Self {
        // read through 'arr' and map each element to the vector

        // use this formula: col * height + row, where row and col are indices
        // and height is the height of the 2D array

        /// Invariant satisfied:
        /// every element in the 2D array will be mapped to an element in our
        /// data vector based on its row and column indexes.

        Array2 {
            width,
            height,
            data,
        }
    }

    /// Creates a default Array2, where each element is set to a specified `value`
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`
    /// * `arr`: the 2D array from which to map into the vector
    pub fn blank_state(width: usize, height: usize) -> Self {
        // error checking for at least the dimensions of the 2D array matching the provided width and height
        // assign the width and height attributes to the parameters given
        // take the width * height to find the length of the vector
        // create the vec (data) width a set default value for each element
        Array2 {
            width,
            height,
            data,
        }
    }

    /// Retries the value for an element in the vec at the specified (row, col)
    ///
    /// # Arguments
    ///
    /// * `row`: row index in the 2D array
    /// * `col`: col index n the 2D array
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        // error checking if the row and col are within bounds of the width and height
        // calculate the index in row-major order based on the specified (row, col)
        // get the value from that index in the vec
    }

    /// Sets the value for an element in the vec at the specified (row, col)
    ///
    /// # Arguments
    ///
    /// * `row`: row index in the 2D array
    /// * `col`: col index n the 2D array
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        // error checking if the row and col are within bounds of the width and height
        // calculate the index in row-major order based on the specified (row, col)
        // set the value from that index in the vec
    }

    /// Returns an iterator that iterates over the elements in the vector in row-major order
    pub fn iter_row_major(&self) -> std::slice::Iter<T> {
        // simply iterates over one-by-one across the vector
    }

    /// Returns an iterator that iterates over the elements in the vector in column-major order
    pub fn iter_col_major() {
        // uses a step in the iterator to skip the over width of the array
        // each step and increase the initial position in the step each outer-iteration
    }
}
