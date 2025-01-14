//! Keep track of the mouse cursor in world space, and convert it into a tile position, if
//! available.
use crate::graphics::terrain::TerrainTilemap;
use bevy::math::Vec4Swizzles;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::hex_grid::axial::AxialPos;
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize};
use bevy_ecs_tilemap::tiles::TilePos;

/// Initializes the [`CursorWorldPos`] and [`CursorTilePos`] resources, which are kept updated  
/// updated using [`update_cursor_pos`].
pub struct CursorTilePosPlugin;

impl Plugin for CursorTilePosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPos>()
            .init_resource::<CursorTilePos>()
            // FIXME: Ideally, this should be executed after the bevy_pancam plugin's
            // `camera_movement` and `camera_zoom` systems; but for now we don't have the tools to
            // specify this.
            .add_system_to_stage(CoreStage::Last, update_cursor_pos);
    }
}

/// Converts cursor screen position into a world position, taking into account any transforms
/// applied to the camera.
pub fn cursor_pos_in_world(
    windows: &Windows,
    cursor_pos: Vec2,
    cam_t: &Transform,
    cam: &Camera,
) -> Vec3 {
    let window = windows.primary();

    let window_size = Vec2::new(window.width(), window.height());

    // Convert screen position [0..resolution] to ndc [-1..1]
    // (ndc = normalized device coordinates)
    let ndc_to_world = cam_t.compute_matrix() * cam.projection_matrix().inverse();
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
    ndc_to_world.project_point3(ndc.extend(0.0))
}

/// The world position of the mouse cursor.
#[derive(Resource, Clone, Copy, Deref, DerefMut)]
pub struct CursorWorldPos(Vec3);

impl Default for CursorWorldPos {
    fn default() -> Self {
        Self(Vec3::new(f32::INFINITY, f32::INFINITY, 0.0))
    }
}

/// The tile position of the mouse cursor, if it lies over the map.
#[derive(Resource, Default, Clone, Copy)]
pub struct CursorTilePos(Option<TilePos>);

impl CursorTilePos {
    /// The position of the cursor in hex coordinates, if it is on the hex map.
    ///
    /// If the cursor is outside the map, this will return `None`.
    pub fn maybe_tile_pos(&self) -> Option<TilePos> {
        self.0
    }
}

/// Convert a world position into a tile position, if applicable.
pub fn tile_pos_from_world_pos(
    world_pos: &Vec2,
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
) -> Option<TilePos> {
    AxialPos::from_world_pos_row(world_pos, grid_size).as_tile_pos_given_map_size(map_size)
}

/// Keeps the cursor position updated based on [`CursorMoved`] events.
pub fn update_cursor_pos(
    windows: Res<Windows>,
    camera_query: Query<(&Transform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_world_pos_res: ResMut<CursorWorldPos>,
    mut cursor_tile_pos_res: ResMut<CursorTilePos>,
    terrain_tilemap_query: Query<
        (&TilemapSize, &TilemapGridSize, &Transform),
        With<TerrainTilemap>,
    >,
) {
    // We only have one camera.
    let (cam_t, cam) = camera_query.single();
    let (map_size, grid_size, map_transform) = terrain_tilemap_query.single();

    if let Some(cursor_moved) = cursor_moved_events.iter().last() {
        **cursor_world_pos_res = cursor_pos_in_world(&windows, cursor_moved.position, cam_t, cam);
    }

    // Grab the cursor position from the `Res<CursorPos>`
    let cursor_world_pos: Vec3 = cursor_world_pos_res.0;
    // We need to make sure that the cursor's world position is correct relative to the map
    // due to any map transformation.
    let cursor_map_pos: Vec2 = {
        // Extend the cursor_pos vec3 by 1.0
        let cursor_pos = Vec4::from((cursor_world_pos, 1.0));
        let cursor_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
        cursor_map_pos.xy()
    };

    cursor_tile_pos_res.0 = tile_pos_from_world_pos(&cursor_map_pos, map_size, grid_size);
}
