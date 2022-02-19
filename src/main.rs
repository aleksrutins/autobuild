use crate::steps::run_all;

mod format;
mod steps;
mod console;
fn main() {
    println!("Hello, world!");
    run_all(ron::from_str(include_str!("example.ron")).unwrap());
}
