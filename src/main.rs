extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use draw::to_coord;
use piston_window::*;
use piston_window::types::Color;

use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (50, 50);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = Window.next() {
        if let some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
