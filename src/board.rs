use crate::*;

pub fn full_lines(board: &[Block]) -> Vec<i32> {
    let mut lines = Vec::new();
    for y in 0..GRID_SIZE.y {
        let mut full = true;
        for x in 0..GRID_SIZE.x {
            if !board.iter().any(|block| block.pos == ivec2(x, y)) {
                full = false;
                break;
            }
        }
        if full {
            lines.push(y);
        }
    }
    lines
}

pub fn clear_lines(board: &mut Vec<Block>, lines: &[i32]) {
    for line in lines.iter() {
        board.retain(|block| block.pos.y != *line || block.index == 7);
        for block in board.iter_mut() {
            if block.pos.y < *line && block.index != 7 {
                block.pos.y += 1;
            }
        }
    }
}
