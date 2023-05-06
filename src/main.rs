extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rand::Rng;

pub fn main() {
    // Normal vars
    let screen_height: u8=20;
    let screen_width: u8=20;
    let cell_size: u8=30;
    let fps_limit: u8=8;

    let mut rng=rand::thread_rng();

    let mut apple_pos: (u8,u8)=(rng.gen_range(0..screen_width),rng.gen_range(0..screen_height));
    let mut snake_tail: std::collections::VecDeque<(i8,i8)> = std::collections::VecDeque::new();
    snake_tail.push_back((1,1));
    snake_tail.push_back((1,2));
    let mut snake_direction: u8 = 2;

    // SDL2 Vars
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Snake", screen_width as u32 * cell_size as u32, screen_height as u32 * cell_size as u32).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Main loop
    'main_loop: loop {
        // Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::W), .. } | Event::KeyDown { keycode: Some(Keycode::Up), .. } => {if snake_direction != 2 {snake_direction=0;}}, // up
                Event::KeyDown { keycode: Some(Keycode::D), .. } | Event::KeyDown { keycode: Some(Keycode::Right), .. } => {if snake_direction != 3 {snake_direction=1;}}, // right
                Event::KeyDown { keycode: Some(Keycode::S), .. } | Event::KeyDown { keycode: Some(Keycode::Down), .. } => {if snake_direction != 0 {snake_direction=2;}}, // down
                Event::KeyDown { keycode: Some(Keycode::A), .. } | Event::KeyDown { keycode: Some(Keycode::Left), .. } => {if snake_direction != 1 {snake_direction=3;}}, // left
                _ => {}
            }
        }

        // Movement
        let current_xy = snake_tail.back().unwrap();
        let mut next_pos: (i8, i8) = *current_xy;
        match snake_direction {
            0 => next_pos=(current_xy.0,current_xy.1-1), // up
            1 => next_pos=(current_xy.0+1,current_xy.1), // right
            2 => next_pos=(current_xy.0,current_xy.1+1), // down
            3 => next_pos=(current_xy.0-1,current_xy.1), // left
            _ => {}
        }

        if snake_tail.contains(&next_pos) || next_pos.0 < 0 || next_pos.0 >= screen_width as i8 || next_pos.1 < 0 || next_pos.1 >= screen_height as i8 {
            // Game over
            panic!("GAME OVER!");
        } else {
            snake_tail.push_back(next_pos);
            if apple_pos==(next_pos.0 as u8,next_pos.1 as u8){
                apple_pos=(rng.gen_range(0..screen_width),rng.gen_range(0..screen_height));
            }else{
                snake_tail.pop_front();
            }
        }

        // Drawing to the screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        for snake_segment in &snake_tail {
            canvas.fill_rect(Rect::new((snake_segment.0 as i16 * cell_size as i16) as i32, (snake_segment.1 as i16 * cell_size as i16) as i32, cell_size as u32, cell_size as u32)).unwrap();
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new((apple_pos.0 as i16 * cell_size as i16) as i32, (apple_pos.1 as i16 * cell_size as i16) as i32, cell_size as u32, cell_size as u32)).unwrap();

        canvas.present();

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / fps_limit as u32));
    }
}
