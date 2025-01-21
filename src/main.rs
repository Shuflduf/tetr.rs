use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLACK);

        let texture = load_texture("assets/texture_simple.png").await.unwrap();
        let params = DrawTextureParams{
            dest_size: Some(Vec2::new(100.0, 100.0)),
            source: Some(Rect::new(0.0, 0.0, 32.0, 32.0)),
            ..Default::default()
        };
        draw_texture_ex(&texture, 0.0, 0.0, WHITE,
            params   
            );

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
