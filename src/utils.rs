#![allow(unused)]

#[derive(Clone)]
pub struct Entity {
    pub x: u32,
    pub y: u32,
    pub c: char,
    pub a: bool,
}

impl Copy for Entity {

}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            x: 0,
            y: 0,
            c: '?',
            a: true,
        }
    }

    pub fn from(x: u32, y:u32, cha: char) -> Entity {
        Entity {
            x: x,
            y: y,
            c: cha,
            a: true
        }
    }

    pub fn print(&self) {
        println!("Ent:{}:[x:{}, y:{}]", self.c, self.x, self.y);
    }
}