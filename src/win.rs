use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::ui::text_button::TextButton;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32, highscore: i32) -> (String, TextureManager, i32, i32 ){

    let win= StillImage::from_preload(
    tm.get_preload("assets/win.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let btn_exit = TextButton::new(
        950.0,
        25.0,
        60.0,
        60.0,
        "X",
        RED,
        ORANGE,
        30
    );

let btn_return = TextButton::new(
        25.0,
        25.0,
        60.0,
        60.0,
        "<-",
        BLUE,
        PURPLE,
        30
    );

let mut lbl_score = Label::new(format!("Score:\n{}", score), 450.0, 75.0, 60);
lbl_score.with_colors(YELLOW, Some(BLACK));

    loop {
        clear_background(WHITE);
        win.draw();
        lbl_score.draw();

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), tm, score, highscore);
        }

        if btn_exit.click() {
            return("".to_string(),tm, score, highscore);
        }

        if btn_return.click() {
            return("menu".to_string(),tm, score, highscore);
        }

        next_frame().await;
    }
}