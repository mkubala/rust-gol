use super::piston::window::WindowSettings;
use super::piston::input::*;
use super::glutin_window::GlutinWindow as Window;
use super::opengl_graphics::{ GlGraphics, OpenGL };
use super::piston_graphics::*;
use super::settings::DisplaySettings;
use super::tile::Tile;

pub struct Graphics {
    gl: GlGraphics,
    tiles: Vec<Tile>,
    display_settings: DisplaySettings,
    window: Window
}

impl Graphics {
    pub fn new(display_settings: DisplaySettings, cells: &Vec<bool>) -> Graphics {
        let opengl = OpenGL::V3_2;

        let window: Window = WindowSettings::new(
                "Game of life",
                [display_settings.window_size(), display_settings.window_size()]
            )
            .resizable(false)
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let tiles = cells.into_iter()
            .enumerate()
            .map(|e| {
                let (idx, is_alive) = e;
                Tile::new(idx % display_settings.tiles_in_row(), idx / display_settings.tiles_in_row(), *is_alive)
            })
            .collect();
        Graphics {
            gl: GlGraphics::new(opengl),
            tiles,
            display_settings,
            window
        }
    }

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn update_state(&mut self, cells: &Vec<bool>) {
        for tile in &mut self.tiles {
            let alive = cells[tile.y * self.display_settings.tiles_in_row() + tile.x];
            tile.set_alive(alive);
        }
    }

    pub fn handle_event(&mut self, e: Event) {
        if let Some(args) = e.render_args() {
            let tiles = &mut self.tiles;
            let display_settings = &self.display_settings;

            self.gl.draw(args.viewport(), |c, mut gl| {
                clear(super::colors::BACKGROUND, gl);
                for tile in tiles {
                    tile.render(&display_settings, &c, &mut gl);
                }
            });
        }

        if let Some(_) = e.update_args() {
            for tile in &mut self.tiles {
                tile.update();
            }
        }
    }
}