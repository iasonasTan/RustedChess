use std::io;
use crate::utils::Entity;
use crate::pieces::is_white_piece;

pub const PROMOTE_POS_WHITE: u32 = 1;
pub const PROMOTE_POS_BLACK: u32 = 8;

const CHOOSABLE_PIECES_WHITE: [char; 4] = ['♕', '♖', '♗', '♘'];
const CHOOSABLE_PIECES_BLACK: [char; 4] = ['♛', '♜', '♝', '♞'];

pub fn check_promotion(pawn: &mut Entity) {
    let is_white = is_white_piece(&pawn);
    if is_white && PROMOTE_POS_WHITE == pawn.y ||
        !is_white && PROMOTE_POS_BLACK == pawn.y {

        promote(pawn);
    }
}

fn promote(pawn: &mut Entity) {
    let color: bool = is_white_piece(&pawn);
    let piece_type: char = ask_for_type(color);
    pawn.c = piece_type;
}

fn ask_for_type(white: bool) -> char {
    let idx: usize = ask_for_type_id() as usize;
    if white {
        return CHOOSABLE_PIECES_WHITE[idx];
    } else {
        return CHOOSABLE_PIECES_BLACK[idx];
    }
}

fn ask_for_type_id() -> u32 {
    println!("Choose a piece: ");
    for i in 1..CHOOSABLE_PIECES_WHITE.len()+1 {
        let v = CHOOSABLE_PIECES_WHITE[i-1];
        println!("{i}) {v}");
    }

    let mut type_str: String = String::new();
    let _ = io::stdin().read_line(&mut type_str);

    return match type_str.trim().parse::<u32>() {
        Ok(n) => n-1,
        Err(_) => ask_for_type_id(),
    }
}