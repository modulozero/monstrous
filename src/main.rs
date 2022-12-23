// Copyright 2022 ModZero.
// SPDX-License-Identifier: 	AGPL-3.0-or-later

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::mouse::MouseMotion,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use defs::SurfaceDef;

mod defs;

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

#[derive(Component, Clone)]
struct TileTerrain {
    terrain_id: usize,
}

impl TileTerrain {
    fn def(&self) -> &SurfaceDef {
        &TERRAINS[self.terrain_id]
    }
}

#[derive(Bundle, Default)]
struct PawnBundle {
    position: TilePos,
    sprite: Sprite,
    transform: Transform,
    global_transform: GlobalTransform,
    texture: Handle<Image>,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

fn make_ground_layer(
    commands: &mut Commands,
    tilemap_size: TilemapSize,
    texture_handle: Handle<Image>,
    tile_size: TilemapTileSize,
) {
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
}

fn make_pawn(commands: &mut Commands, texture_handle: Handle<Image>, tile_size: TilemapTileSize) {
    commands.spawn(PawnBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        texture: texture_handle,
        sprite: Sprite {
            rect: Option::Some(Rect::new(
                3. * tile_size.x,
                4. * tile_size.y,
                4. * tile_size.x,
                5. * tile_size.y,
            )),
            ..Default::default()
        },
        visibility: Visibility { is_visible: true },
        ..Default::default()
    });
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("tileset.png");

    let tilemap_size = TilemapSize { x: 320, y: 320 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };

    make_ground_layer(
        &mut commands,
        tilemap_size,
        texture_handle.clone(),
        tile_size,
    );
    make_pawn(&mut commands, texture_handle, tile_size);
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    if buttons.pressed(MouseButton::Middle) {
        for ev in motion_evr.iter() {
            for (mut transform, mut _ortho) in query.iter_mut() {
                let direction = Vec3::new(ev.delta.x, ev.delta.y * -1.0, 0.0);
                transform.translation += direction;
            }
        }
    }
}


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1270.0,
                        height: 720.0,
                        title: String::from("Monstrous"),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(mouse_motion)
        .run();
}
