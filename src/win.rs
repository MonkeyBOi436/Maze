/*
By: <Austin>
Date: 2026-09-21  
Program Details: <Program Description Here>
*/

use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
use crate::ui::still_image::StillImage;


pub async fn run() -> String {
    
let tm = TextureManager::new();
tm.preload_all(&["assets/win.png"]).await;

let mut img_win = StillImage::new(
        "",     // Empty string creates a transparent image
        400.0,  // width
        500.0,  // height
        400.0,  // x position
        200.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;



img_win.set_preload(tm.get_preload("assets/win.png").unwrap());

    loop {
        clear_background(DARKBLUE);
        draw_text("You Win!", 20.0, 40.0, 30.0, WHITE);
        draw_text("Press Space to exit", 20.0, 80.0, 20.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            break;
        }

        img_win.draw();   
        next_frame().await;
    }
    "_end".to_string()
}
