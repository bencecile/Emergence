//! Code for managing the map between terrain position and entity

use crate::simulation::map::resources::MapResource;
use bevy::prelude::{Entity, Resource};

/// Helper for managing the mapping between a tile position and the terrain entity associated with
/// that position
///
/// It is generated by [`generate_terrain`](crate::simulation::generation::generate_terrain).
#[derive(Resource)]
pub struct TerrainEntityMap {
    /// The inner [`MapResource`] backing this resource
    pub inner: MapResource<Entity>,
}
