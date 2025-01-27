use crate::*;
use bag::reset_bag;
use hold_piece::hold_piece;
use macroquad::math::IVec2;

pub const START_POS: IVec2 = IVec2 { x: 4, y: 0 };
const GRAVITY_DELAY: f32 = 1.0;
const LOCK_DELAY: f32 = 0.5;
const MAX_LOCK_DELAY: f32 = 2.0;

const ARR: f32 = 0.0333;
const DAS: f32 = 0.1667;
const SDF: f32 = 10.0;

#[derive(Clone, Copy)]
pub struct Piece {
    pub index: i8,
    pub rotation: i8,
    pub pos: IVec2,
}
impl Piece {
    pub fn add_to_board(self, board: &mut Vec<Block>) {
        unsafe {
            for pos in SRS_DATA["pieces"][self.index as usize][self.rotation as usize]
                .as_array()
                .unwrap()
            {
                let x = pos[0].as_i64().unwrap() as i32 + self.pos.x;
                let y = pos[1].as_i64().unwrap() as i32 + self.pos.y;
                board.push(Block {
                    index: self.index,
                    pos: IVec2 { x, y },
                });
                ON_GROUND = false;
                LOCK_DELAY_TIMER = 0.0;
                MAX_LOCK_DELAY_TIMER = 0.0;
                GRAVITY_TIMER = 0.0;
            }
        }
    }
    // also does rotation checking
    pub fn can_move(self, board: &[Block]) -> bool {
        unsafe {
            for pos in SRS_DATA["pieces"][self.index as usize][self.rotation as usize]
                .as_array()
                .unwrap()
            {
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
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub left_timer: f32,
    pub right_timer: f32,
    pub soft_drop: bool,
}
#[derive(Debug)]
pub enum TSpin {
    None,
    Mini,
    Regular,
}

pub static mut SRS_DATA: serde_json::Value = serde_json::Value::Null;
static mut ACTIVE_PIECE: Piece = Piece {
    index: 0,
    rotation: 0,
    pos: START_POS,
};
static mut GRAVITY_TIMER: f32 = 0.0;
static mut LOCK_DELAY_TIMER: f32 = 0.0;
static mut MAX_LOCK_DELAY_TIMER: f32 = 0.0;
static mut ON_GROUND: bool = false;
static mut LAST_KICK: i8 = 0;
static mut LAST_TSPIN: TSpin = TSpin::None;
static mut MAIN_INPUTS: Inputs = Inputs {
    left: false,
    right: false,
    left_timer: 0.0,
    right_timer: 0.0,
    soft_drop: false,
};

pub fn ready() {
    let json = include_str!("srs.json");
    unsafe {
        ACTIVE_PIECE = bag::next_piece();
        SRS_DATA = serde_json::from_str(json).unwrap();
    }
}

fn update_inputs(target_piece: &mut Piece) {
    unsafe {
        if is_key_pressed(KeyCode::A) {
            target_piece.pos.x -= 1;
            MAIN_INPUTS.left = true;
            MAIN_INPUTS.right = false;
        } else if is_key_pressed(KeyCode::D) {
            target_piece.pos.x += 1;
            MAIN_INPUTS.right = true;
            MAIN_INPUTS.left = false;
        }

        if is_key_released(KeyCode::A) {
            MAIN_INPUTS.left = false;
            if is_key_down(KeyCode::D) {
                target_piece.pos.x += 1;
                MAIN_INPUTS.right = true;
            }
        } else if is_key_released(KeyCode::D) {
            MAIN_INPUTS.right = false;
            if is_key_down(KeyCode::A) {
                target_piece.pos.x -= 1;
                MAIN_INPUTS.left = true;
            }
        }

        if !is_key_down(KeyCode::A) && !is_key_down(KeyCode::D) {
            MAIN_INPUTS.left = false;
            MAIN_INPUTS.right = false;
            MAIN_INPUTS.left_timer = 0.0;
            MAIN_INPUTS.right_timer = 0.0;
        }

        MAIN_INPUTS.soft_drop = is_key_down(KeyCode::W);
        if is_key_pressed(KeyCode::W) {
            GRAVITY_TIMER = GRAVITY_DELAY
        }
    }
}

fn pos_empty(pos: IVec2, board: &[Block]) -> bool {
    for block in board.iter() {
        if block.pos == pos {
            return false;
        }
    }
    true
}

fn check_for_tspin(board: &[Block]) {
    unsafe {
        if ACTIVE_PIECE.index != 6 {
            return
        }
        println!("Last kick: {:?}", LAST_KICK);
        let current_3x3 =
            [ivec2(0, 0), ivec2(2, 0), ivec2(2, 2), ivec2(0, 2)].map(|i| i + ACTIVE_PIECE.pos);
        let (on_front, on_back) = {
            let mut final_front = [IVec2::ZERO; 2];
            let mut final_back = [IVec2::ZERO; 2];
            for i in 0..2 {
                final_front[i] = current_3x3[(ACTIVE_PIECE.rotation as usize + i) % 4];
                final_back[i] = current_3x3[(ACTIVE_PIECE.rotation as usize + i + 2) % 4];
            }
            (final_front, final_back)
        };
        println!("On front: {:?}. On back: {:?}", on_front, on_back);
        if LAST_KICK == 0 {
            for pos in on_front {
                if pos_empty(pos, board) {
                    LAST_TSPIN = TSpin::None;
                    return;
                }
            }
            for pos in on_back {
                if !pos_empty(pos, board) {
                    LAST_TSPIN = TSpin::Regular;
                    return;
                }
            }
        } else if LAST_KICK == 4 {
            for pos in on_back {
                if pos_empty(pos, board) {
                    LAST_TSPIN = TSpin::None;
                    return;
                }
            }
            for pos in on_front {
                if !pos_empty(pos, board) {
                    LAST_TSPIN = TSpin::Regular;
                    return;
                }
            }
        } else {
            for pos in on_back {
                if pos_empty(pos, board) {
                    LAST_TSPIN = TSpin::None;
                    return;
                }
            }
            for pos in on_front {
                if !pos_empty(pos, board) {
                    LAST_TSPIN = TSpin::Mini;
                    return;
                }
            }
        }
    }
}

pub fn update(texture: &Texture2D, block_size: f32, offset_x: f32, board: &mut Vec<Block>) -> bool {
    let mut placed = false;
    unsafe {
        if is_key_pressed(KeyCode::LeftShift) {
            GRAVITY_TIMER = 0.0;
            LOCK_DELAY_TIMER = 0.0;
            MAX_LOCK_DELAY_TIMER = 0.0;
            ON_GROUND = false;
            let held = hold_piece(ACTIVE_PIECE);
            ACTIVE_PIECE = held;
        }

        GRAVITY_TIMER += get_frame_time();
        if ON_GROUND {
            MAX_LOCK_DELAY_TIMER += get_frame_time();
        } else {
            MAX_LOCK_DELAY_TIMER = 0.0;
        }

        let mut future_piece = ACTIVE_PIECE.copy();
        if is_key_pressed(KeyCode::S) {
            future_piece.pos.y += get_drop_distance(board);
            placed = true;
            check_for_tspin(board);
            future_piece.add_to_board(board);
            ACTIVE_PIECE = bag::next_piece();
        } else {
            update_inputs(&mut future_piece);
            do_arr_magic(&mut future_piece);
            if is_key_pressed(KeyCode::W) {
                GRAVITY_TIMER = 0.0;
                future_piece.pos.y += 1;
            }
            if is_key_pressed(KeyCode::Left) {
                future_piece.rotation += 3;
                future_piece.rotation %= 4;
                LAST_KICK = 0;
            } else if is_key_pressed(KeyCode::Right) {
                future_piece.rotation += 1;
                future_piece.rotation %= 4;
                LAST_KICK = 0;
            }
            let actual_gravity_delay = if MAIN_INPUTS.soft_drop {
                GRAVITY_DELAY / SDF
            } else {
                GRAVITY_DELAY
            };
            if GRAVITY_TIMER >= actual_gravity_delay {
                future_piece.pos.y += 1;
                GRAVITY_TIMER = 0.0;
            }
            if future_piece.can_move(board) {
                if ACTIVE_PIECE.pos != future_piece.pos {
                    LOCK_DELAY_TIMER = 0.0;
                    if future_piece.rotation == ACTIVE_PIECE.rotation {
                        LAST_KICK = -1;
                    }
                } else if future_piece.rotation != ACTIVE_PIECE.rotation {
                    LOCK_DELAY_TIMER = 0.0
                }
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
                for (i, kick) in kick_table[kick_index as usize].as_array().unwrap().iter().enumerate() {
                    let x = kick[0].as_i64().unwrap() as i32;
                    let y = kick[1].as_i64().unwrap() as i32;
                    let pos = IVec2 { x, y };
                    let new_piece = future_piece.moved(pos);
                    if new_piece.can_move(board) {
                        LOCK_DELAY_TIMER = 0.0;
                        ACTIVE_PIECE = new_piece;
                        LAST_KICK = (i as i8) + 1;
                        break;
                    }
                }
            }

            if !ACTIVE_PIECE.moved(ivec2(0, 1)).can_move(board) {
                ON_GROUND = true;
                LOCK_DELAY_TIMER += get_frame_time();
                if LOCK_DELAY_TIMER >= LOCK_DELAY {
                    check_for_tspin(board);
                    ACTIVE_PIECE.add_to_board(board);
                    ACTIVE_PIECE = bag::next_piece();
                    placed = true;
                }
            } else {
                LOCK_DELAY_TIMER = 0.0;
            }

            if MAX_LOCK_DELAY_TIMER >= MAX_LOCK_DELAY {
                check_for_tspin(board);
                ACTIVE_PIECE
                    .moved(ivec2(0, get_drop_distance(board)))
                    .add_to_board(board);
                ACTIVE_PIECE = bag::next_piece();
                placed = true;
            }
        }
        for pos in SRS_DATA["pieces"][ACTIVE_PIECE.index as usize][ACTIVE_PIECE.rotation as usize]
            .as_array()
            .unwrap()
        {
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
                //WHITE,
                {
                    let base_col = WHITE;
                    let darkness = ((-((LOCK_DELAY_TIMER / LOCK_DELAY) - 0.5) + 0.5) * 0.7) + 0.3;
                    Color::new(
                        base_col.r * darkness,
                        base_col.g * darkness,
                        base_col.b * darkness,
                        base_col.a,
                    )
                },
                params.clone(),
            );

            // draw ghost
            draw_texture_ex(
                texture,
                offset_x + x as f32 * block_size,
                (y + get_drop_distance(board)) as f32 * block_size,
                Color::new(1.0, 1.0, 1.0, 0.2),
                params,
            );
        }
        if placed {
            println!("{:?}", LAST_TSPIN);
            hold_piece::JUST_HELD = false;
            LAST_TSPIN = TSpin::None;
            if !ACTIVE_PIECE.can_move(board) {
                *board = reset_board();
                reset_bag();
                hold_piece::HELD_PIECE_INDEX = None;
                ACTIVE_PIECE = bag::next_piece();
                ACTIVE_PIECE = bag::next_piece();
                scoring::reset();
            }
        }
    }
    placed
}

pub fn do_arr_magic(target_piece: &mut Piece) {
    unsafe {
        if MAIN_INPUTS.left {
            MAIN_INPUTS.left_timer += get_frame_time();
            if MAIN_INPUTS.left_timer >= DAS + ARR {
                target_piece.pos.x -= 1;
                MAIN_INPUTS.left_timer = DAS;
            }
        } else {
            MAIN_INPUTS.left_timer = 0.0;
        }
        if MAIN_INPUTS.right {
            MAIN_INPUTS.right_timer += get_frame_time();
            if MAIN_INPUTS.right_timer >= DAS + ARR {
                target_piece.pos.x += 1;
                MAIN_INPUTS.right_timer = DAS;
            }
        } else {
            MAIN_INPUTS.right_timer = 0.0;
        }
    }
}

pub fn get_drop_distance(board: &[Block]) -> i32 {
    unsafe {
        let mut future_piece = ACTIVE_PIECE.copy();
        while future_piece.moved(ivec2(0, 1)).can_move(board) {
            future_piece.pos.y += 1;
        }
        future_piece.pos.y - ACTIVE_PIECE.pos.y
    }
}

// dont try to do 180 spins with this
fn get_kick_index(before: i8, after: i8) -> i8 {
    if after == (before + 1) % 4 {
        before * 2
    } else {
        (before * 2 + 7) % 8
    }
}
