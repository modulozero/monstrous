// Copyright 2022 ModZero.
// SPDX-License-Identifier: 	AGPL-3.0-or-later

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::{mouse::{MouseMotion, MouseButtonInput}, ButtonState},
    math::Vec4Swizzles,
    prelude::*,
    render::camera::RenderTarget,
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

#[derive(Component)]
struct PlayerCharacter {}

#[derive(Component, Clone)]
struct TileTerrain {
    terrain_id: usize,
}

impl TileTerrain {
    fn def(&self) -> &SurfaceDef {
        &TERRAINS[self.terrain_id]
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct PawnPos {
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct PawnDest {
    pub x: u32,
    pub y: u32,
}

#[derive(Bundle, Default)]
struct PawnBundle {
    position: PawnPos,
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

fn make_pawn(
    commands: &mut Commands,
    tilemap_entity: Entity,
    texture_handle: Handle<Image>,
    tile_size: &TilemapTileSize,
    map_size: &TilemapSize,
) {
    let pos = PawnPos {
        x: map_size.x / 2,
        y: map_size.y / 2,
    };
    commands
        .spawn(PawnBundle {
            position: pos,
            transform: Transform::from_xyz(
                (pos.x as f32) * tile_size.x,
                (pos.y as f32) * tile_size.y,
                1.0,
            ),
            texture: texture_handle,
            visibility: Visibility { is_visible: true },
            ..Default::default()
        })
        .insert(PlayerCharacter {})
        .set_parent(tilemap_entity);
    info!("Pawn at {:?}", pos);
}

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

fn pawn_control(
    mut commands: Commands,
    wnds: Res<Windows>,
    query: Query<Entity, With<PlayerCharacter>>,
    mut buttons_evr: EventReader<MouseButtonInput>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
) {
    for ev in buttons_evr.iter() {
        if (ev.button == MouseButton::Right) && (ev.state == ButtonState::Released) {
            let (cam_gt, cam) = camera_q.single();
            let wnd = if let RenderTarget::Window(id) = cam.target {
                wnds.get(id).unwrap()
            } else {
                wnds.get_primary().unwrap()
            };
    
            if let Some(screen_pos) = wnd.cursor_position() {
                query.for_each(|entity| {
                    if let Some(world_ray) = cam.viewport_to_world(cam_gt, screen_pos) {
                        let (map_size, map_grid_size, map_type, map_t) = tilemap_q.single();
    
                        let tilemap_pos = (map_t.compute_matrix().inverse()
                            * Vec4::from((world_ray.origin, 1.0)))
                        .xy();
                        info!("WP {:?} {:?}", world_ray.origin, tilemap_pos);
                        if let Some(tile_pos) =
                            TilePos::from_world_pos(&tilemap_pos, map_size, map_grid_size, map_type)
                        {
                            info!("{:?}", tile_pos);
                            commands.entity(entity).insert(PawnDest {
                                x: tile_pos.x,
                                y: tile_pos.y,
                            });
                        }
                    }
                });
            }
        }
    }
    
}

type ChangedPawns = Or<(Changed<PawnPos>, With<PawnDest>)>;

fn pawn_motion(
    mut commands: Commands,
    mut pawn_q: Query<(Entity, &mut PawnPos, &mut Transform, Option<&PawnDest>), ChangedPawns>,
    tilemap_q: Query<&TilemapTileSize>,
) {
    let map_tile_size = tilemap_q.single();
    pawn_q.for_each_mut(|(entity, mut pos, mut transform, dest)| match dest {
        Some(d) => {
            pos.x = d.x;
            pos.y = d.y;
            transform.translation = Vec3::from((
                (pos.x as f32) * map_tile_size.x,
                (pos.y as f32) * map_tile_size.y,
                1.0,
            ));
            commands.entity(entity).remove::<PawnDest>();
        }
        None => {
            transform.translation = Vec3::from((
                (pos.x as f32) * map_tile_size.x,
                (pos.y as f32) * map_tile_size.y,
                1.0,
            ));
        }
    })
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
