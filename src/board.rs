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
    let mut y: i32 = if white_plays { 0 } else { HEIGHT } as i32;
    let end_y: i32 = if white_plays { HEIGHT } else { 0 } as i32;
    let y_step: i32 = if white_plays { 1 } else { -1 };

    loop {
        print!("{}. ", y);

        let mut x: i32 = if white_plays { 1 } else { WIDTH } as i32;
        let end_x: i32 = if white_plays { WIDTH } else { 1 } as i32;
        let x_step: i32 = if white_plays { 1 } else { -1 };

        loop {
            if y == 0 {
                print!("{} ", x);
            } else {
                draw_pxls(x as u32, y as u32, pieces);
            }

            if x == end_x {
                break;
            }
            x += x_step;
        }

        println!();

        if y == end_y {
            break;
        }
        y += y_step;
    }
}