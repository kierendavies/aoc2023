#![feature(iterator_try_collect)]

use std::io;

fn main() {
    let input: Vec<_> = io::stdin().lines().try_collect().unwrap();

    let nums = input.iter().map(|line| {
        let a = line.chars().find(|c| c.is_numeric()).unwrap();
        let b = line.chars().rfind(|c| c.is_numeric()).unwrap();

        let num: u32 = format!("{a}{b}").parse().unwrap();
        num
    });

    let sum: u32 = nums.sum();

    println!("{sum}");
}
