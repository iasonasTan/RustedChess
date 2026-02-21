use utils::Entity;

// ['♔', '♕', '♖', '♗', '♘', '♙']
// ['♚', '♛', '♜', '♝', '♞', '♟']

pub fn is_white_piece(c: &char) -> bool {
    ['♔', '♕', '♖', '♗', '♘', '♙'].contains(c)
}

fn move_kill(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) {
    let mut to_move = pieces[i];
    let typ: bool = is_white_piece(&pieces[i].c);
    for to_check in &mut *pieces {
        // If to_check is in dx, dy
        if to_check.x == dx && to_check.y == dy && is_white_piece(&to_check.c)!=typ {
            // And it's NOT a piece of the same color
            to_check.a = false;
            to_move.x = dx;
            to_move.y = dy;
        }
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

fn move_rock(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    pieces[i].x==dx || pieces[i].y==dy
}

fn move_bishop(i: usize, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    let en: Entity = pieces[i];
    let delta_x = en.x.abs_diff(dx);
    let delta_y = en.y.abs_diff(dy);
    
    delta_x == delta_y
}

fn move_pawn(_i: usize, _dx: u32, _dy: u32, _pieces: &mut [Entity; 32]) -> bool {
    true
}

pub fn move_piece(ox: u32, oy: u32, dx: u32, dy: u32, pieces: &mut [Entity; 32]) -> bool {
    for i in 0..pieces.len() {
        let e: &mut Entity = &mut pieces[i];
        if e.x==ox && e.y==oy && e.a {
            println!("Moving {}", e.c);
            let mv: bool = match e.c {
                '♚'|'♔' => move_king(i, dx, dy, pieces),
                '♛'|'♕' => move_queen(i, dx, dy, pieces),
                '♜'|'♖' => move_rock(i, dx, dy, pieces),
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
        Entity::from_mut(1, 1, '♜'),
        Entity::from_mut(2, 1, '♞'),
        Entity::from_mut(3, 1, '♝'),
        Entity::from_mut(4, 1, '♛'),
        Entity::from_mut(5, 1, '♚'),
        Entity::from_mut(6, 1, '♝'),
        Entity::from_mut(7, 1, '♞'),
        Entity::from_mut(8, 1, '♜'),
        
        Entity::from_mut(1, 2, '♟'),
        Entity::from_mut(2, 2, '♟'),
        Entity::from_mut(3, 2, '♟'),
        Entity::from_mut(4, 2, '♟'),
        Entity::from_mut(5, 2, '♟'),
        Entity::from_mut(6, 2, '♟'),
        Entity::from_mut(7, 2, '♟'),
        Entity::from_mut(8, 2, '♟'),
        
        Entity::from_mut(1, 8, '♖'),
        Entity::from_mut(2, 8, '♘'),
        Entity::from_mut(3, 8, '♗'),
        Entity::from_mut(4, 8, '♕'),
        Entity::from_mut(5, 8, '♔'),
        Entity::from_mut(6, 8, '♗'),
        Entity::from_mut(7, 8, '♘'),
        Entity::from_mut(8, 8, '♖'),

        Entity::from_mut(1, 7, '♙'),
        Entity::from_mut(2, 7, '♙'),
        Entity::from_mut(3, 7, '♙'),
        Entity::from_mut(4, 7, '♙'),
        Entity::from_mut(5, 7, '♙'),
        Entity::from_mut(6, 7, '♙'),
        Entity::from_mut(7, 7, '♙'),
        Entity::from_mut(8, 7, '♙'),
    ]
}