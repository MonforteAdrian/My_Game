use crate::resources::tile::Tile;
use std::ops::{Deref, DerefMut};

pub struct TileMap {
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn empty(height: u16, width: u16) -> Self {
        let map = (0..height).into_iter()
                .map(|_| (0..width).into_iter()
                    .map(|_| Tile::Empty)
                .collect())
            .collect();
        Self {
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!("Map ({}, {})\n", self.width, self.height);
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    // Getters

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
