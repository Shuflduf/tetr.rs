use bevy::math::U16Vec2;

pub const PIECES: [[[U16Vec2; 4]; 4]; 7] = [
    // Z Piece (Red, Index 2)
    [
        [
            U16Vec2::new(0, 0),
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 1),
        ],
        [
            U16Vec2::new(2, 0),
            U16Vec2::new(2, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 2),
        ],
        [
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 2),
            U16Vec2::new(2, 2),
        ],
        [
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(0, 1),
            U16Vec2::new(0, 2),
        ],
    ],
    // L Piece (Orange, Index 1)
    [
        [
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 1),
            U16Vec2::new(2, 0),
        ],
        [
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 2),
            U16Vec2::new(2, 2),
        ],
        [
            U16Vec2::new(0, 2),
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 1),
        ],
        [
            U16Vec2::new(0, 0),
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 2),
        ],
    ],
    // O Piece (Yellow, Index 2)
    [
        [
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 0),
            U16Vec2::new(2, 1),
        ],
        [
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 0),
            U16Vec2::new(2, 1),
        ],
        [
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 0),
            U16Vec2::new(2, 1),
        ],
        [
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 0),
            U16Vec2::new(2, 1),
        ],
    ],
    // S Piece (Green, Index 3)
    [
        [
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 0),
            U16Vec2::new(2, 0),
        ],
        [
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 1),
            U16Vec2::new(2, 2),
        ],
        [
            U16Vec2::new(0, 2),
            U16Vec2::new(1, 2),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 1),
        ],
        [
            U16Vec2::new(0, 0),
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 2),
        ],
    ],
    // I Piece (Cyan, Index 4)
    [
        [
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 1),
            U16Vec2::new(3, 1),
        ],
        [
            U16Vec2::new(2, 0),
            U16Vec2::new(2, 1),
            U16Vec2::new(2, 2),
            U16Vec2::new(2, 3),
        ],
        [
            U16Vec2::new(0, 2),
            U16Vec2::new(1, 2),
            U16Vec2::new(2, 2),
            U16Vec2::new(3, 2),
        ],
        [
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 2),
            U16Vec2::new(1, 3),
        ],
    ],
    // J Piece (Blue, Index 5)
    [
        [
            U16Vec2::new(0, 0),
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 1),
        ],
        [
            U16Vec2::new(2, 0),
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 2),
        ],
        [
            U16Vec2::new(2, 2),
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 1),
            U16Vec2::new(2, 1),
        ],
        [
            U16Vec2::new(0, 2),
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 1),
            U16Vec2::new(1, 2),
        ],
    ],
    // T Piece (Pink, Index 6)
    [
        [
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 0),
            U16Vec2::new(2, 1),
            U16Vec2::new(1, 1),
        ],
        [
            U16Vec2::new(1, 2),
            U16Vec2::new(1, 0),
            U16Vec2::new(2, 1),
            U16Vec2::new(1, 1),
        ],
        [
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 2),
            U16Vec2::new(2, 1),
            U16Vec2::new(1, 1),
        ],
        [
            U16Vec2::new(0, 1),
            U16Vec2::new(1, 0),
            U16Vec2::new(1, 2),
            U16Vec2::new(1, 1),
        ],
    ]
];
