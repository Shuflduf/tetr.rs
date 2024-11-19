use bevy::prelude::*;
use rand::Rng;
use pieces::PIECES;
mod pieces;

#[derive(Component)]
struct ActivePiece;

#[derive(Component)]
struct Block {
    col_index: u8,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_piece)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let mut blocks: Vec<Entity> = vec![];
    let piece_type_index = rand::thread_rng().gen_range(0..6);
    let piece_data = PIECES[piece_type_index];
    for block_data in piece_data {
        let id = commands.spawn((
            Block {
                col_index: piece_type_index as u8
            },
            SpriteBundle {
                texture: asset_server.load("Tetr-Skin.png"),
                transform: Transform::from_xyz((block_data.x as f32) * 31.0, (block_data.y as f32) * -31.0, 0.0),
                ..default()
            },
            TextureAtlas {
                layout: asset_server.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(31),
                    12,
                    1,
                    None,
                    None
                )),
                index: piece_type_index
            },
        )).id();
        blocks.push(id);
    }
}


fn move_piece(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Block>>
) {
    let mut dir: f32 = 0.0;
    let left = keys.pressed(KeyCode::ArrowLeft); 
    let right = keys.pressed(KeyCode::ArrowRight); 
    if (left && right) || (!left && !right) { dir = 0.0 }
    else if left { dir = -1.0 }
    else if right { dir = 1.0 }
    for mut transform in &mut query {
        transform.translation.x += 31.0 * dir;
        if keys.just_pressed(KeyCode::Space) {

            transform.translation.y += -31.0;
        }
    }
}
