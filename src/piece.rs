use bevy::math::U16Vec2;
use bevy::prelude::*;
use rand::Rng;

use crate::srs::PIECES;
use crate::{Block, TILE_SIZE};

#[derive(Component)]
pub struct Active {
    offset: IVec2,
    rotation: usize,
    block_index: usize,
}

pub fn spawn_piece(
    asset_server: Res<AssetServer>,
    atlas: Res<crate::AtlasTextureHandle>,
    mut commands: Commands,
) {
    let piece_type_index = rand::thread_rng().gen_range(0..=6);
    //let piece_type_index = 2;
    let piece_data = PIECES[piece_type_index][0];
    for (i, block_data) in piece_data
        .iter()
        .map(|block| block.as_ivec2() * IVec2::new(1, -1))
        .enumerate() {
        let block = commands.spawn((
            Block {
                grid_pos: block_data
            },
            Active {
                offset: IVec2::new(-2, 8),
                rotation: 0,
                block_index: i,
            },
            SpriteBundle {
                texture: atlas.data.clone(),
                transform: Transform::from_xyz((block_data.x * TILE_SIZE) as f32, (block_data.y * TILE_SIZE) as f32, 0.0),
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
        //block.
    }
}

pub fn move_piece(
    keys: Res<ButtonInput<KeyCode>>,
    sleeping_query: Query<&Block, Without<Active>>,
    systems: Res<crate::OneshotSystems>,
    mut active_query: Query<(Entity, &mut Transform, &TextureAtlas, &mut Active, &mut Block)>,
    mut commands: Commands,
) {
    const DOWN: IVec2 = IVec2::new(0, -1);
    let mut dir: f32 = 0.0;
    let left = keys.just_pressed(KeyCode::KeyA); 
    let right = keys.just_pressed(KeyCode::KeyD); 
    let mut hard_drop = false;
    if (left && right) || (!left && !right) { dir = 0.0 }
    else if left { dir = -1.0 }
    else if right { dir = 1.0 }

    // TODO: remake this so collision is only calculated when the board updates
    //let mut collision: Vec<IVec2> = vec![];
    //for transform in &sleeping_query {
    //    collision.push(IVec2::new(
    //        (transform.translation.x / 31.0).trunc() as i32,
    //        (transform.translation.y / 31.0).trunc() as i32
    //    ))
    //}
    let collision: Vec<IVec2> = sleeping_query
        .iter()
        .map(|block| block.grid_pos)
        .collect();

    let mut piece_placed = false;
    for (entity, mut transform, atlas, mut active, mut block) in &mut active_query {
        let mut temp_movement = active.offset;
        let mut temp_rotation = active.rotation;
        temp_movement.x += dir as i32;

        if keys.just_pressed(KeyCode::ArrowDown) {
            hard_drop = true
        }
        if keys.just_pressed(KeyCode::ArrowUp) {
            temp_movement.y -= 1;
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            temp_rotation += 3;
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            temp_rotation += 1;
        }
        temp_rotation %= 4;

        // The Collision Part 😱😱
        let piece_data = PIECES[atlas.index][temp_rotation];

        if hard_drop {
            while can_move(&piece_data, &collision, active.offset + DOWN) {
                active.offset.y -= 1;
                block.grid_pos.y -= 1;
                transform.translation.y -= TILE_SIZE as f32;
            }
        }
        else if can_move(&piece_data, &collision, temp_movement) {
            active.offset = temp_movement;
            block.grid_pos = temp_movement;
            active.rotation = temp_rotation;
            let block_data = piece_data[active.block_index].as_ivec2();
            transform.translation = Vec3::new(
                (active.offset.x + block_data.x) as f32,
                (active.offset.y - block_data.y) as f32,
                0.0
            ) * TILE_SIZE as f32;
        }
        if !can_move(&piece_data, &collision, active.offset + DOWN) {
            commands.entity(entity)
                .remove::<Active>();
            piece_placed = true;
        }
        
    }
    if piece_placed {
        for id in [systems.0["spawn_piece"], systems.0["check_board"]] {
            commands.run_system(id);
        }
    }
}

fn can_move(piece_data: &[U16Vec2; 4], collision: &[IVec2], dir: IVec2) -> bool {
    for i in piece_data {
        let block_data = i.as_ivec2();
        let future = dir + block_data;
        if collision.contains(&future) {
            return false
        }
    }
    true
}
