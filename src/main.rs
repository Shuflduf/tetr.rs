use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    const GRID_SIZE: IVec2 = ivec2(10, 20);
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

        // Draw board
        let block_size = screen_height() / (GRID_SIZE.y as f32 + 1.0);
        let board_width = block_size * (GRID_SIZE.x as f32 + 2.0);
        let offset_x = (screen_width() - board_width) / 2.0;
        for block in bounds.iter() {
            let params = DrawTextureParams {
                dest_size: Some(vec2(block_size, block_size)),
                source: get_rect_from_index(7),
                ..Default::default()
            };
            draw_texture_ex(
                &texture,
                offset_x + block.x as f32 * block_size,
                block.y as f32 * block_size,
                WHITE,
                params,
            );
        }

        draw_rectangle(
            0.0,
            0.0,
            20.0,
            20.0,
            YELLOW
        );

        next_frame().await
    }
}

fn get_rect_from_index(index: i32) -> Option<Rect> {
    Some(Rect::new(index as f32 * 32.0 + 4.0, 4.0, 26.0, 26.0))
}
