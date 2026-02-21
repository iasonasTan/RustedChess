use utils::Entity;

pub const WIDTH: u32 = 8;
pub const HEIGHT: u32 = 8;

fn draw_pxls(x: u32, y: u32, pieces: &mut [Entity; 32]) {
    let mut drawn: bool = false;
    for i in 0..pieces.len() {
        let en: Entity = pieces[i];
        if en.x==x && y==en.y && en.a {
            print!("{} ", en.c);
            drawn = true;
            break;
        }
    }
    if !drawn {
        let ch: char = if y%2==0&&x%2==0||y%2!=0&&x%2!=0
                { '■' } else { '□' };
        print!("{} ", ch);
    }
}

pub fn draw(pieces: &mut [Entity; 32], white_plays: bool) {
    let start_y = if white_plays { 0 } else { HEIGHT+1 };
    let start_x = if white_plays { 1 } else { WIDTH+1 };
    for y in 0..HEIGHT+1 {
        print!("{}. ", y);
        for x in 1..WIDTH+1 {
            if y==0 {
                print!("{} ", x);
            } else {
                draw_pxls(x, y, pieces);
            }
        }
        println!();
    }
}