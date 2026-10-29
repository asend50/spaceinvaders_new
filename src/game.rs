use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::collision::check_collision;
use crate::ui::text_button::TextButton;
use crate::custom::player::Player;
use crate::custom::bullet::Bullet;
use crate::custom::enemy::Enemy;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, mut score: i32, mut highscore: i32) -> (String, TextureManager, i32, i32 ){

    let mut player = Player::new(
    500.0,                   // X position
    680.0,                   // Y position
    screen_width() * 0.07,  // Width
    screen_height() * 0.08, // Height
    "assets/player.png",    // Image name
    true,                   // Enable stretching
    1.0,                    // Zoom level (100%)
    vec2(35.0, 15.0),       // Old position
).await;

    let background= StillImage::from_preload(
    tm.get_preload("assets/gamebackground.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let gamebars= StillImage::from_preload(
    tm.get_preload("assets/gamebars.png").unwrap(),
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

let mut lbl_score = Label::new(format!("Score:\n{}", score), 25.0, 125.0, 45);
let mut lbl_highscore = Label::new(format!("Highscore:\n{}", highscore), 815.0, 125.0, 45);

let mut bullets: Vec<Bullet> = Vec::new();
let mut enemies: Vec<Enemy> = Vec::new();
let columns = 6;
let rows = 4;
let spacing_x = screen_width() * 0.08;
let spacing_y = screen_height() * 0.09;
let mut enemy_dir = 0.2;
let min_x = 230.0;
let max_x = 730.0;

for row in 0..rows {
    for col in 0..columns {
        let enemy = Enemy::new(
            240.0 + col as f32 * spacing_x,
            25.0 + row as f32 * spacing_y,
            screen_width() * 0.055,
            screen_height() * 0.065,
            "assets/enemy.png",
            true,
            1.0,
        ).await;
        enemies.push(enemy);
    }
}

let mut shoot_cooldown = 0.0;


    loop {
        clear_background(WHITE);
        background.draw();
        for enemy in &enemies {
            enemy.get_image().draw();
        }
        player.get_image().draw();


        player.keypress();
        player.move_player();

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &gamebars, 1) {
                player.move_back_x();
            }
        }

        let time = get_frame_time();
        shoot_cooldown -= time;

         if is_key_down(KeyCode::Space) && shoot_cooldown <= 0.0 {
        shoot_cooldown = 0.75;

        let bullet = Bullet::new(
            player.image.get_x() + 25.0,
            player.image.get_y(),
            screen_width() * 0.02,
            screen_height() * 0.05,
            "assets/bullet.png",
            true,
            1.0,
            3.0,
        ).await;

        bullets.push(bullet);
    }

    for bullet_index in (0..bullets.len()).rev() {
        let bullet = &mut bullets[bullet_index];
        bullet.shoot();
        bullet.get_image().draw();

        if bullet.image.get_y() < -100.0 {
            bullet.image.clear();
            bullet.move_dir = vec2(0.0, 0.0);
            bullets.remove(bullet_index);
            continue;
        }

        for enemy_index in (0..enemies.len()).rev() {
            if check_collision(bullet.get_image(), enemies[enemy_index].get_image(), 1) {
                enemies.remove(enemy_index);
                bullets.remove(bullet_index);
                score += 100;
                lbl_score.set_text(format!("Score:\n{}", score));
                if score > highscore {
                highscore = score;
                }
                lbl_highscore.set_text(format!("Highscore:\n{}", highscore));
                break;
            }
        }

        if enemies.is_empty() {
            return ("win".to_string(), tm, score, highscore);
        }
    }


    let last_shot = 0.0;

if is_key_down(KeyCode::Space) && get_time() - last_shot > 1.0 {
}

// move entire fleet
    for enemy in &mut enemies {
        enemy.image.set_x(enemy.image.get_x() + enemy_dir * 2.0);
    }

    // find bounds of the fleet
    let leftmost = enemies.iter().map(|e| e.image.get_x()).reduce(f32::min).unwrap_or(0.0);
    let rightmost = enemies.iter().map(|e| e.image.get_x()).reduce(f32::max).unwrap_or(0.0);

    if rightmost >= max_x || leftmost <= min_x {
        for enemy in &mut enemies {
            enemy.image.set_x(enemy.image.get_x() - enemy_dir * 2.0);
            enemy.image.set_y(enemy.image.get_y() + 25.0);
        }
        enemy_dir *= -1.0;
    }

    if enemies.iter().any(|enemy| enemy.image.get_y() >= 650.0) {
        return ("lose".to_string(), tm, score, highscore);
    }

    gamebars.draw();
    lbl_score.draw();
    lbl_highscore.draw();

    if btn_exit.click() {
            return("".to_string(),tm, score, highscore);
        }

    if btn_return.click() {
            return("menu".to_string(),tm, score, highscore);
        }

        next_frame().await;
    }
}