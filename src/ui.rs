use macroquad::{color::GRAY, input::{is_mouse_button_pressed, mouse_position}, math::vec2, text::{draw_text_ex, load_ttf_font, Font, TextParams}, window::{screen_height, screen_width}};
use miniquad::FilterMode;

static mut FONT: Option<Font> = None;
static mut CONTROLS_HIDDEN: bool = false;

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

        if !CONTROLS_HIDDEN {
            for (i, control) in CONTROLS.iter().enumerate() {
                draw_text_ex(control, final_position.x, final_position.y + (i as f32 * 19.0), text_params.clone());
            }
        }

        // detect if click on text

        // NOTE: current implementation doesnt account for x position
        //if is_mouse_button_down(miniquad::MouseButton::Left) {
        //    let mouse = mouse_position();
        //    let mut y = final_position.y;
        //    for control in CONTROLS.iter() {
        //        if mouse.1 > y && mouse.1 < y + 19.0 {
        //            CONTROLS_HIDDEN = true;
        //            break;
        //        }
        //        y += 19.0;
        //    }
        //}
        
        
        if is_mouse_button_pressed(miniquad::MouseButton::Left) {
            let mouse = mouse_position();

            if mouse.0 > final_position.x - 100.0 && mouse.0 < final_position.x + 100.0 && mouse.1 > final_position.y - 100.0 && mouse.1 < final_position.y + 100.0 {
                CONTROLS_HIDDEN = true;
            }
        }

    }
}
