use raylib::prelude::*;


pub fn menu() {
    //window open
    let (mut rl, thread) = raylib::init().size(640, 480).title("pong").build();
    //main game loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.set_target_fps(120);
        d.draw_fps(640 / 2 - 40, 40);
        d.draw_text("Press ENTER to start", 200, 200, 20, Color::BLACK);
        d.draw_text("Press ESC to quit", 200, 240, 20, Color::BLACK);

        if d.is_key_down(KeyboardKey::KEY_ENTER) {
            break;
        }
    }
}