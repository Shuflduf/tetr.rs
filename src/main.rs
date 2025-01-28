use board::*;
use macroquad::prelude::*;
use pieces::{Block, LAST_TSPIN};

mod bag;
mod board;
mod hold_piece;
mod pieces;
mod scoring;
mod ui;

const GRID_SIZE: IVec2 = ivec2(10, 20);

#[macroquad::main("tetr.rs")]
async fn main() {
    pieces::ready();
    scoring::ready();
    ui::ready().await;

    let texture = load_texture("assets/texture_simple.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let mut collision: Vec<Block> = reset_board();
    let mut is_fullscreen = false;

    loop {
        clear_background(BLACK);
        if false {
            draw_debug_rectangle();
        }

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
            unsafe {

                println!("ABC: {:?}", LAST_TSPIN)
            }
            scoring::update_score(full_lines.len() as i32);
            clear_lines(&mut collision, &full_lines);
        }
        if is_key_pressed(KeyCode::F) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }

        bag::draw(&texture, block_size, offset_x);
        hold_piece::draw(&texture, block_size, offset_x);
        ui::draw();
        scoring::draw();

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

fn draw_debug_rectangle() {
    let aspect_ratio = screen_width() / screen_height();
    let target_aspect_ratio = 4.0 / 3.0;
    let target_width = screen_height() * target_aspect_ratio;
    let target_height = screen_width() / target_aspect_ratio;
    let x_offset = (screen_width() - target_width) / 2.0;
    let y_offset = (screen_height() - target_height) / 2.0;
    if aspect_ratio > target_aspect_ratio {
        draw_rectangle(x_offset, 0.0, target_width, screen_height(), GREEN);
    } else {
        draw_rectangle(0.0, y_offset, screen_width(), target_height, BLACK);
    }
}
