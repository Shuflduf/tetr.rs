use macroquad::{rand::{srand, ChooseRandom}, time};

use crate::pieces::{self, Piece};

static mut BAG: Vec<Piece> = Vec::new();

pub fn next_piece() -> Piece {
    unsafe {
        if (*BAG).is_empty() {
            reset_bag();
        }
        let last = *(*BAG).last().unwrap();
        BAG = BAG[..(*BAG).len() - 1].to_vec();

        last
    }    
}

pub fn reset_bag() {
    unsafe {
        srand(time::get_time() as u64);
        BAG = (0..7).map(|i| {
            Piece { index: i, rotation: 0, pos: pieces::START_POS }
        }).collect();
        (*BAG).shuffle();
    }
}
