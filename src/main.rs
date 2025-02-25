use algorithm::{matris::{self}, matris_tree::MatrisTree};

mod algorithm;
mod ai;
fn main() {
    let matris1 = MatrisTree::create_rmatris1(5.0);
    println!("{:?}",matris1);
}
