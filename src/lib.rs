#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod koopa;

#[skyline::main(name = "midbus")]
pub fn main() {
    koopa::install();
}