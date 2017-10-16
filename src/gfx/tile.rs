use super::settings::DisplaySettings;
use super::opengl_graphics::GlGraphics;
use super::piston_graphics::*;

pub struct Tile {
    pub x: usize,
    pub y: usize,
    scale: f64,
    is_alive: bool
}

impl Tile {
    pub fn new(x: usize, y: usize, is_alive: bool) -> Tile {
        Tile {
            x,
            y,
            is_alive,
            scale: 0.0
        }
    }

    pub fn set_alive(&mut self, alive: bool) {
        self.is_alive = alive;
    }

    pub fn render(&self, settings: &DisplaySettings, context: &Context, gl: &mut GlGraphics) {
        let tile_size = settings.tile_size();
        let spacing = settings.spacing();
        let square = rectangle::square(0.0, 0.0, tile_size);

        let index_to_coord = |idx| {
            (tile_size / 2.0) + spacing + idx as f64 * tile_size + idx as f64 * spacing
        };

        let scale = self.scale;
        let transform = context.transform.trans(index_to_coord(self.x), index_to_coord(self.y))
                                   .scale(scale, scale)
                                   .trans(-settings.tile_size() / 2.0, -settings.tile_size() / 2.0);
            rectangle([1.0, 0.0, 0.0, scale as f32], square, transform, gl);
    }

    pub fn update(&mut self) {
        let factor = 0.02;
        if self.is_alive && self.scale < 1.0 {
            self.scale += factor;
        } else if !self.is_alive && self.scale > 0.0 {
            self.scale -= factor;
        }
    }
}