use macroquad::{color::GRAY, math::vec2, text::{draw_text_ex, load_ttf_font, Font, TextParams}, window::{screen_height, screen_width}};
use miniquad::FilterMode;

static mut FONT: Option<Font> = None;

pub async fn ready() {
    unsafe {
        FONT = Some({
            let mut f = load_ttf_font("assets/PressStart2P.ttf").await.unwrap();
            f.set_filter(FilterMode::Nearest);
            f
        });
    }
}

pub fn draw_ui() {
    unsafe {
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let text_params = TextParams {
            // rust is not a game dev language
            #[allow(static_mut_refs)]
            font: FONT.as_ref(),
            font_size: (screen_height() / 27.0) as u16,
            ..Default::default()
        };

        let offset = vec2(-0.65, -0.45);

        let final_position = center + vec2(offset.x * screen_height(), offset.y * screen_height());

        draw_text_ex("TETR.RS", final_position.x, final_position.y, text_params.clone());
        draw_controls();
    }
}

fn draw_controls() {
    const CONTROLS: [&str; 8] = [
        "A - LEFT",
        "D - RIGHT",
        "L ARROW - CCW",
        "R ARROW - CW",
        "S - SOFT",
        "W - HARD",
        "SHIFT - HOLD",
        "CLICK TO HIDE"
    ];
    
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    unsafe {
        let text_params = TextParams {
            #[allow(static_mut_refs)]
            font: FONT.as_ref(),
            font_size: (screen_height() / 48.0) as u16,
            color: GRAY,
            ..Default::default()
        };

        let offset = vec2(-0.64, 0.3);

        let final_position = center + vec2(offset.x * screen_height(), offset.y * screen_height());

        for (i, control) in CONTROLS.iter().enumerate() {
            draw_text_ex(control, final_position.x, final_position.y + (i as f32 * 19.0), text_params.clone());
        }
    }
}
