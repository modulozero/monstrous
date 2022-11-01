// Copyright 2022 ModZero.
// SPDX-License-Identifier: 	AGPL-3.0-or-later

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::mouse::MouseMotion,
    prelude::*,
    render::texture::ImageSettings,
};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

mod defs;

fn make_ground_layer(
    commands: &mut Commands,
    tilemap_size: TilemapSize,
    texture_handle: Handle<Image>,
    tile_size: TilemapTileSize,
) {
    let mut tile_storage = TileStorage::empty(tilemap_size);
    let tilemap_entity = commands.spawn().id();
    let mut random = thread_rng();

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture: TileTexture(random.gen_range(13..=19)),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: tile_size.into(),
            size: tilemap_size,
            storage: tile_storage.clone(),
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                0.0,
            ),
            ..Default::default()
        });
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle = asset_server.load("tileset.png");

    let tilemap_size = TilemapSize { x: 320, y: 320 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };

    make_ground_layer(
        &mut commands,
        tilemap_size,
        texture_handle,
        tile_size,
    );
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
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Monstrous"),
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(mouse_motion)
        .run();
}
