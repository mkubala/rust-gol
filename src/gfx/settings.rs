pub struct DisplaySettings {
    window_size: u32,
    tiles_in_row: usize,
    spacing: f64
}

impl DisplaySettings {
    pub fn new(window_size: u32, tiles_in_row: usize, spacing: f64) -> DisplaySettings {
        DisplaySettings {
            window_size,
            tiles_in_row,
            spacing
        }
    }

    pub fn tiles_in_row(&self) -> usize {
        self.tiles_in_row
    }

    pub fn tile_size(&self) -> f64 {
        (self.window_size as f64 - self.spacing) / self.tiles_in_row as f64 - self.spacing
    }

    pub fn spacing(&self) -> f64 {
        self.spacing
    }

    pub fn window_size(&self) -> u32 {
        self.window_size
    }
}