use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    math::Vec4Swizzles,
    prelude::*,
    render::camera::RenderTarget,
};
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
pub struct PlayerCharacter {}

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

pub fn make_pawn(
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

pub fn pawn_control(
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
                        if let Some(tile_pos) =
                            TilePos::from_world_pos(&tilemap_pos, map_size, map_grid_size, map_type)
                        {
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

pub fn pawn_motion(
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
