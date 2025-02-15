use super::{matris::RMatris, probability::{self, Probability}};

pub struct MatrisTree<T>{
    parent:Probability,
    left:RMatris<T>,
    right:RMatris<T>,
}