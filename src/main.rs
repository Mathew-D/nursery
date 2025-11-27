/*
By: <jeremiah headrick>
Date: 2025-09-24
Program Details: <makes nursery rhyme with image>
*/

mod modules;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "nursery".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tm = TextureManager::new();
    let mut lbl_out = Label::new("Hello\nWorld", 100.0, 700.0, 15);
    let btn_text = TextButton::new(100.0, 200.0, 200.0, 60.0, "Click Me 1", BLUE, GREEN, 15);
    let btn_two = TextButton::new(200.0, 300.0, 200.0, 60.0, "Click Me 2", BLUE, GREEN, 15);
    let btn_three = TextButton::new(300.0, 400.0, 200.0, 60.0, "Click Me 3", BLUE, GREEN, 15);
    let btn_exit = TextButton::new(400.0, 600.0, 200.0, 60.0, "EXIT", BLUE, GREEN, 8);

    tm.preload_with_loading_screen(&["assets/hump.png", "assets/wall.png", "assets/muffet.png", "assets/spider.png"], None).await;


    let mut img = StillImage::new(
        "assets/hump.png",
        100.0, // width
        200.0, // height
        500.0, // x position
        80.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    loop {
         use_virtual_resolution(1024.0, 768.0);
        clear_background(WHITE);

        if btn_text.click() {
            lbl_out.set_text ("Humpty Dumpty sat on a wall, humpty dumpty had a great fall, all the king's horses and all the king's men couldnt put humpty together again");
            
            img.set_preload(tm.get_preload("assets/wall.png").unwrap());
            
           
        }
        img.draw();
        if btn_two.click() {
            lbl_out.set_text("little miss muffet sat on her tuffet, eating her curds and whey");
            img.set_preload(tm.get_preload("assets/muffet.png").unwrap());
            
        }
        img.draw();
        if btn_three.click() {
            lbl_out.set_text("the itsy bitsy spider climbed up the waterspout,along came the rain and washed the spider out");
            img.set_preload(tm.get_preload("assets/spider.png").unwrap());
            
        }
        if btn_exit.click() {
            break;
        }
        img.draw();
        lbl_out.draw();
        next_frame().await;
    }
}
