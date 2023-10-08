pub struct Array2<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Array2<T> {
        // Row Major constructor
    pub fn from_row_major(
        width: usize,
        height: usize,
        elements: Vec<T>,
    ) -> Result<Self, &'static str> {
        // ensure the board has valid number of elements
        if elements.len() != width * height {
            return Err("Invalid number of elements");
        }
        // set a empty vec which will be filled with the inputted elements
        let mut data = Vec::with_capacity(width * height);
        //  fill the vec
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                data.push(elements[index].clone());
            }

        // construct our instance of array2 with the width, height, and vec 
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }

    // Col-major constructor
    pub fn from_col_major(
        width: usize,
        height: usize,
        elements: Vec<T>,
    ) -> Result<Self, &'static str> {
        // ensure the board has valid number of elements
        if elements.len() != width * height {
            return Err("Invalid number of elements");
        }
        // set a empty vec which will be filled with the inputted elements
        let mut data = Vec::with_capacity(width * height);
        //  fill the vec
        for y in 0..width {
            for x in 0..height {
                let index = y * width + x;
                data.push(elements[index].clone());
            }
        }
        // construct our instance of array2 with the width, height, and vec 
        Ok(Self {
            width,
            height,
            data,
        })
    }
    // blank-state constructor if needed
    pub fn blank_state(width: usize, height: usize, val: T) -> Self {
        let data = vec![val; width * height];

        Self {
            width,
            height,
            data,
        }
    }

    // iterates over the rows
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y, self.get(x, y))))
    }

    //iterates over the columns skipping by the width to ensure we are going by column

    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width)
            .map(move |c| (c, self.data.iter().skip(c)))
            .flat_map(move |(c, col)| {
                col.step_by(self.width)
                    .enumerate()
                    .map(move |(r, val)| (c, r, val))
            })
    }

    //function to return element from a pair of coordinates
    pub fn get(&self, x: usize, y: usize) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.data[x + y * self.width]
    }
    // way to access width
    pub fn width(&self) -> usize {
        self.width
    }
    // way to access height
    pub fn height(&self) -> usize {
        self.height
    }
}
