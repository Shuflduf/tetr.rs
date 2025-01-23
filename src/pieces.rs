use macroquad::math::IVec2;
use crate::*;

//const NULL_PIECE: Piece = Piece {
//    index: -1,
//    rotation: -1,
//    pos: IVec2::ZERO,
//};
pub const START_POS: IVec2 = IVec2 { x: 4, y: 0 };

#[derive(Clone, Copy)]
pub struct Piece {
    pub index: i8,
    pub rotation: i8,
    pub pos: IVec2,
} impl Piece {
    pub fn add_to_board(self, board: &mut Vec<Block>) {
        unsafe{
            for pos in SRS_DATA["pieces"][self.index as usize][self.rotation as usize].as_array().unwrap() {
                let x = pos[0].as_i64().unwrap() as i32 + self.pos.x;
                let y = pos[1].as_i64().unwrap() as i32 + self.pos.y;
                board.push(Block {
                    index: self.index,
                    pos: IVec2 { x, y },
                });
            }
        }
    }
    // also does rotation checking
    pub fn can_move(self, board: &[Block]) -> bool {
        unsafe{
            for pos in SRS_DATA["pieces"][self.index as usize][self.rotation as usize].as_array().unwrap() {
                let x = pos[0].as_i64().unwrap() as i32 + self.pos.x;
                let y = pos[1].as_i64().unwrap() as i32 + self.pos.y;
                if !(0..GRID_SIZE.x + 1).contains(&x) || y >= 20 {
                    return false;
                }
                for block in board.iter() {
                    if block.pos == (IVec2 { x, y }) {
                        return false;
                    }
                }
            }
        }
        true
    }
    pub fn moved(self, dir: IVec2) -> Piece {
        Piece {
            pos: self.pos + dir,
            ..self
        }
    }
    pub fn copy(self) -> Piece {
        Piece {
            index: self.index,
            rotation: self.rotation,
            pos: self.pos,
        }
    }
}
pub struct Block {
    pub index: i8,
    pub pos: IVec2,
}

static mut ACTIVE_PIECE: Piece = Piece {
    index: 0,
    rotation: 0,
    pos: START_POS,
};
static mut SRS_DATA: serde_json::Value = serde_json::Value::Null;

pub fn ready() {
    let json = include_str!("srs.json");
    unsafe {
        ACTIVE_PIECE = bag::next_piece();
        SRS_DATA = serde_json::from_str(json).unwrap();
    }
}

pub fn update(texture: &Texture2D, block_size: f32, offset_x: f32, board: &mut Vec<Block>) -> bool {
    let mut placed = false;
    unsafe{
        let mut future_piece = ACTIVE_PIECE.copy();
        if is_key_pressed(KeyCode::A) {
            future_piece.pos.x -= 1;
        } else if is_key_pressed(KeyCode::D) {
            future_piece.pos.x += 1;
        }
        if is_key_pressed(KeyCode::W) {
            future_piece.pos.y += 1;
        }
        if is_key_pressed(KeyCode::S) {
            while future_piece.moved(ivec2(0, 1)).can_move(board) {
                future_piece.pos.y += 1;
            }
        }
        if is_key_pressed(KeyCode::Left){
            future_piece.rotation += 3;
            future_piece.rotation %= 4;
        } else if is_key_pressed(KeyCode::Right) {
            future_piece.rotation += 1;
            future_piece.rotation %= 4;
        }
        if future_piece.can_move(board) {
            ACTIVE_PIECE = future_piece;
        } else if future_piece.rotation != ACTIVE_PIECE.rotation {
            let kick_index = get_kick_index(ACTIVE_PIECE.rotation, future_piece.rotation);
            let kick_table = {
                if ACTIVE_PIECE.index != 4 {
                    SRS_DATA["kicks"].as_array().unwrap()
                } else {
                    SRS_DATA["kicks_i"].as_array().unwrap()
                }
            };
            for kick in kick_table[kick_index as usize].as_array().unwrap() {
                let x = kick[0].as_i64().unwrap() as i32;
                let y = kick[1].as_i64().unwrap() as i32;
                let pos = IVec2 { x, y };
                let new_piece = future_piece.moved(pos);
                if new_piece.can_move(board) {
                    ACTIVE_PIECE = new_piece;
                    break;
                }
            }
        }

        if !ACTIVE_PIECE.moved(ivec2(0, 1)).can_move(board) {
            ACTIVE_PIECE.add_to_board(board);
            ACTIVE_PIECE = bag::next_piece();
            placed = true;
        }

        for pos in SRS_DATA["pieces"][ACTIVE_PIECE.index as usize][ACTIVE_PIECE.rotation as usize].as_array().unwrap() {
            let x = pos[0].as_i64().unwrap() as i32 + ACTIVE_PIECE.pos.x;
            let y = pos[1].as_i64().unwrap() as i32 + ACTIVE_PIECE.pos.y;
            let params = DrawTextureParams {
                dest_size: Some(vec2(block_size, block_size)),
                source: get_rect_from_index(ACTIVE_PIECE.index.into()),
                ..Default::default()
            };
            draw_texture_ex(
                texture,
                offset_x + x as f32 * block_size,
                y as f32 * block_size,
                WHITE,
                params,
            );
        }
    }
    placed
}

// dont try to do 180 spins with this
fn get_kick_index(before: i8, after: i8) -> i8 {
    if after == (before + 1) % 4 {
        before * 2
    } else {
        (before * 2 + 7) % 8
    }
}
