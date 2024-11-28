#![allow(clippy::type_complexity)]

use bevy::{ecs::system::SystemId, math::ivec2};
use bevy::prelude::*;
use bevy::utils::HashMap;
use piece::{spawn_piece, Active};

mod piece;
mod srs;

const BOARD_SIZE: IVec2 = IVec2::new(10, 20);
const TILE_SIZE: i32 = 31;

#[derive(Resource)]
struct AtlasTextureHandle {
    data: Handle<Image>
}

#[derive(Resource)]
struct OneshotSystems(HashMap<String, SystemId>);
impl FromWorld for OneshotSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = OneshotSystems(HashMap::new());
        systems.0.insert(
            "spawn_piece".into(),
            world.register_system(spawn_piece)
        );
        systems.0.insert(
            "check_board".into(),
            world.register_system(check_board)
        );
        systems
    }
}

#[derive(Component)]
struct Wall; 

#[derive(Component)]
struct Block {
    grid_pos: IVec2
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_board, spawn_piece).chain())
        .add_systems(Update, (
            piece::move_piece,
        ))
        .init_resource::<OneshotSystems>()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn( Camera2dBundle::default() );
    commands.insert_resource( AtlasTextureHandle { data: asset_server.load("Tetr-Skin.png") } );
}

fn get_board_pos() -> Vec<IVec2> {
    let half_width = BOARD_SIZE.x / 2;
    let half_height = BOARD_SIZE.y / 2;
    let mut board: Vec<IVec2> = vec![];
    for a in -half_height..(half_height + 1){
        board.push(IVec2::new(-half_width - 1, a));
        board.push(IVec2::new(half_width, a));
    }
    for b in -half_width..half_width {
        board.push(IVec2::new(b, -half_height));
    }
    board
}

fn setup_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas: Res<AtlasTextureHandle>,
) {
    for i in get_board_pos() {
        commands.spawn((
            Wall,
            Block {
                grid_pos: i
            },
            SpriteBundle {
                texture: atlas.data.clone(),
                transform: Transform::from_xyz((i.x * TILE_SIZE) as f32, (i.y * TILE_SIZE) as f32, 0.0),
                ..default()
            },
            TextureAtlas {
                layout: asset_server.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(TILE_SIZE as u32),
                    12,
                    1,
                    None,
                    None
                )),
                index: 7
            },
        ));
    }
}

fn check_board(
    query: Query<(Entity, &mut Block, &mut Transform), (Without<Active>, Without<Wall>)>,
    commands: Commands,
) {
    let grid_positions: Vec<IVec2> = query.iter()
        .map(|(_, block, _)| 
            ivec2(
                block.grid_pos.x,
                block.grid_pos.y
            )
        )
        .collect();
    let mut found_rows: Vec<i32> = vec![];
    let half_width = BOARD_SIZE.x / 2;
    let half_height = BOARD_SIZE.y / 2;
    for y in -half_height..half_height {
        let mut found = true;
        'x: for x in -half_width..half_width{
            let pos = ivec2(x, y);
            println!("{pos:?}");
            if !grid_positions.contains(&pos) {
                found = false;
                break 'x;
            }
        }
        if found {
            found_rows.push(y);
        }
    }
    println!("Found: {found_rows:?}");
    if !found_rows.is_empty() {
        clear_lines(query, found_rows, commands);
    }
}

fn clear_lines(
    mut query: Query<(Entity, &mut Block, &mut Transform), (Without<Active>, Without<Wall>)>,
    found_rows: Vec<i32>,
    mut commands: Commands,
) {
    // Clear lines
    for (entity, block, _) in &query {
        if found_rows.contains(&block.grid_pos.y) {
            commands.entity(entity).despawn();
        }
    }

    // Move down lines
    for (_, mut block, mut transform) in &mut query {
        let cleared_below = found_rows.iter().filter(|&&row| row < block.grid_pos.y).count() as i32;
        if cleared_below > 0 {
            block.grid_pos.y -= cleared_below;
            transform.translation.y -= (cleared_below * TILE_SIZE) as f32; // Adjust TILE_SIZE to match your game's tile size
        }
    }
}
