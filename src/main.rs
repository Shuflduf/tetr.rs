use bevy::prelude::*;
use bevy::reflect::List;
use rand::Rng;

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
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let mut blocks: Vec<Entity> = vec![];
    for i in 0..4 {
        let index = rand::thread_rng().gen_range(0..12);
        let id = commands.spawn((
            Block {
                col_index: index
            },
            SpriteBundle {
                texture: asset_server.load("Tetr-Skin.png"),
                transform: Transform::from_xyz((i * 31) as f32, 0.0, 0.0),
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
                index: index as usize
            },
        )).id();
        blocks.push(id);
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Block>>
) {
    for mut transform in &mut query {

        if keys.just_pressed(KeyCode::Space) {
            transform.translation.y += -31.0;
        }
    }
}
