#![allow(unused)]

use core::num;
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


// fn main () {dasdasdasdasdas
//     let mut my_age: i32 = 47;
//     let can_vote = if my_age >= 18{
//         true
//     } else {
//         false
//     };
//     println!("Can Vote: {}", can_vote);
// }

// fn main () {
//     let age2 = 8;
//     match age2 {
//         0..=18 => println!("Important Birthday"),
//         21 | 50 => println!("Not so Important Birthday"),
//         65..=i32::MAX => println!("Crazy important Birthday"),   
//         _ =>println!("Not an Important Birthday ")
//     }; 
// }

// fn main () {
//     let my_age = 20;
//     println!("My age is {}", my_age); 
//     let votingAge = 18;
//     match my_age.cmp(&votingAge) {
//         Ordering::Less => println!("Cant vote"),
//         Ordering::Greater => println!("Can vote"),
//         Ordering::Equal => println!("Can vote"),
        
//     }
// }

// Arrays with RUST!!
fn main () {
    // let arr_1 = [1,2,3,4,5];
    // println!("First element of the array: {}",arr_1.len());
    // let mut arr_1_index = 0;
    // while(arr_1_index != arr_1.len()) {
    //     println!("{}", arr_1[arr_1_index]);
    //     arr_1_index += 1;
    // }

    // let my_tuple: (u8,String, f64) = (47, "Res".to_string(), 4.0);
    // println!("Here is my tuple {}", my_tuple.1);


    // // Strings with Rust
    //  // There are 2 types of strings
    // // 1. String : Vector of bytes that can be changed
    // // 2. &str : Points to the string and allows for viewing
    // let mut st1 = String::new();
    // st1.push('a');

    // st1.push_str(" word");

    // for word in st1.split_whitespace(){
    //     println!("{}", word);
    // }    
    // let arr_2 = [1,2,3,4,5,6,7,8,9];
    // let mut loop_idx: usize = 0;
    // loop {
    //     if arr_2[loop_idx] %2 == 0 {
    //         loop_idx +=1;
    //         continue;
    //     }
    //     if  arr_2[loop_idx] > 9 {
    //         break;
    //     }
    //     println!("Val : {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }
    // let mut user_name = "Res";
    // println!("Hello {}!", user_name );
    // println!("Please Enter your Age");

    // let mut user_age = String::new();
    // io::stdin().read_line(&mut user_age).expect("Failed to recieve line");

    // println!("Name: {}", user_name);    
    // println!("Age: {}", user_age.trim());    
    
//     let age:u32 = 30;
//     let price:f64 = 99.99;
//     let is_active = true;
//     let initials: char = 'T';
//     let message : &str = "Hello Rust!";
//     let name = String::from("Resly Kadiri");

//     println!("Name: {}", name);
//     println!("Age: {}", age);
//     println!("Black: {}", is_active);
//     println!("Initials: {}", initials);
//     println!("Message: {}", message);
//     println!("Ratings: {}", price);


//     // Try to write a small program that declares variables of different data types (integer, float, boolean, char, string)
//     //  and prints their values to the console. This will help solidify your understanding of these basic types. 
//     // Once you've attempted that, we can move on to more complex topics like control flow (if/else, loops) and ownership.

//     let my_integer:i32 = 10;
//     let my_float:f32 = 10.1;
//     let my_bool = true;
//     let my_char = 'r';
//     let my_string = String::from("Res");

//     for number in 1..4 {
//         println!("{}", number);
//     }
//     println!("LIFTOFF!!!!!!!");

//     let a = [1,2,3,4,5,6,7];

//     for element in a.iter(){
//         println!("Value is: {}", element);
//     }

//     // Write a program that takes an integer as input from the user. 
//     // Use an if statement to check if the number is positive, negative, or zero. 
//     // Print an appropriate message for each case.
//     // Then, use a loop (either while or for) to print the numbers from 1 to the absolute value of the input number.

//     println!("Give me a number");
//     let mut user_number = String::new();
//     io::stdin().read_line(&mut user_number)
//     .unwrap();
//     let trimmed_number = user_number.trim();
//     let target_number:i32 = trimmed_number.parse().unwrap();
//     println!("Your number is: {}", target_number);
//     if target_number > 0 {
//         println!("Number is positive");
//     } else if target_number < 0 {
//         println!("Number is negative");
//     }
//     else if target_number == 0{
//         println!("Number is zero");
//     }

//     for numbers in 1..=target_number {
//         println!("{}", numbers);
//     }



//     let s1 = String::from("Hello");
//     let s2 = s1;

//     println!("s1 = {}", s2);


let s1 = String::from("Hello");
let s2 = s1;

println!("s1 = {}", s2);

let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world!");

}