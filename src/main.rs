mod snake;
mod game;
mod drawing;

extern crate piston_window;
extern crate rand;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use drawing::to_gui_coord_u32;

const BACKGROUND_COLOR: Color = [0.204, 0.286, 0.369, 1.0];




fn main() {
    let (width, height) = (20,20);
    let mut window: PistonWindow =
        WindowSettings::new("Rust Snake",
                            [to_gui_coord_u32(width),
                                 to_gui_coord_u32(height)])
        .exit_on_esc(true).build().unwrap();
    let mut game = Game::new(width, height);

    while let Some(event) = window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key)
        }

        window.draw_2d(&event, |c, g, _|{
           clear(BACKGROUND_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg|{
           game.update(arg.dt)
        });
    }
}