/*
    feistel-fill-screen  Copyright (C) 2025 Mattia Santaniello 
    This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `show c' for details.
*/

use std::io::{Write,stdout};
use std::{thread,time};

fn nonlinear_transform(num:u8) -> u8 {
    /* it seems that instead of modular behaviour rust launch segfault when you try to make a
     shift that goes overflow, so I had to use a variable with bigger size */
    let safe_num: u16 = num as u16; 
    (((safe_num << 1) + (3*safe_num)) >> 3) as u8 & 63
}

fn feistel_transform(input: u16) -> u16{
    // extract left and right side of coordinates
    let right:u8 = (input & 0x3F) as u8;
    let left:u8 = ((input >> 6) as u8) & 0x3F;
    
    /* in Feistel transformation the original
       left sight must be equal to the left
       side of the output */

    let mut result:u16 = right as u16;

    //place in the right (pun not intended) position the left side
    result = result << 6;

    // apply nonlinear transormation
    result |= (left ^ nonlinear_transform(right)) as u16;
    
    result
}

fn print_at(pattern:char,point:u16) {
    //extract point coordinates
    let x: u8 = (point >> 6) as u8 & 0x3F;
    let y: u8 = point as u8 & 0x3F;

    //print at specified position and make it visible with fflush(stdout)
    print!("\x1B[{};{}H{}",y,x,pattern);
    let _ = stdout().flush().unwrap();
}

fn main() {

    // clear the screen
    print!("\x1B[2J");

    let delay = time::Duration::from_millis(10);
    let tot_rounds = 400;
    
    for point in 0..(64*64) {
       let mut iter = feistel_transform(point);

       /* feistel networks require a certain amount of 
         ounds in order to make the algorithm effective */
       for _ in 0..tot_rounds {
           iter = feistel_transform(iter);
       }

       print_at('+',iter);
       thread::sleep(delay);
    }

    println!("");
}
