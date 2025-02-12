pub struct RMatris<T> {
    matris: Vec<Vec<T>>,
}

impl<T> RMatris<T> where T: Default + 'static +Clone{
    pub fn new(rows: usize, cols: usize) -> Self {
        let matris = vec![vec![T::default(); cols]; rows];
        RMatris { matris }
    }

    // Matrisin bir elemanını almak için bir metod
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.matris.get(row).and_then(|r| r.get(col))
    }

    // Matrisin bir elemanını ayarlamak için bir metod
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if let Some(r) = self.matris.get_mut(row) {
            if col < r.len() {
                r[col] = value;
            }
        }
    }
}
