#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile{
    Block,
    Empty,
}

impl Tile {

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!("{},"
            match self {
                Tile::Block => "1",
                Tile::Empty => " ",
}
