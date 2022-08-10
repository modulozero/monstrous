use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    render::texture::ImageSettings, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_ecs_tilemap::prelude::*;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle = asset_server.load("tileset.png");

    let tilemap_size = TilemapSize { x: 320, y: 320 };
    let mut tile_storage = TileStorage::empty(tilemap_size);
    let tilemap_entity = commands.spawn().id();

    for x in 0..320u32 {
        for y in 0..320u32 {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }
    
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 32.0, y: 32.0 },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(&tilemap_size, &tile_size, 0.0),
            ..Default::default()
        });
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
