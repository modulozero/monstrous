// Copyright 2022 ModZero.
// SPDX-License-Identifier: 	AGPL-3.0-or-later

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SurfaceDef<'a> {
    pub label: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub texture_index: u32,
    pub support: f32,
}

#[derive(Component, Clone)]
struct TileTerrain {
    terrain_id: usize,
}

impl TileTerrain {
    fn def(&self) -> &SurfaceDef {
        &TERRAINS[self.terrain_id]
    }
}

const TERRAINS: [SurfaceDef; 3] = [
    SurfaceDef {
        label: "Mud",
        name: "mud",
        description: "Soil saturated with water.",
        texture_index: 13,
        support: 20.0,
    },
    SurfaceDef {
        label: "Grass",
        name: "grass",
        description: "Green. Try to touch it.",
        texture_index: 14,
        support: 40.0,
    },
    SurfaceDef {
        label: "Sand",
        name: "sand",
        description: "Gets everywhere, ruins vanilla sex fantasies.",
        texture_index: 15,
        support: 20.0,
    },
];

pub fn make_ground_layer(
    commands: &mut Commands,
    tilemap_size: TilemapSize,
    texture_handle: Handle<Image>,
    tile_size: TilemapTileSize,
) -> Entity {
    let mut tile_storage = TileStorage::empty(tilemap_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };

            let tile_terrain = TileTerrain { terrain_id: 1 };
            let tile_entity = commands
                .spawn((
                    tile_terrain.clone(),
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(tile_terrain.def().texture_index),
                        ..Default::default()
                    },
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage.clone(),
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
    tilemap_entity
}
