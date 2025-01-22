use macroquad::math::IVec2;

//static A: i32 = 234;
struct Piece {
    pub index: u8,
    pub rotation: u8,
    pub pos: IVec2,
}

static ACTIVE_PIECE: Piece = Piece {
    index: 0,
    rotation: 0,
    pos: IVec2::ZERO,
};

const NULL_PIECE: Piece = Piece {
    index: 0,
    rotation: 0,
    pos: IVec2::ZERO,
};

