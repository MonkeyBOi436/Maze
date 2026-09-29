/*
By: <Austin>
Date: 2026-09-21  
Program Details: <Program Description Here>
*/

use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;

pub async fn run() -> String {
    
let tm = TextureManager::new();
tm.preload_all(&["assets/panda.png"]).await;

let mut img_panda = StillImage::new(
        "",     // Empty string creates a transparent image
        200.0,  // width
        200.0,  // height
        400.0,  // x position
        230.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


    let btn_exit = TextButton::new(
        100.0,
        600.0,
        200.0,
        60.0,
        "Exit",
        BLUE,
        GREEN,
        30
    );




img_panda.set_preload(tm.get_preload("assets/panda.png").unwrap());

    loop {
        clear_background(DARKGRAY);
        draw_text("Hello in this game you will be going through a maze", 20.0, 40.0, 30.0, WHITE);

        draw_text("You can use arrow keys to move ", 20.0, 80.0, 30.0, WHITE);

        draw_text("To start the game press space bar", 20.0, 130.0, 30.0, WHITE);

if btn_exit.click() {
    return "_end".to_string();
}

        if is_key_pressed(KeyCode::Space) {
            break;
        }

        img_panda.draw();   
        next_frame().await;
    }
    "game".to_string()
}
