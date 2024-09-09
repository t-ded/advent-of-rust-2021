mod benchmark;

mod day_15;
mod day_16;
mod day_17;
mod day_18;

mod array_2d;


use crate::benchmark::{benchmark_run, print_day, print_header};
extern crate lazy_static;

fn main() {
    benchmark_all!(
        day_15,
        day_16,
        day_17,
        day_18
    );
}