mod data;
mod exercise;
mod lesson;

use std::fs::File;
use std::io::{self, Read};
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}


#[derive(Debug)]
struct Person<'a> {
    name: &'a str
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = Person {
        name: first_sentence,
    };

    println!("i is {i:?}");
}