use macroquad::prelude::*;

mod pieces;

const GRID_SIZE: IVec2 = ivec2(10, 20);
pub static mut BLOCK_SIZE: f32 = 0.0;
pub static mut OFFSET_X: f32 = 0.0;

#[macroquad::main("MyGame")]
async fn main() {
    pieces::load_json();
    let texture = load_texture("assets/texture_simple.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let bounds: Vec<IVec2> = {
        let mut final_bounds = Vec::new();
        for y in 0..(GRID_SIZE.y + 1) {
            final_bounds.push(ivec2(0, y));
            final_bounds.push(ivec2(GRID_SIZE.x + 1, y));
        }
        for x in 0..(GRID_SIZE.x) {
            final_bounds.push(ivec2(x + 1, GRID_SIZE.y));
        }
        final_bounds
    };
    loop {
        clear_background(BLACK);

        unsafe {
            // Draw board
            BLOCK_SIZE = screen_height() / (GRID_SIZE.y as f32 + 1.0);
            let board_width = BLOCK_SIZE * (GRID_SIZE.x as f32 + 2.0);
            OFFSET_X = (screen_width() - board_width) / 2.0;
            for block in bounds.iter() {
                let params = DrawTextureParams {
                    dest_size: Some(vec2(BLOCK_SIZE, BLOCK_SIZE)),
                    source: get_rect_from_index(7),
                    ..Default::default()
                };
                draw_texture_ex(
                    &texture,
                    OFFSET_X + block.x as f32 * BLOCK_SIZE,
                    block.y as f32 * BLOCK_SIZE,
                    WHITE,
                    params,
                );
            }
        }

        pieces::update(&texture);

        next_frame().await
    }
}

fn get_rect_from_index(index: i32) -> Option<Rect> {
    Some(Rect::new(index as f32 * 32.0 + 4.0, 4.0, 26.0, 26.0))
}
