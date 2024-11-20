use bevy::prelude::*;
use bevy::input::common_conditions::*;
use rand::Rng;
use pieces::PIECES;
mod pieces;

#[derive(Component)]
struct Active {
    offset: IVec2,
    rotation: u8,
    block_index: u8,
}

#[derive(Resource)]
struct AtlasTextureHandle {
    data: Handle<Image>
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_piece,
            spawn_piece
                .run_if(input_just_pressed(KeyCode::Tab)),
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    //mut atlas: ResMut<AtlasTextureHandle>,
    //asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    //atlas = asset_server.load("Tetr-Skin.png");
}

fn spawn_piece(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let piece_type_index = rand::thread_rng().gen_range(0..6);
    let piece_data = PIECES[piece_type_index][0];
    for (i, block_data) in piece_data.iter().enumerate() {
        commands.spawn((
            Active {
                offset: IVec2::ZERO,
                rotation: 0,
                block_index: i as u8,
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
        ));
    }
}


fn move_piece(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut TextureAtlas, &mut Active)>,
) {
    let mut dir: f32 = 0.0;
    let left = keys.pressed(KeyCode::KeyA); 
    let right = keys.pressed(KeyCode::KeyD); 
    if (left && right) || (!left && !right) { dir = 0.0 }
    else if left { dir = -1.0 }
    else if right { dir = 1.0 }
    for (mut transform, mut atlas, mut active) in &mut query {
        active.offset.x += dir as i32;

        if keys.just_pressed(KeyCode::ArrowDown) {
            atlas.index = (atlas.index + 1) % 7;
            //active.offset.y += -1;
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            active.rotation += 3;
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            active.rotation += 1;
        }
        active.rotation %= 4;
        println!("{0}", active.rotation);
        let piece_data = PIECES[atlas.index][active.rotation as usize][active.block_index as usize].as_ivec2();
        transform.translation = Vec3::new(
            (active.offset.x + piece_data.x) as f32,
            (active.offset.y + -piece_data.y) as f32,
            0.0
        ) * 31.0;
    }
}
