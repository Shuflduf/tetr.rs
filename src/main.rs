use bevy::math::I16Vec2;
use bevy::prelude::*;
use bevy::input::common_conditions::*;
use rand::Rng;
use pieces::PIECES;
mod pieces;

#[derive(Component)]
struct Active {
    offset: IVec2,
    rotation: usize,
    block_index: usize,
}

#[derive(Component)]
struct Block;

#[derive(Resource)]
struct AtlasTextureHandle {
    data: Handle<Image>
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_board).chain())
        .add_systems(Update, (
            move_piece,
            spawn_piece
                .run_if(input_just_pressed(KeyCode::Tab)),
        ))
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
    const WIDTH: i16 = 10;
    const HEIGHT: i16 = 20;
    let half_width = WIDTH / 2;
    let half_height = HEIGHT / 2;
    let mut board: Vec<I16Vec2> = vec![];
    for a in -half_height..(half_height + 1){
        board.push(I16Vec2::new(-half_width - 1, a));
        board.push(I16Vec2::new(half_width, a));
    }
    for b in -half_width..half_width {
        board.push(I16Vec2::new(b, half_height));
    }
    for i in board {
        commands.spawn((
            Block,
            SpriteBundle {
                texture: atlas.data.clone(),
                transform: Transform::from_xyz((i.x as f32) * 31.0, (i.y as f32) * -31.0, 0.0),
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
                index: 7
            },
        ));
    }
}

fn spawn_piece(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas: Res<AtlasTextureHandle>,
) {
    let piece_type_index = rand::thread_rng().gen_range(0..=6);
    let piece_data = PIECES[piece_type_index][0];
    for (i, block_data) in piece_data.iter().enumerate() {
        commands.spawn((
            Block,
            Active {
                offset: IVec2::ZERO,
                rotation: 0,
                block_index: i,
            },
            SpriteBundle {
                texture: atlas.data.clone(),
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
    sleeping_query: Query<&Transform, (Without<Active>, With<Block>)>,
    mut active_query: Query<(&mut Transform, &TextureAtlas, &mut Active), With<Block>>,
) {
    let mut dir: f32 = 0.0;
    let left = keys.just_pressed(KeyCode::KeyA); 
    let right = keys.just_pressed(KeyCode::KeyD); 
    if (left && right) || (!left && !right) { dir = 0.0 }
    else if left { dir = -1.0 }
    else if right { dir = 1.0 }

    // TODO: remake this so collision is only calculated when the board updates
    let mut collision: Vec<IVec2> = vec![];
    for transform in &sleeping_query {
        collision.push(IVec2::new(
            (transform.translation.x / 31.0).trunc() as i32,
            (transform.translation.y / 31.0).trunc() as i32
        ))
    }

    for (mut transform, atlas, mut active) in &mut active_query {
        let mut temp_movement = active.offset;
        let mut temp_rotation = active.rotation;
        temp_movement.x += dir as i32;
        //active.offset.x += dir as i32;

        if keys.just_pressed(KeyCode::ArrowDown) {
            //atlas.index = (atlas.index + 1) % 7;
            temp_movement.y += -1;
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            //active.rotation += 3;
            temp_rotation += 3;
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            //active.rotation += 1;
            temp_rotation += 1;
        }
        //active.rotation %= 4;
        temp_rotation %= 4;

        // The Collision Part 😱😱
        let piece_data = PIECES[atlas.index][temp_rotation];
        let mut can_move = true;
        for (i, _) in piece_data.iter().enumerate() {
            let block_data = piece_data[i].as_ivec2();
            let future = temp_movement + (block_data * IVec2::new(1, -1));
            if collision.contains(&future) {
                can_move = false;
                break;
            }
        }
        //active.offset = {
        //};

        if can_move {
            active.offset = temp_movement;
            active.rotation = temp_rotation;
            let block_data = piece_data[active.block_index].as_ivec2();
            transform.translation = Vec3::new(
                (active.offset.x + block_data.x) as f32,
                (active.offset.y - block_data.y) as f32,
                0.0
            ) * 31.0;
        }
    }
}
