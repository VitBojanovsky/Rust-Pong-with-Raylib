use raylib::prelude::*;
use std::time::Duration;
mod launcher;

fn main() {
    launcher::menu();
    let height = 480;
    let width = 640;
    //raketa1
    let square_x = 20;
    let mut square_y = 20;
    //raketa2
    let square2_x = width - square_x - 15;
    let mut square2_y = 400;
    //raketa border
    let mut borderx_raketa1;
    let mut borderx_raketa2;
    let mut bordery_raketa1;
    let mut bordery_raketa2;
    //ball
    let mut ballx: f64 = (width as f64) / 2.0 - 7.0;
    let mut bally: f64 = (height as f64) / 2.0 - 7.0;
    //ball movement
    let ball_speed = 1.3;
    let mut ball_speed_x = 1.3;
    let mut ball_speed_y = 1.3;
    //score
    let mut score1: i8 = 0;
    let mut score2: i8 = 0;

    //open window
    let (mut rl, thread) = raylib::init().size(width, height).title("pong").build();
    //main game loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.set_target_fps(120);
        d.draw_fps(width / 2 - 40, 40);
        d.draw_rectangle(square_x, square_y, 15, 50, Color::RED);
        d.draw_rectangle(square2_x, square2_y, 15, 50, Color::BLUE);
        d.draw_circle(ballx as i32, bally as i32, 7.0, Color::GREEN);
        d.draw_text(&format!("Score: {}", score1), 20, 20, 20, Color::BLACK);
        d.draw_text(
            &format!("Score: {}", score2),
            width - 100,
            20,
            20,
            Color::BLACK,
        );

        //raket movement
        if d.is_key_down(KeyboardKey::KEY_W) && square_y > 0 {
            square_y -= 2;
        }
        if d.is_key_down(KeyboardKey::KEY_S) && square_y < height - 50 {
            square_y += 2;
        }
        if d.is_key_down(KeyboardKey::KEY_UP) && square2_y > 0 {
            square2_y -= 2;
        }
        if d.is_key_down(KeyboardKey::KEY_DOWN) && square2_y < height - 50 {
            square2_y += 2;
        }
        //recalculate racket borders x
        borderx_raketa1 = square_x + 15;
        borderx_raketa2 = square2_x;
        //recalculate racket borders y
        bordery_raketa1 = square_y;
        bordery_raketa2 = square2_y + 50;
        //ball movement
        ballx += ball_speed * ball_speed_x;
        bally += ball_speed * ball_speed_y;
        //bounce ball
        if bally > (height as f64) - 7.0 || bally < 7.0 {
            ball_speed_y *= -1.0;
        }
        if ballx > (width as f64) - 7.0 || ballx < 7.0 {
            ball_speed_x *= -1.0;
        }
        //bounce ball from racket and score
        if ballx < (borderx_raketa1 as f64) + 7.0
            && bally > (bordery_raketa1 as f64)
            && bally < (bordery_raketa1 as f64) + 50.0
        {
            ball_speed_x *= -1.0;
            score1 += 1;
        }
        if ballx > (borderx_raketa2 as f64) - 7.0
            && bally > (bordery_raketa2 as f64) - 50.0
            && bally < (bordery_raketa2 as f64)
        {
            ball_speed_x *= -1.0;
            score2 += 1;
        }
        if ballx - 7.0 < 29.0 {
            score1 -= 2;
        }
        if ballx + 7.0 > (width as f64) - 29.0 {
            score2 -= 2;
        }
        //reset ball
        if ballx - 7.0 < 29.0 || ballx + 7.0 > (width as f64) - 29.0 {
            ballx = (width as f64) / 2.0 - 7.0;
            bally = (height as f64) / 2.0 - 7.0;
            ball_speed_x = 1.0;
            ball_speed_y = 1.0;
            square2_y = 400;
        }
        //win condition
        if score1 >= 3 || score2 >= 3 {
            break;
        }
    }
    // After breaking out of the main loop, show the victory message for 10 seconds
    let winner_text;
    let winner_color;
    if score1 >= 3 {
        winner_text = "Player 1 Wins!";
        winner_color = Color::RED;
    } else if score2 >= 3 {
        winner_text = "Player 2 Wins!";
        winner_color = Color::BLUE;
    } else {
        return;
    }
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_millis(10000) {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text(
            winner_text,
            width / 2 - 100,
            height / 2 - 20,
            40,
            winner_color,
        );
        d.set_target_fps(60);
    }
}
