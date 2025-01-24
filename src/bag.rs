use macroquad::{
    color::WHITE,
    math::{ivec2, vec2, IVec2},
    rand::{srand, ChooseRandom},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    time,
};

use crate::{
    get_rect_from_index,
    pieces::{self, Piece, SRS_DATA},
};

const OFFSET: IVec2 = ivec2(13, 2);

static mut BAG: Vec<Piece> = Vec::new();
static mut NEXT_PIECE: Option<Piece> = None;

pub fn draw_next_piece(texture: &Texture2D, block_size: f32, offset_x: f32) {
    unsafe {
        let piece_data = SRS_DATA["pieces"][NEXT_PIECE.unwrap().index as usize][0].clone();
        for pos in piece_data.as_array().unwrap().iter().map(|x| {
            let coords = x.as_array().unwrap();
            ivec2(
                coords[0].as_i64().unwrap() as i32,
                coords[1].as_i64().unwrap() as i32,
            )
        }) {
            let params = DrawTextureParams {
                dest_size: Some(vec2(block_size, block_size)),
                source: get_rect_from_index(NEXT_PIECE.unwrap().index.into()),
                ..Default::default()
            };
            draw_texture_ex(
                texture,
                offset_x + (OFFSET.x + pos.x) as f32 * block_size,
                (OFFSET.y + pos.y) as f32 * block_size,
                WHITE,
                params,
            );
        }
    }
}

pub fn next_piece() -> Piece {
    unsafe {
        if (*BAG).is_empty() {
            reset_bag();
        }
        let current = NEXT_PIECE.unwrap_or_else(|| {
            let piece = *(*BAG).last().unwrap();
            BAG = BAG[..(*BAG).len() - 1].to_vec();
            piece
        });

        NEXT_PIECE = if (*BAG).is_empty() {
            None
        } else {
            Some(*(*BAG).last().unwrap())
        };

        if !(*BAG).is_empty() {
            BAG = BAG[..(*BAG).len() - 1].to_vec();
        }

        current
    }
}

pub fn reset_bag() {
    unsafe {
        srand(time::get_time() as u64);
        BAG = (0..7)
            .map(|i| Piece {
                index: i,
                rotation: 0,
                pos: pieces::START_POS,
            })
            .collect();
        (*BAG).shuffle();
    }
}
