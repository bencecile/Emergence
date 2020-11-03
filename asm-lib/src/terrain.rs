use bevy::prelude::*;
use crate::utils::Position;

struct Tile{}

pub fn build_tile(position: Position){
    (Tile{}, position);
}

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system(render_terrain.system());
    }
}

fn render_terrain(_tile: &Tile, position: &Position){    
    println!("Tile: ({}, {})", position.x, position.y);    
}