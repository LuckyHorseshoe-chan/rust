use crate::garden::vegetables::Asparagus;
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

use std::io::{self, Write};

pub mod garden;
pub mod vegetables;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}