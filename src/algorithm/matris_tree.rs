use std::{fmt::{Debug, Display}, future::Future, ops::Add};

use super::matris::RMatris;
#[derive(Debug)]
pub struct MatrisTree<T>{
    left:RMatris<T>,
    right:RMatris<T>,
}
impl<T> MatrisTree<T> where T: Default + 'static +Copy+ Clone + Display+Debug+From<f64>+Add<Output = T>{
    pub async fn new(value: T) -> Self{
        MatrisTree{
            left: RMatris::new(value),
            right: RMatris::new(value),
        }
    }
    pub fn create_rmatris1(value: T) ->RMatris<T>{
        let mut cons_matris = RMatris::new(value);
        let mut current_value = value.clone();
        let factor = 0.1;
        for row in 0..cons_matris.rows() {
            for col in 0..cons_matris.cols() {
                let index = row * cons_matris.cols() + col;
                
                // Değer hesaplaması
                let adjusted_value = value.clone() + T::from(f64::from(index as f64) * factor);
                cons_matris.set(row, col, adjusted_value);
            }
        }
        cons_matris
    }
}