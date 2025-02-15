pub struct RMatris<T> {
    matris: Vec<Vec<T>>,
}

impl<T> RMatris<T> 
where 
    T: Default + 'static + Clone,
{
    pub fn new(rows: usize, cols: usize, value: T) -> Self {
        let matris = vec![vec![value; cols]; rows]; // Use `value` instead of `T::default()`
        RMatris { matris }
    }

    // Get an element from the matrix
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.matris.get(row).and_then(|r| r.get(col))
    }

    // Set an element in the matrix
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if let Some(r) = self.matris.get_mut(row) {
            if col < r.len() {
                r[col] = value;
            }
        }
    }

    // Create a matrix with specified values
    pub fn create_matris_with_values(values: Vec<T>, data_type: &str) -> Self {
        let rows = calc_rows(&values);
        let cols = 3; // Assuming a fixed number of columns; adjust as needed

        match data_type {
            SIGNEDINTEGER => RMatris::new(rows, cols, T::default()), // Use default for initialization
            UNSIGNEDINTEGER => RMatris::new(rows, cols, T::default()),
            _ => {
                println!("Unsupported Data Type");
                RMatris::new(0, 0, T::default()) // Return an empty matrix on unsupported type
            }
        }
    }
}

// Function to calculate the number of rows based on values
pub fn calc_rows<T>(values: &Vec<T>) -> usize {
    if values.is_empty() {
        println!("This matrix is empty");
        return 0;
    }

    if values.len() % 3 == 0 {
        values.len() / 3
    } else {
        (values.len() / 2).saturating_sub(1) // Use saturating_sub to prevent underflow
    }
}
