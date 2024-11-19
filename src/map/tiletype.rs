pub enum TileType {
    Wall,
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
        TileType::Wall => true,
        _ => false,
    }
}
