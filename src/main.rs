extern crate rand;

use std::char;
use rand::*;

fn main() {
    let x: u32 = thread_rng().gen_range(0x1F600, 0x1F64F);
    let emoji = char::from_u32(x).unwrap_or('💔');
    println!("{}", emoji);
}
