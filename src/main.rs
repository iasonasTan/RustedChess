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
        let played: bool = match inp.trim() {
            "quit" => exit(0),
            _ => play(&inp, &mut piec),
        };
        
        if played {
            white_plays = !white_plays;
        }
        println!();
    }
}

fn play(code: &String, pieces: &mut [Entity; 32]) -> bool{
    let poss: Vec<&str> = code.split(',').collect();
    let ox: i32 = to_i32(poss[0]);
    let oy: i32 = to_i32(poss[1]);
    let dx: i32 = to_i32(poss[2]);
    let dy: i32 = to_i32(poss[3]);
    if ox == -1 || oy == -1 || dx == -1 || dy == -1 {
        println!("Unknown coordinates...");
        return false;
    }
    let res: bool = move_piece(
        ox as u32,
        oy as u32,
        dx as u32,
        dy as u32,
        pieces
    );
    if !res {
        println!("You can't move there!");
    }
    return res;
}

fn to_i32(s: &str) -> i32 {
    match s.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => -1,
    }
}