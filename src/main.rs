#![allow(clippy::type_complexity)]

use bevy::ecs::system::SystemId;
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
struct Block;

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

fn setup_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas: Res<AtlasTextureHandle>,
) {
    let half_width = BOARD_SIZE.x / 2;
    let half_height = BOARD_SIZE.y / 2;
    let mut board: Vec<IVec2> = vec![];
    for a in -half_height..(half_height + 1){
        board.push(IVec2::new(-half_width - 1, a));
        board.push(IVec2::new(half_width, a));
    }
    for b in -half_width..half_width {
        board.push(IVec2::new(b, half_height));
    }
    for i in board {
        commands.spawn((
            Wall,
            Block,
            SpriteBundle {
                texture: atlas.data.clone(),
                transform: Transform::from_xyz((i.x * TILE_SIZE) as f32, (i.y * -TILE_SIZE) as f32, 0.0),
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
    query: Query<&Transform, (With<Block>, Without<Active>, Without<Wall>)>,
) {
    let mut grid_positions: Vec<IVec2> = vec![];
    for transform in &query {
        grid_positions.push(
            IVec2::new(
                (transform.translation.x as i32) / TILE_SIZE,
                (transform.translation.y as i32) / -TILE_SIZE
            )
        );
    }
}
