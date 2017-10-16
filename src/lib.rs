extern crate piston;
use piston::event_loop::*;
use piston::input::*;

mod game;
use game::*;

mod gfx;
use gfx::Graphics;
use gfx::DisplaySettings;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

fn init_graphics(game: &Game) -> Graphics {
    let window_size = 600;
    let spacing_between_tiles = 2.0;
    let display_settings = DisplaySettings::new(window_size, game.width(), spacing_between_tiles);
    Graphics::new(display_settings, game.board())
}

pub fn run(size: usize, initial_state: Vec<(usize, usize)>) {
    let mut game = Game::new(size, &initial_state);
    let mut graphics = init_graphics(&game);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(graphics.window()) {
        if let Some(Button::Keyboard(_)) = e.press_args() {
            game.next_gen();
            graphics.update_state(game.board());
        }
        graphics.handle_event(e);
    }
}
