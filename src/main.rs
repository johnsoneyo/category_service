mod exercise;
mod lesson;

use std::collections::HashMap;


fn main() {
    let mut v = vec![8, 6, 8, 3, 5, 2, 5, 2, 2];

    // let median = get_median(&mut v);
    // println!("median is {}", median.unwrap_or_default());

    let mode = exercise::get_mode(&mut v);
    println!("mode is {}", mode.unwrap_or_default());
}