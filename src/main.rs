// Copyright 2022 ModZero.
// SPDX-License-Identifier: 	AGPL-3.0-or-later

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::mouse::MouseMotion,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;

use pawns::{make_pawn, pawn_control, pawn_motion};
use terrain::make_ground_layer;

mod pawns;
mod terrain;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let tileset_texture_handle = asset_server.load("tileset.png");
    let pawn_texture_handle: Handle<Image> = asset_server.load("pawn.png");

    let tilemap_size = TilemapSize { x: 320, y: 320 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };

    let tilemap_entity = make_ground_layer(
        &mut commands,
        tilemap_size,
        tileset_texture_handle,
        tile_size,
    );
    make_pawn(
        &mut commands,
        tilemap_entity,
        pawn_texture_handle,
        &tile_size,
        &tilemap_size,
    );
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for ev in motion_evr.iter() {
        if buttons.pressed(MouseButton::Middle) {
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
        .add_system(pawn_control)
        .add_system(pawn_motion)
        .run();
}
