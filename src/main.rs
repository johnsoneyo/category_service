mod exercise;
mod lesson;
mod data;

use std::collections::HashMap;
use crate::exercise::add_employee;
//
// fn main() {
//     let mut v = vec![8, 6, 8, 3, 5, 2, 5, 2, 2];
//
//     // let median = get_median(&mut v);
//     // println!("median is {}", median.unwrap_or_default());
//
//     let mode = exercise::get_mode(&mut v);
//     println!("mode is {}", mode.unwrap_or_default());
// }

fn main() {

    add_employee()
}