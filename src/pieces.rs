use macroquad::math::IVec2;
use crate::*;

const NULL_PIECE: Piece = Piece {
    index: -1,
    rotation: -1,
    pos: IVec2::ZERO,
};
const START_POS: IVec2 = IVec2 { x: 4, y: 0 };

struct Piece {
    pub index: i8,
    pub rotation: i8,
    pub pos: IVec2,
}

static mut ACTIVE_PIECE: Piece = Piece {
    index: 5,
    rotation: 0,
    pos: START_POS,
};
static mut SRS_DATA: serde_json::Value = serde_json::Value::Null;

pub fn load_json() {
    let json = include_str!("srs.json");
    unsafe {
        SRS_DATA = serde_json::from_str(json).unwrap();
    }
}

pub fn update(texture: &Texture2D) {
    unsafe{
        if is_key_pressed(KeyCode::A) {
            ACTIVE_PIECE.pos.x -= 1;
        } else if is_key_pressed(KeyCode::D) {
            ACTIVE_PIECE.pos.x += 1;
        }

        for pos in SRS_DATA["pieces"][ACTIVE_PIECE.index as usize][ACTIVE_PIECE.rotation as usize].as_array().unwrap() {
            let x = pos[0].as_i64().unwrap() as i32 + ACTIVE_PIECE.pos.x;
            let y = pos[1].as_i64().unwrap() as i32 + ACTIVE_PIECE.pos.y;
            let params = DrawTextureParams {
                dest_size: Some(vec2(BLOCK_SIZE, BLOCK_SIZE)),
                source: get_rect_from_index(ACTIVE_PIECE.index.into()),
                ..Default::default()
            };
            draw_texture_ex(
                texture,
                OFFSET_X + x as f32 * BLOCK_SIZE,
                y as f32 * BLOCK_SIZE,
                WHITE,
                params,
            );
        }
    }
}
