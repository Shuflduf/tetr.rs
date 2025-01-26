use macroquad::text::{draw_text_ex, load_ttf_font, Font, TextParams};

static mut FONT: Option<Font> = None;

pub async fn ready() {
    unsafe {
        FONT = Some(load_ttf_font("assets/PressStart2P.ttf").await.unwrap());
    }
}

pub fn draw_ui() {
    unsafe {
        let text_params = TextParams {
            // rust is not a game dev language
            #[allow(static_mut_refs)]
            font: FONT.as_ref(),
            ..Default::default()
        };
        draw_text_ex("TETR.RS", 10.0, 50.0, text_params);
    }
}
