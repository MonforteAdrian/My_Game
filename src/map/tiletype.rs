#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TileType {
    Block,
    Floor,
}

pub fn tile_walkable(tiletype: TileType) -> bool {
    match tiletype {
        TileType::Floor => true,
        _ => false,
    }
}

pub fn tile_opaque(tiletype: TileType) -> bool {
    match tiletype {
        TileType::Block => true,
        _ => false,
    }
}
