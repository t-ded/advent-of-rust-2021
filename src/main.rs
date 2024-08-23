mod benchmark;
mod day_15;
mod day_16;

use crate::benchmark::{benchmark_run, print_day, print_header};
extern crate lazy_static;

fn main() {
    benchmark_all!(
        day_15,
        day_16
    );
}