use macroquad::{
    miniquad::window::{set_window_position, set_window_size},
    prelude::*,
};

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        //Basic window config
        clear_background(WHITE);
        let x_pos = -1930i32 as u32;
        set_window_position(x_pos, 0);
        set_window_size(1920, 1050);

        //Draw the external walls
        draw_line(40.0, 40.0, 1900.0, 40.0, 15.0, BLACK);
        draw_line(47.5, 40.0, 47.5, 1000.0, 15.0, BLACK);
        draw_line(1892.5, 40.0, 1892.5, 1000.0, 15.0, BLACK);
        draw_line(40.0, 1000.0, 1900.0, 1000.0, 15.0, BLACK);

        //Draw the internal obstacles
        draw_line(40.0, 320.0, 1400.0, 320.0, 15.0, BLACK);
        draw_line(500.0, 640.0, 1900.0, 640.0, 15.0, BLACK);

        //Draw the checkpoints
        draw_line(1400.0, 320.0, 1885.0, 320.0, 10.0, BLUE);
        draw_line(55.0, 640.0, 500.0, 640.0, 10.0, BLUE);

        //Draw the finish line
        draw_line(140.0, 47.5, 140.0, 312.5, 20.0, GREEN);

        next_frame().await
    }
}
