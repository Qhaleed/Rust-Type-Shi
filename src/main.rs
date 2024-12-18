#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting: &str = "Nice to meet you";
//     io::stdin()
//     .read_line( &mut name)
//     .expect("Did not recieve input!");
//     println!("Hello, {}! {}!", name.trim_end(), greeting);
// }
//  fn main () {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32= 3.141592;

//     // Shadowing
//     let age = "47";
//     let mut age: u32 = age.trim().parse()
//         .expect("Age wasnt assigned a number");
//     age = age + 1;

//     print!("I'm {} and I want ${}", age, ONE_MIL)
//  }

// fn main () {
//     print!("Max u32: {}", u32::MAX);
//     print!("Max u64: {}", u64::MAX);
//     print!("Max usize: {}", usize::MAX);
//     print!("Max u128: {}", u128::MAX);
//     print!("Max f32: {}", f32::MAX);
//     print!("Max f64: {}", f64::MAX);
// }

// fn main () {
//     let _is_true: bool = true;
//     let my_grade = 'A'
// }

// fn main() {
//     let num_1: f32 = 1.111111111111111;
//     print!("f32: {}", num_1 + 0.111111111111111);
//     let num_2: f64 = 1.111111111111111;
//     print!("f64: {}", num_2 + 0.111111111111111);
// }
// fn main () {
//     let num_3: u32 = 5;
//     let num_4: u32 = 4;
//     print!("5 + 4 = {}", num_3 + num_4);
//     print!("5 - 4 = {}", num_3 - num_4);
// }

// fn main () {
//     let random_num = rand::thread_rng().gen_range(1, 100);
//     print!("Random Number: {}", random_num);
// }   

// fn main() {
//     let age: i32 = 8;
//     if(age>=1) && (age <=18) {
//         println!("Important Birthday");
//     } else if (age == 21 || age == 50) {
//         println!("Important Birthday");
//     } else if (age >= 65) {
//         println!("Not important Birthday")
//     }
// }


// fn main () {
//     let mut my_age: i32 = 47;
//     let can_vote = if my_age >= 18{
//         true
//     } else {
//         false
//     };
//     println!("Can Vote: {}", can_vote);
// }

fn main () {
    let age2 = 8;
    match age2 {
        0..=18 => println!("Important Birthday"),
        21 | 50 => println!("Not so Important Birthday"),
        65..=i32::MAX => println!("Crazy important Birthday"),   
        _ =>println!("Not an Important Birthday ")
    };
}