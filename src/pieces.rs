use utils::Entity;

// ['♔', '♕', '♖', '♗', '♘', '♙']
// ['♚', '♛', '♜', '♝', '♞', '♟']

const WHITE_PAWN_DEFAULT_Y: u32 = 7;
const BLACK_PAWN_DEFAULT_Y: u32 = 2;

fn is_white_piece(en: &Entity) -> bool {
    ['♔', '♕', '♖', '♗', '♘', '♙'].contains(&en.c)
}

fn move_kill(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) {
    let color: bool = is_white_piece(&pieces[i]);
    let mut mv: bool = true;
    for j in 0..pieces.len() {
        let j_color: bool = is_white_piece(&pieces[j]);
        if pieces[j].x == dx && pieces[j].y == dy {
            if j_color == color {
                mv = false;
            }
            if j_color != color {
                pieces[j].a = false;
            }
        }
    }
    if mv {
        pieces[i].x = dx;
        pieces[i].y = dy;
    }
}

fn move_king(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    let en: Entity = pieces[i];
    let delta_x = en.x.abs_diff(dx);
    let delta_y = en.y.abs_diff(dy);
    delta_x==1||delta_y==1
}

fn move_queen(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    let en: Entity = pieces[i];
    let delta_x = en.x.abs_diff(dx);
    let delta_y = en.y.abs_diff(dy);
    delta_x == delta_y && pieces[i].x==dx || pieces[i].y==dy
}

fn move_knight(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    let en: Entity = pieces[i];
    let delta_x = en.x.abs_diff(dx);
    let delta_y = en.y.abs_diff(dy);
    delta_x==2&&delta_y==1||delta_x==1&&delta_y==2
}

fn move_rook(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    pieces[i].x==dx || pieces[i].y==dy
}

fn move_bishop(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    let en: Entity = pieces[i];
    let delta_x = en.x.abs_diff(dx);
    let delta_y = en.y.abs_diff(dy);
    delta_x == delta_y
}

fn move_pawn(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    let en: Entity = pieces[i];
    let delta_x = en.x.abs_diff(dx);
    let delta_y = en.y.abs_diff(dy);
    if delta_x == 0 {
        let is_white: bool = is_white_piece(&pieces[i]);
        let default_pos: u32 = if is_white { WHITE_PAWN_DEFAULT_Y } else { BLACK_PAWN_DEFAULT_Y };
        let max_delta: u32 = if default_pos == pieces[i].y { 2 } else { 1 };
        return delta_y <= max_delta;
    } else {
        return delta_y == 1 && delta_x == 1;
    }
}

pub fn move_piece(ox: u32, oy: u32, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    for i in 0..pieces.len() {
        let e: &mut Entity = &mut pieces[i];
        if e.x==ox && e.y==oy && e.a {
            println!("Moving {}", e.c);
            let mv: bool = match e.c {
                '♚'|'♔' => move_king(i, dx, dy, pieces),
                '♛'|'♕' => move_queen(i, dx, dy, pieces),
                '♜'|'♖' => move_rook(i, dx, dy, pieces),
                '♝'|'♗' => move_bishop(i, dx, dy, pieces),
                '♞'|'♘' => move_knight(i, dx, dy, pieces),
                '♟'|'♙' => move_pawn(i, dx, dy, pieces),
                _ => false,
            };
            if mv {
                move_kill(i, dx, dy, pieces);
            }
            return mv;
        }
    }
    false
}

pub fn default() -> [Entity; 32] {
    [
        Entity::from(1, 1, '♜'),
        Entity::from(2, 1, '♞'),
        Entity::from(3, 1, '♝'),
        Entity::from(4, 1, '♛'),
        Entity::from(5, 1, '♚'),
        Entity::from(6, 1, '♝'),
        Entity::from(7, 1, '♞'),
        Entity::from(8, 1, '♜'),
        
        Entity::from(1, 2, '♟'),
        Entity::from(2, 2, '♟'),
        Entity::from(3, 2, '♟'),
        Entity::from(4, 2, '♟'),
        Entity::from(5, 2, '♟'),
        Entity::from(6, 2, '♟'),
        Entity::from(7, 2, '♟'),
        Entity::from(8, 2, '♟'),
        
        Entity::from(1, 8, '♖'),
        Entity::from(2, 8, '♘'),
        Entity::from(3, 8, '♗'),
        Entity::from(4, 8, '♕'),
        Entity::from(5, 8, '♔'),
        Entity::from(6, 8, '♗'),
        Entity::from(7, 8, '♘'),
        Entity::from(8, 8, '♖'),

        Entity::from(1, 7, '♙'),
        Entity::from(2, 7, '♙'),
        Entity::from(3, 7, '♙'),
        Entity::from(4, 7, '♙'),
        Entity::from(5, 7, '♙'),
        Entity::from(6, 7, '♙'),
        Entity::from(7, 7, '♙'),
        Entity::from(8, 7, '♙'),
    ]
}