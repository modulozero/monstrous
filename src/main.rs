use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("tileset.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(3, 3),
            ChunkSize(8, 8),
            TileSize(8.0, 8.0),
            TextureSize(24.0, 24.0)
        ),
        0u16,
        0u16
    );
    layer_builder.set_all(TileBundle::default());

    map_query.build_layer(&mut commands, layer_builder, texture_handle);
    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
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
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        .add_system(mouse_motion)
        .run();
}
