use board::*;
use macroquad::prelude::*;
use pieces::Block;

mod bag;
mod board;
mod pieces;

const GRID_SIZE: IVec2 = ivec2(10, 20);

#[macroquad::main("tetr.rs")]
async fn main() {
    pieces::ready();
    let texture = load_texture("assets/texture_simple.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let mut collision: Vec<Block> = reset_board();
    let mut is_fullscreen = false;

    loop {
        clear_background(BLACK);

        // Draw board
        let block_size = screen_height() / (GRID_SIZE.y as f32 + 1.0);
        let board_width = block_size * (GRID_SIZE.x as f32 + 2.0);
        let offset_x = (screen_width() - board_width) / 2.0;
        for block in collision.iter() {
            let params = DrawTextureParams {
                dest_size: Some(vec2(block_size, block_size)),
                source: get_rect_from_index(block.index.into()),
                ..Default::default()
            };
            draw_texture_ex(
                &texture,
                offset_x + block.pos.x as f32 * block_size,
                block.pos.y as f32 * block_size,
                WHITE,
                params.clone(),
            );
        }
        if pieces::update(&texture, block_size, offset_x, &mut collision) {
            let full_lines = full_lines(&collision);
            clear_lines(&mut collision, &full_lines);
        }
        bag::draw_next_piece(&texture, block_size, offset_x);
        if is_key_pressed(KeyCode::F) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }
        next_frame().await
    }
}

fn reset_board() -> Vec<Block> {
    let mut board: Vec<Block> = Vec::new();
    board.clear();
    for y in 0..(GRID_SIZE.y + 1) {
        board.push(Block {
            index: 7,
            pos: ivec2(0, y),
        });
        board.push(Block {
            index: 7,
            pos: ivec2(GRID_SIZE.x + 1, y),
        });
    }
    for x in 0..(GRID_SIZE.x) {
        board.push(Block {
            index: 7,
            pos: ivec2(x + 1, GRID_SIZE.y),
        });
    }
    board
}

pub fn get_rect_from_index(index: i32) -> Option<Rect> {
    Some(Rect::new(index as f32 * 32.0 + 4.0, 4.0, 26.0, 26.0))
}
