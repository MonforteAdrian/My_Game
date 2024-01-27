//  //use bevy::prelude::*;
//
//  // We generate the complete map at the start dwarf fortress style.
//  // There should be a vector/array of chunks depending on the size of the map choosed before
//  // The position of the array in the map is calculated by
//  //  the index in the vector / sqrt(len) to get the x and index in the vector % 4 to get the y
//  // Every chunk should have a 3d vector/array x * y * z fixed size
//  const CHUNK_SIZE: usize = 16;
//
//  #[derive(Copy, Clone, PartialEq)]
//  pub enum TileType {
//      Empty,
//      Grass,
//      Sand,
//      Stone,
//  }
//
//  #[derive(Clone)]
//  pub struct Chunk {
//      pub tiles: [[[TileType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
//  }
//
//  pub struct Map {
//      pub chunks: Vec<Chunk>,
//  }
//
//  impl Chunk {
//      fn new() -> Self {
//          Self {
//              tiles: [[[TileType::Grass; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
//          }
//      }
//
//      pub fn render(&self) {}
//  }
//
//  impl Map {
//      pub fn new() -> Self {
//          Self {
//              chunks: vec![Chunk::new(); 25],
//          }
//      }
//  }
