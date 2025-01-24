use macroquad::{
    color::Color,
    math::{ivec2, vec2, IVec2},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
    bag, get_rect_from_index,
    pieces::{Piece, SRS_DATA, START_POS},
};

pub static mut JUST_HELD: bool = false;
pub static mut HELD_PIECE_INDEX: Option<i8> = None;

pub fn hold_piece(piece: Piece) -> Piece {
    unsafe {
        if JUST_HELD {
            return piece;
        }
        if let Some(held_index) = HELD_PIECE_INDEX {
            HELD_PIECE_INDEX = Some(piece.index);
            JUST_HELD = true;
            Piece {
                index: held_index,
                rotation: 0,
                pos: START_POS,
            }
        } else {
            HELD_PIECE_INDEX = Some(piece.index);
            JUST_HELD = true;
            bag::next_piece()
        }
    }
}

pub fn draw_held_piece(texture: &Texture2D, block_size: f32, offset_x: f32) {
    const OFFSET: IVec2 = ivec2(-5, 2);
    unsafe {
        if let Some(held_index) = HELD_PIECE_INDEX {
            let piece_data = SRS_DATA["pieces"][held_index as usize][0].clone();
            for pos in piece_data.as_array().unwrap().iter().map(|x| {
                let coords = x.as_array().unwrap();
                ivec2(
                    coords[0].as_i64().unwrap() as i32,
                    coords[1].as_i64().unwrap() as i32,
                )
            }) {
                let params = DrawTextureParams {
                    dest_size: Some(vec2(block_size, block_size)),
                    source: get_rect_from_index(held_index.into()),
                    ..Default::default()
                };
                draw_texture_ex(
                    texture,
                    offset_x + (OFFSET.x + pos.x) as f32 * block_size,
                    (OFFSET.y + pos.y) as f32 * block_size,
                    Color::new(1.0, 1.0, 1.0, if JUST_HELD { 0.2 } else { 1.0 }),
                    params,
                );
            }
        }
    }
}
