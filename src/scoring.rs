use macroquad::{
    math::vec2,
    text::{draw_text_ex, TextParams},
    window::{screen_height, screen_width},
};

use crate::{pieces::SRS_DATA, ui::FONT};

static mut LINE_CLEAR_TABLE: [u32; 4] = [0; 4];
static mut TSPIN_TABLE: [u32; 4] = [0; 4];
static mut MINI_TSPIN_TABLE: [u32; 3] = [0; 3];

static mut SCORE: u32 = 0;
static mut HIGH_SCORE: u32 = 0;

fn populate_table(key: &str, table: &mut [u32]) {
    unsafe {
        let data = SRS_DATA[key].as_array().unwrap();
        for (i, value) in data.iter().enumerate() {
            table[i] = value.as_u64().unwrap() as u32;
        }
    }
}

pub fn ready() {
    unsafe {
        // never use rust for gamedev (unless bevy)
        #[allow(static_mut_refs)]
        populate_table("scoring_lines", &mut LINE_CLEAR_TABLE);
        #[allow(static_mut_refs)]
        populate_table("scoring_tspin", &mut TSPIN_TABLE);
        #[allow(static_mut_refs)]
        populate_table("scoring_mini", &mut MINI_TSPIN_TABLE);
    }
}

pub fn reset() {
    unsafe {
        SCORE = 0;
    }
}

pub fn lines_cleared(amount: i32) {
    unsafe {
        SCORE += LINE_CLEAR_TABLE[amount as usize - 1];
        if SCORE > HIGH_SCORE {
            HIGH_SCORE = SCORE;
        }
    }
}

pub fn draw() {
    unsafe {
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let text_params = TextParams {
            #[allow(static_mut_refs)]
            font: FONT.as_ref(),
            font_size: (screen_height() / 48.0) as u16,
            ..Default::default()
        };

        let mut offset = vec2(0.31, 0.45);
        let final_position = center + vec2(offset.x * screen_height(), offset.y * screen_height());
        draw_text_ex(
            #[allow(static_mut_refs)]
            format!("SCORE: {}", SCORE).as_str(),
            final_position.x,
            final_position.y,
            text_params.clone(),
        );

        offset += vec2(0.0, 0.03);
        let final_position = center + vec2(offset.x * screen_height(), offset.y * screen_height());
        draw_text_ex(
            #[allow(static_mut_refs)]
            format!("HIGH: {}", HIGH_SCORE).as_str(),
            final_position.x,
            final_position.y,
            text_params.clone(),
        );
    }
}
