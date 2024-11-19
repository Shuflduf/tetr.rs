use bevy::prelude::*;
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("{keys:?}");
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Tetr-Skin.png"),
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
            index: rand::thread_rng().gen_range(0..12),
        },
    ));
}
