#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;

mod _1;
mod _10;
mod _11;
mod _12;
mod _2;
mod _3;
mod _4;
mod _5;
mod _6;
mod _7;
mod _8;
mod _9;

pub mod util;

lazy_static! {
    static ref EXERCISES: BTreeMap<usize, fn()> = {
        let mut m = BTreeMap::new();
        m.insert(1, _1::solve as fn());
        m.insert(2, _2::solve as fn());
        m.insert(3, _3::solve as fn());
        m.insert(4, _4::solve as fn());
        m.insert(5, _5::solve as fn());
        m.insert(6, _6::solve as fn());
        m.insert(7, _7::solve as fn());
        m.insert(8, _8::solve as fn());
        m.insert(9, _9::solve as fn());
        m.insert(10, _10::solve as fn());
        m.insert(11, _11::solve as fn());
        m.insert(12, _12::solve as fn());
        m
    };
}

fn main() {
    match std::env::args().len() {
        1 => {
            for number in EXERCISES.keys() {
                run(number);
            }
        }
        2 => {
            let number = std::env::args().nth(1).unwrap();
            if let Ok(number) = number.parse::<usize>() {
                run(&number);
            } else {
                eprintln!("Invalid exercise `{}`", number);
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Invalid arguments");
            std::process::exit(1);
        }
    }
}

fn run(number: &usize) {
    if let Some(f) = EXERCISES.get(number) {
        print!("Exercise {}: ", number);
        f();
        println!();
    } else {
        eprintln!("Day {} does not exist", number);
        std::process::exit(1);
    }
}
