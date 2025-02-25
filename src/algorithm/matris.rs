use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct RMatris<T> {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) value: Vec<Vec<T>>,
}

impl<T> RMatris<T> 
where 
    T: Default + 'static + Clone + Display + Debug,
{
    pub fn new(value: T) -> Self {
        let rows =3; // Static value for usage
        let cols = 3;// Static value for usage
        let value = vec![vec![value; cols]; rows];
        RMatris { rows, cols, value }
    }
    // Get an element from the matrix
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.value.get(row).and_then(|r| r.get(col))
    }

    // Set an element in the matrix
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row < self.rows && col < self.cols {
            self.value[row][col] = value;
        }
    }

    // Get number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    // Get number of columns
    pub fn cols(&self) -> usize {
        self.cols
    }
}
/* 
This Function is optional , with in works different multiple values

// Function to calculate the number of rows and columns based on values
pub fn calc_matris_size<T>(values: &[T]) -> (usize, usize) {
    let len = values.len();
    if len == 0 {
        println!("This matrix is empty");
        return (0, 0);
    }

    let rows;
    let cols;

    if len % 3 == 0 {
        rows = len / 3;
        cols = 3;
    } else {
        rows = (len / 2).saturating_sub(1);
        cols = len / rows;
    }
        // why this works last in first out ? wtf
    (cols,rows)
}
*/