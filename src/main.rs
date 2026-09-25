/*
By: <Austin>
Date: 2026-09-21  
Program Details: <Program Description Here>
*/

mod ui;
mod utils;


use macroquad::prelude::*;
use macroquad::input::KeyCode;

use crate::ui::grid::draw_grid;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
 use crate::utils::preload_image::TextureManager;
use crate::utils::collision::check_collision;
    use crate::ui::label::Label;


/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Maze".to_string(),
        window_width: 1624,
        window_height: 1168,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {


    const MOVE_SPEED: f32 = 300.0;

let tm = TextureManager::new();

    let btn_exit = TextButton::new(
        100.0,
        1200.0,
        200.0,
        60.0,
        "Exit",
        RED,
        GREEN,
        30
    );


 tm.preload_all(&["assets/tom.png" , "assets/maze_1.png" , "assets/pumba.png" , "assets/g_bug.png", "assets/wall.png" , "assets/wall_vert.png" , "assets/jerry.png"]).await;

 let mut lbl_out = Label::new("You are trapped in a maze to escape\n you need to go through Pumba the pig.", 1150.0, 1150.0, 30);


    
let mut img = StillImage::new(
        "",     // Empty string creates a transparent image
        50.0,  // width
        50.0,  // height
        40.0,  // x position
        30.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

        let mut img_maze = StillImage::new(
        "",     // Empty string creates a transparent image
        2824.0,  // width
        1368.0,  // height
        -300.0,  // x position
        -130.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

        let mut img_pumba = StillImage::new(
        "",     // Empty string creates a transparent image
        200.0,  // width
        100.0,  // height
        2050.0,  // x position
        1050.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

        let mut img_bug = StillImage::new(
        "",     // Empty string creates a transparent image
        104.0,  // width
        108.0,  // height
        1160.0,  // x position
        980.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut img_jerry = StillImage::new(
        "",     // Empty string creates a transparent image
        104.0,  // width
        108.0,  // height
        1160.0,  // x position
        980.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

     let mut img_outside_wall_left = StillImage::new(
        "",     // Empty string creates a transparent image
        54.0,  // width
        238.0,  // height
        1960.0,  // x position
        1080.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


     let mut img_outside_wall_right = StillImage::new(
        "",     // Empty string creates a transparent image
        54.0,  // width
        238.0,  // height
        2230.0,  // x position
        1080.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


     let mut img_outside_wall_down = StillImage::new(
        "",     // Empty string creates a transparent image
        304.0,  // width
        58.0,  // height
        1960.0,  // x position
        1280.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


            let mut img_wall = StillImage::new(
        "",     // Empty string creates a transparent image
        154.0,  // width
        60.0,  // height
        1250.0,  // x position
        890.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


    let mut wall_left = true;

    img.set_preload(tm.get_preload("assets/tom.png").unwrap());
    img_maze.set_preload(tm.get_preload("assets/maze_1.png").unwrap());
        img_pumba.set_preload(tm.get_preload("assets/pumba.png").unwrap());
    img_bug.set_preload(tm.get_preload("assets/g_bug.png").unwrap());
img_wall.set_preload(tm.get_preload("assets/wall.png").unwrap());
img_outside_wall_left.set_preload(tm.get_preload("assets/wall_vert.png").unwrap());
img_outside_wall_right.set_preload(tm.get_preload("assets/wall_vert.png").unwrap());
img_outside_wall_down.set_preload(tm.get_preload("assets/wall.png").unwrap());
img_jerry.set_preload(tm.get_preload("assets/jerry.png").unwrap());


let mut bug = false;

    loop {

let collision = check_collision(&img, &img_bug, 1);

    if bug == true {
img_bug.clear();
        }
 let mut move_dir = vec2(0.0, 0.0);

    // Keyboard input
    if is_key_down(KeyCode::Right) {
        move_dir.x += 2.0;
    }
    if is_key_down(KeyCode::Left) {
        move_dir.x -= 2.0;
    }
    if is_key_down(KeyCode::Down) {
        move_dir.y += 2.0;
    }
    if is_key_down(KeyCode::Up) {
        move_dir.y -= 2.0;
    }


    // Normalize the movement to prevent faster diagonal movement
    if move_dir.length() > 0.0 {
        move_dir = move_dir.normalize();
    }

    // Apply movement based on frame time
    let movement = move_dir * MOVE_SPEED * get_frame_time();

    // Save old position in case of collision
    let old_pos = img.pos();

    // Move X first
    if movement.x != 0.0 {
        img.set_x(img.get_x() + movement.x);
        if check_collision(&img, &img_maze, 1) {
            img.set_x(old_pos.x); // Undo if collision happens
        }
                    if check_collision(&img, &img_wall, 1){
                img.set_x(old_pos.x);
                img.set_y(old_pos.y + 20.0);
            }

                     if check_collision(&img, &img_pumba, 1) && bug == false{
                img.set_x(old_pos.x);
                lbl_out.set_text("Mhhhhh, me Pumba hungry, if only a had a bug to eat!");
            }
            else if check_collision(&img, &img_pumba, 1) && bug == true {
lbl_out.set_text("Mhhhhh, me Pumba Full, I let you through");
            }
    }

    // Move Y next
    if movement.y != 0.0 {
        img.set_y(img.get_y() + movement.y);
            if check_collision(&img, &img_maze, 1)  {
                img.set_y(old_pos.y); // Undo if collision happens
            }
            if check_collision(&img, &img_wall, 1){
                img.set_y(old_pos.y + 20.0);
                img.set_x(old_pos.x);
            }

            if check_collision(&img, &img_pumba, 1) && bug == false{
                img.set_y(old_pos.y);
                lbl_out.set_text("Mhhhhh, me Pumba hungry, if only a had a bug to eat!");
            }
            else if check_collision(&img, &img_pumba, 1) && bug == true {
                lbl_out.set_text("Mhhhhh, me Pumba Full, I let you through");
            }
    }
        if collision {
            bug = true;
        }
// Update the module's position
        clear_background(WHITE);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
if btn_exit.click() {
break;
}




        let mut x = img_wall.get_x();
        
if x <= 800.0{
    wall_left = true;
    img_wall.set_x(x);
}

if wall_left == true {
    x += 2.0;
    img_wall.set_x(x);
}

if x >= 1300.0{
    wall_left = false;
    img_wall.set_x(x);
}

if wall_left == false {
    x -= 2.0;
    img_wall.set_x(x);
}


img_maze.draw();
img.draw();
img_pumba.draw();
img_bug.draw();
img_wall.draw();
img_outside_wall_left.draw();
img_outside_wall_right.draw();
img_outside_wall_down.draw();
lbl_out.draw();
img_jerry.draw();

draw_grid(50.0, RED);
        next_frame().await;
    }
}
