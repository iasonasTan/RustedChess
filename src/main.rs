#![warn(dead_code)]
#![warn(unused_imports)]
#![warn(unknown_lints)]

mod utils;
mod board;
mod pieces;

use utils::Entity;
use pieces::*;
use board::draw;

use std::io;
use std::process::exit;

fn main() {
    println!("Type 'quit' to stop application.");
    println!("Type 'original cords,destination cords' to play.");
    println!("Cords are typed this way: x,y");
    println!("X is the distance from the left side and Y is the distance from the top");
    let mut piec: [Entity; 32] = pieces::default();
    let mut white_plays: bool = true;
    loop {
        draw(&mut piec, white_plays);

        println!("Enter your move: ");
        let mut inp: String = String::new();
        let _  = io::stdin().read_line(&mut inp);
        match inp.trim() {
            "quit" => exit(0),
            _ => play(&inp, &mut piec),
        }
        
        white_plays = !white_plays;
        println!();
    }
}

fn play(code: &String, pieces: &mut [Entity; 32]) {
    let poss: Vec<&str> = code.split(',').collect();
    let ox: u32 = to_u32(poss[0]);
    let oy: u32 = to_u32(poss[1]);
    let dx: u32 = to_u32(poss[2]);
    let dy: u32 = to_u32(poss[3]);
    let res: bool = move_piece(ox, oy, dx, dy, pieces);
    if !res {
        println!("You can't move there!");
    }
}

fn to_u32(s: &str) -> u32 {
    match s.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("This was not a number.");
            exit(1);
        },
    }
}