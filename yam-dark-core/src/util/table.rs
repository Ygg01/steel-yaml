pub const U8_BYTE_COL_TABLE: [[u8; 8]; 256] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [0, 0, 1, 2, 3, 4, 5, 6],
    [0, 1, 0, 1, 2, 3, 4, 5],
    [0, 0, 0, 1, 2, 3, 4, 5],
    [0, 1, 2, 0, 1, 2, 3, 4],
    [0, 0, 1, 0, 1, 2, 3, 4],
    [0, 1, 0, 0, 1, 2, 3, 4],
    [0, 0, 0, 0, 1, 2, 3, 4],
    [0, 1, 2, 3, 0, 1, 2, 3],
    [0, 0, 1, 2, 0, 1, 2, 3],
    [0, 1, 0, 1, 0, 1, 2, 3],
    [0, 0, 0, 1, 0, 1, 2, 3],
    [0, 1, 2, 0, 0, 1, 2, 3],
    [0, 0, 1, 0, 0, 1, 2, 3],
    [0, 1, 0, 0, 0, 1, 2, 3],
    [0, 0, 0, 0, 0, 1, 2, 3],
    [0, 1, 2, 3, 4, 0, 1, 2],
    [0, 0, 1, 2, 3, 0, 1, 2],
    [0, 1, 0, 1, 2, 0, 1, 2],
    [0, 0, 0, 1, 2, 0, 1, 2],
    [0, 1, 2, 0, 1, 0, 1, 2],
    [0, 0, 1, 0, 1, 0, 1, 2],
    [0, 1, 0, 0, 1, 0, 1, 2],
    [0, 0, 0, 0, 1, 0, 1, 2],
    [0, 1, 2, 3, 0, 0, 1, 2],
    [0, 0, 1, 2, 0, 0, 1, 2],
    [0, 1, 0, 1, 0, 0, 1, 2],
    [0, 0, 0, 1, 0, 0, 1, 2],
    [0, 1, 2, 0, 0, 0, 1, 2],
    [0, 0, 1, 0, 0, 0, 1, 2],
    [0, 1, 0, 0, 0, 0, 1, 2],
    [0, 0, 0, 0, 0, 0, 1, 2],
    [0, 1, 2, 3, 4, 5, 0, 1],
    [0, 0, 1, 2, 3, 4, 0, 1],
    [0, 1, 0, 1, 2, 3, 0, 1],
    [0, 0, 0, 1, 2, 3, 0, 1],
    [0, 1, 2, 0, 1, 2, 0, 1],
    [0, 0, 1, 0, 1, 2, 0, 1],
    [0, 1, 0, 0, 1, 2, 0, 1],
    [0, 0, 0, 0, 1, 2, 0, 1],
    [0, 1, 2, 3, 0, 1, 0, 1],
    [0, 0, 1, 2, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [0, 0, 0, 1, 0, 1, 0, 1],
    [0, 1, 2, 0, 0, 1, 0, 1],
    [0, 0, 1, 0, 0, 1, 0, 1],
    [0, 1, 0, 0, 0, 1, 0, 1],
    [0, 0, 0, 0, 0, 1, 0, 1],
    [0, 1, 2, 3, 4, 0, 0, 1],
    [0, 0, 1, 2, 3, 0, 0, 1],
    [0, 1, 0, 1, 2, 0, 0, 1],
    [0, 0, 0, 1, 2, 0, 0, 1],
    [0, 1, 2, 0, 1, 0, 0, 1],
    [0, 0, 1, 0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1, 0, 0, 1],
    [0, 0, 0, 0, 1, 0, 0, 1],
    [0, 1, 2, 3, 0, 0, 0, 1],
    [0, 0, 1, 2, 0, 0, 0, 1],
    [0, 1, 0, 1, 0, 0, 0, 1],
    [0, 0, 0, 1, 0, 0, 0, 1],
    [0, 1, 2, 0, 0, 0, 0, 1],
    [0, 0, 1, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 1],
    [0, 1, 2, 3, 4, 5, 6, 0],
    [0, 0, 1, 2, 3, 4, 5, 0],
    [0, 1, 0, 1, 2, 3, 4, 0],
    [0, 0, 0, 1, 2, 3, 4, 0],
    [0, 1, 2, 0, 1, 2, 3, 0],
    [0, 0, 1, 0, 1, 2, 3, 0],
    [0, 1, 0, 0, 1, 2, 3, 0],
    [0, 0, 0, 0, 1, 2, 3, 0],
    [0, 1, 2, 3, 0, 1, 2, 0],
    [0, 0, 1, 2, 0, 1, 2, 0],
    [0, 1, 0, 1, 0, 1, 2, 0],
    [0, 0, 0, 1, 0, 1, 2, 0],
    [0, 1, 2, 0, 0, 1, 2, 0],
    [0, 0, 1, 0, 0, 1, 2, 0],
    [0, 1, 0, 0, 0, 1, 2, 0],
    [0, 0, 0, 0, 0, 1, 2, 0],
    [0, 1, 2, 3, 4, 0, 1, 0],
    [0, 0, 1, 2, 3, 0, 1, 0],
    [0, 1, 0, 1, 2, 0, 1, 0],
    [0, 0, 0, 1, 2, 0, 1, 0],
    [0, 1, 2, 0, 1, 0, 1, 0],
    [0, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 0, 1, 0, 1, 0],
    [0, 0, 0, 0, 1, 0, 1, 0],
    [0, 1, 2, 3, 0, 0, 1, 0],
    [0, 0, 1, 2, 0, 0, 1, 0],
    [0, 1, 0, 1, 0, 0, 1, 0],
    [0, 0, 0, 1, 0, 0, 1, 0],
    [0, 1, 2, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 2, 3, 4, 5, 0, 0],
    [0, 0, 1, 2, 3, 4, 0, 0],
    [0, 1, 0, 1, 2, 3, 0, 0],
    [0, 0, 0, 1, 2, 3, 0, 0],
    [0, 1, 2, 0, 1, 2, 0, 0],
    [0, 0, 1, 0, 1, 2, 0, 0],
    [0, 1, 0, 0, 1, 2, 0, 0],
    [0, 0, 0, 0, 1, 2, 0, 0],
    [0, 1, 2, 3, 0, 1, 0, 0],
    [0, 0, 1, 2, 0, 1, 0, 0],
    [0, 1, 0, 1, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 1, 0, 0],
    [0, 1, 2, 0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0],
    [0, 1, 2, 3, 4, 0, 0, 0],
    [0, 0, 1, 2, 3, 0, 0, 0],
    [0, 1, 0, 1, 2, 0, 0, 0],
    [0, 0, 0, 1, 2, 0, 0, 0],
    [0, 1, 2, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0],
    [0, 1, 2, 3, 0, 0, 0, 0],
    [0, 0, 1, 2, 0, 0, 0, 0],
    [0, 1, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0],
    [0, 1, 2, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 2, 3, 4, 5, 6, 7],
    [0, 0, 1, 2, 3, 4, 5, 6],
    [0, 1, 0, 1, 2, 3, 4, 5],
    [0, 0, 0, 1, 2, 3, 4, 5],
    [0, 1, 2, 0, 1, 2, 3, 4],
    [0, 0, 1, 0, 1, 2, 3, 4],
    [0, 1, 0, 0, 1, 2, 3, 4],
    [0, 0, 0, 0, 1, 2, 3, 4],
    [0, 1, 2, 3, 0, 1, 2, 3],
    [0, 0, 1, 2, 0, 1, 2, 3],
    [0, 1, 0, 1, 0, 1, 2, 3],
    [0, 0, 0, 1, 0, 1, 2, 3],
    [0, 1, 2, 0, 0, 1, 2, 3],
    [0, 0, 1, 0, 0, 1, 2, 3],
    [0, 1, 0, 0, 0, 1, 2, 3],
    [0, 0, 0, 0, 0, 1, 2, 3],
    [0, 1, 2, 3, 4, 0, 1, 2],
    [0, 0, 1, 2, 3, 0, 1, 2],
    [0, 1, 0, 1, 2, 0, 1, 2],
    [0, 0, 0, 1, 2, 0, 1, 2],
    [0, 1, 2, 0, 1, 0, 1, 2],
    [0, 0, 1, 0, 1, 0, 1, 2],
    [0, 1, 0, 0, 1, 0, 1, 2],
    [0, 0, 0, 0, 1, 0, 1, 2],
    [0, 1, 2, 3, 0, 0, 1, 2],
    [0, 0, 1, 2, 0, 0, 1, 2],
    [0, 1, 0, 1, 0, 0, 1, 2],
    [0, 0, 0, 1, 0, 0, 1, 2],
    [0, 1, 2, 0, 0, 0, 1, 2],
    [0, 0, 1, 0, 0, 0, 1, 2],
    [0, 1, 0, 0, 0, 0, 1, 2],
    [0, 0, 0, 0, 0, 0, 1, 2],
    [0, 1, 2, 3, 4, 5, 0, 1],
    [0, 0, 1, 2, 3, 4, 0, 1],
    [0, 1, 0, 1, 2, 3, 0, 1],
    [0, 0, 0, 1, 2, 3, 0, 1],
    [0, 1, 2, 0, 1, 2, 0, 1],
    [0, 0, 1, 0, 1, 2, 0, 1],
    [0, 1, 0, 0, 1, 2, 0, 1],
    [0, 0, 0, 0, 1, 2, 0, 1],
    [0, 1, 2, 3, 0, 1, 0, 1],
    [0, 0, 1, 2, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [0, 0, 0, 1, 0, 1, 0, 1],
    [0, 1, 2, 0, 0, 1, 0, 1],
    [0, 0, 1, 0, 0, 1, 0, 1],
    [0, 1, 0, 0, 0, 1, 0, 1],
    [0, 0, 0, 0, 0, 1, 0, 1],
    [0, 1, 2, 3, 4, 0, 0, 1],
    [0, 0, 1, 2, 3, 0, 0, 1],
    [0, 1, 0, 1, 2, 0, 0, 1],
    [0, 0, 0, 1, 2, 0, 0, 1],
    [0, 1, 2, 0, 1, 0, 0, 1],
    [0, 0, 1, 0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1, 0, 0, 1],
    [0, 0, 0, 0, 1, 0, 0, 1],
    [0, 1, 2, 3, 0, 0, 0, 1],
    [0, 0, 1, 2, 0, 0, 0, 1],
    [0, 1, 0, 1, 0, 0, 0, 1],
    [0, 0, 0, 1, 0, 0, 0, 1],
    [0, 1, 2, 0, 0, 0, 0, 1],
    [0, 0, 1, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 1],
    [0, 1, 2, 3, 4, 5, 6, 0],
    [0, 0, 1, 2, 3, 4, 5, 0],
    [0, 1, 0, 1, 2, 3, 4, 0],
    [0, 0, 0, 1, 2, 3, 4, 0],
    [0, 1, 2, 0, 1, 2, 3, 0],
    [0, 0, 1, 0, 1, 2, 3, 0],
    [0, 1, 0, 0, 1, 2, 3, 0],
    [0, 0, 0, 0, 1, 2, 3, 0],
    [0, 1, 2, 3, 0, 1, 2, 0],
    [0, 0, 1, 2, 0, 1, 2, 0],
    [0, 1, 0, 1, 0, 1, 2, 0],
    [0, 0, 0, 1, 0, 1, 2, 0],
    [0, 1, 2, 0, 0, 1, 2, 0],
    [0, 0, 1, 0, 0, 1, 2, 0],
    [0, 1, 0, 0, 0, 1, 2, 0],
    [0, 0, 0, 0, 0, 1, 2, 0],
    [0, 1, 2, 3, 4, 0, 1, 0],
    [0, 0, 1, 2, 3, 0, 1, 0],
    [0, 1, 0, 1, 2, 0, 1, 0],
    [0, 0, 0, 1, 2, 0, 1, 0],
    [0, 1, 2, 0, 1, 0, 1, 0],
    [0, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 0, 1, 0, 1, 0],
    [0, 0, 0, 0, 1, 0, 1, 0],
    [0, 1, 2, 3, 0, 0, 1, 0],
    [0, 0, 1, 2, 0, 0, 1, 0],
    [0, 1, 0, 1, 0, 0, 1, 0],
    [0, 0, 0, 1, 0, 0, 1, 0],
    [0, 1, 2, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 2, 3, 4, 5, 0, 0],
    [0, 0, 1, 2, 3, 4, 0, 0],
    [0, 1, 0, 1, 2, 3, 0, 0],
    [0, 0, 0, 1, 2, 3, 0, 0],
    [0, 1, 2, 0, 1, 2, 0, 0],
    [0, 0, 1, 0, 1, 2, 0, 0],
    [0, 1, 0, 0, 1, 2, 0, 0],
    [0, 0, 0, 0, 1, 2, 0, 0],
    [0, 1, 2, 3, 0, 1, 0, 0],
    [0, 0, 1, 2, 0, 1, 0, 0],
    [0, 1, 0, 1, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 1, 0, 0],
    [0, 1, 2, 0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0],
    [0, 1, 2, 3, 4, 0, 0, 0],
    [0, 0, 1, 2, 3, 0, 0, 0],
    [0, 1, 0, 1, 2, 0, 0, 0],
    [0, 0, 0, 1, 2, 0, 0, 0],
    [0, 1, 2, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0],
    [0, 1, 2, 3, 0, 0, 0, 0],
    [0, 0, 1, 2, 0, 0, 0, 0],
    [0, 1, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0],
    [0, 1, 2, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

pub const U8_ROW_TABLE: [[u8; 8]; 256] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 1],
    [1, 2, 2, 2, 2, 2, 2, 2],
    [0, 0, 1, 1, 1, 1, 1, 1],
    [1, 1, 2, 2, 2, 2, 2, 2],
    [0, 1, 2, 2, 2, 2, 2, 2],
    [1, 2, 3, 3, 3, 3, 3, 3],
    [0, 0, 0, 1, 1, 1, 1, 1],
    [1, 1, 1, 2, 2, 2, 2, 2],
    [0, 1, 1, 2, 2, 2, 2, 2],
    [1, 2, 2, 3, 3, 3, 3, 3],
    [0, 0, 1, 2, 2, 2, 2, 2],
    [1, 1, 2, 3, 3, 3, 3, 3],
    [0, 1, 2, 3, 3, 3, 3, 3],
    [1, 2, 3, 4, 4, 4, 4, 4],
    [0, 0, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 2, 2, 2, 2],
    [0, 1, 1, 1, 2, 2, 2, 2],
    [1, 2, 2, 2, 3, 3, 3, 3],
    [0, 0, 1, 1, 2, 2, 2, 2],
    [1, 1, 2, 2, 3, 3, 3, 3],
    [0, 1, 2, 2, 3, 3, 3, 3],
    [1, 2, 3, 3, 4, 4, 4, 4],
    [0, 0, 0, 1, 2, 2, 2, 2],
    [1, 1, 1, 2, 3, 3, 3, 3],
    [0, 1, 1, 2, 3, 3, 3, 3],
    [1, 2, 2, 3, 4, 4, 4, 4],
    [0, 0, 1, 2, 3, 3, 3, 3],
    [1, 1, 2, 3, 4, 4, 4, 4],
    [0, 1, 2, 3, 4, 4, 4, 4],
    [1, 2, 3, 4, 5, 5, 5, 5],
    [0, 0, 0, 0, 0, 1, 1, 1],
    [1, 1, 1, 1, 1, 2, 2, 2],
    [0, 1, 1, 1, 1, 2, 2, 2],
    [1, 2, 2, 2, 2, 3, 3, 3],
    [0, 0, 1, 1, 1, 2, 2, 2],
    [1, 1, 2, 2, 2, 3, 3, 3],
    [0, 1, 2, 2, 2, 3, 3, 3],
    [1, 2, 3, 3, 3, 4, 4, 4],
    [0, 0, 0, 1, 1, 2, 2, 2],
    [1, 1, 1, 2, 2, 3, 3, 3],
    [0, 1, 1, 2, 2, 3, 3, 3],
    [1, 2, 2, 3, 3, 4, 4, 4],
    [0, 0, 1, 2, 2, 3, 3, 3],
    [1, 1, 2, 3, 3, 4, 4, 4],
    [0, 1, 2, 3, 3, 4, 4, 4],
    [1, 2, 3, 4, 4, 5, 5, 5],
    [0, 0, 0, 0, 1, 2, 2, 2],
    [1, 1, 1, 1, 2, 3, 3, 3],
    [0, 1, 1, 1, 2, 3, 3, 3],
    [1, 2, 2, 2, 3, 4, 4, 4],
    [0, 0, 1, 1, 2, 3, 3, 3],
    [1, 1, 2, 2, 3, 4, 4, 4],
    [0, 1, 2, 2, 3, 4, 4, 4],
    [1, 2, 3, 3, 4, 5, 5, 5],
    [0, 0, 0, 1, 2, 3, 3, 3],
    [1, 1, 1, 2, 3, 4, 4, 4],
    [0, 1, 1, 2, 3, 4, 4, 4],
    [1, 2, 2, 3, 4, 5, 5, 5],
    [0, 0, 1, 2, 3, 4, 4, 4],
    [1, 1, 2, 3, 4, 5, 5, 5],
    [0, 1, 2, 3, 4, 5, 5, 5],
    [1, 2, 3, 4, 5, 6, 6, 6],
    [0, 0, 0, 0, 0, 0, 1, 1],
    [1, 1, 1, 1, 1, 1, 2, 2],
    [0, 1, 1, 1, 1, 1, 2, 2],
    [1, 2, 2, 2, 2, 2, 3, 3],
    [0, 0, 1, 1, 1, 1, 2, 2],
    [1, 1, 2, 2, 2, 2, 3, 3],
    [0, 1, 2, 2, 2, 2, 3, 3],
    [1, 2, 3, 3, 3, 3, 4, 4],
    [0, 0, 0, 1, 1, 1, 2, 2],
    [1, 1, 1, 2, 2, 2, 3, 3],
    [0, 1, 1, 2, 2, 2, 3, 3],
    [1, 2, 2, 3, 3, 3, 4, 4],
    [0, 0, 1, 2, 2, 2, 3, 3],
    [1, 1, 2, 3, 3, 3, 4, 4],
    [0, 1, 2, 3, 3, 3, 4, 4],
    [1, 2, 3, 4, 4, 4, 5, 5],
    [0, 0, 0, 0, 1, 1, 2, 2],
    [1, 1, 1, 1, 2, 2, 3, 3],
    [0, 1, 1, 1, 2, 2, 3, 3],
    [1, 2, 2, 2, 3, 3, 4, 4],
    [0, 0, 1, 1, 2, 2, 3, 3],
    [1, 1, 2, 2, 3, 3, 4, 4],
    [0, 1, 2, 2, 3, 3, 4, 4],
    [1, 2, 3, 3, 4, 4, 5, 5],
    [0, 0, 0, 1, 2, 2, 3, 3],
    [1, 1, 1, 2, 3, 3, 4, 4],
    [0, 1, 1, 2, 3, 3, 4, 4],
    [1, 2, 2, 3, 4, 4, 5, 5],
    [0, 0, 1, 2, 3, 3, 4, 4],
    [1, 1, 2, 3, 4, 4, 5, 5],
    [0, 1, 2, 3, 4, 4, 5, 5],
    [1, 2, 3, 4, 5, 5, 6, 6],
    [0, 0, 0, 0, 0, 1, 2, 2],
    [1, 1, 1, 1, 1, 2, 3, 3],
    [0, 1, 1, 1, 1, 2, 3, 3],
    [1, 2, 2, 2, 2, 3, 4, 4],
    [0, 0, 1, 1, 1, 2, 3, 3],
    [1, 1, 2, 2, 2, 3, 4, 4],
    [0, 1, 2, 2, 2, 3, 4, 4],
    [1, 2, 3, 3, 3, 4, 5, 5],
    [0, 0, 0, 1, 1, 2, 3, 3],
    [1, 1, 1, 2, 2, 3, 4, 4],
    [0, 1, 1, 2, 2, 3, 4, 4],
    [1, 2, 2, 3, 3, 4, 5, 5],
    [0, 0, 1, 2, 2, 3, 4, 4],
    [1, 1, 2, 3, 3, 4, 5, 5],
    [0, 1, 2, 3, 3, 4, 5, 5],
    [1, 2, 3, 4, 4, 5, 6, 6],
    [0, 0, 0, 0, 1, 2, 3, 3],
    [1, 1, 1, 1, 2, 3, 4, 4],
    [0, 1, 1, 1, 2, 3, 4, 4],
    [1, 2, 2, 2, 3, 4, 5, 5],
    [0, 0, 1, 1, 2, 3, 4, 4],
    [1, 1, 2, 2, 3, 4, 5, 5],
    [0, 1, 2, 2, 3, 4, 5, 5],
    [1, 2, 3, 3, 4, 5, 6, 6],
    [0, 0, 0, 1, 2, 3, 4, 4],
    [1, 1, 1, 2, 3, 4, 5, 5],
    [0, 1, 1, 2, 3, 4, 5, 5],
    [1, 2, 2, 3, 4, 5, 6, 6],
    [0, 0, 1, 2, 3, 4, 5, 5],
    [1, 1, 2, 3, 4, 5, 6, 6],
    [0, 1, 2, 3, 4, 5, 6, 6],
    [1, 2, 3, 4, 5, 6, 7, 7],
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 2],
    [0, 1, 1, 1, 1, 1, 1, 2],
    [1, 2, 2, 2, 2, 2, 2, 3],
    [0, 0, 1, 1, 1, 1, 1, 2],
    [1, 1, 2, 2, 2, 2, 2, 3],
    [0, 1, 2, 2, 2, 2, 2, 3],
    [1, 2, 3, 3, 3, 3, 3, 4],
    [0, 0, 0, 1, 1, 1, 1, 2],
    [1, 1, 1, 2, 2, 2, 2, 3],
    [0, 1, 1, 2, 2, 2, 2, 3],
    [1, 2, 2, 3, 3, 3, 3, 4],
    [0, 0, 1, 2, 2, 2, 2, 3],
    [1, 1, 2, 3, 3, 3, 3, 4],
    [0, 1, 2, 3, 3, 3, 3, 4],
    [1, 2, 3, 4, 4, 4, 4, 5],
    [0, 0, 0, 0, 1, 1, 1, 2],
    [1, 1, 1, 1, 2, 2, 2, 3],
    [0, 1, 1, 1, 2, 2, 2, 3],
    [1, 2, 2, 2, 3, 3, 3, 4],
    [0, 0, 1, 1, 2, 2, 2, 3],
    [1, 1, 2, 2, 3, 3, 3, 4],
    [0, 1, 2, 2, 3, 3, 3, 4],
    [1, 2, 3, 3, 4, 4, 4, 5],
    [0, 0, 0, 1, 2, 2, 2, 3],
    [1, 1, 1, 2, 3, 3, 3, 4],
    [0, 1, 1, 2, 3, 3, 3, 4],
    [1, 2, 2, 3, 4, 4, 4, 5],
    [0, 0, 1, 2, 3, 3, 3, 4],
    [1, 1, 2, 3, 4, 4, 4, 5],
    [0, 1, 2, 3, 4, 4, 4, 5],
    [1, 2, 3, 4, 5, 5, 5, 6],
    [0, 0, 0, 0, 0, 1, 1, 2],
    [1, 1, 1, 1, 1, 2, 2, 3],
    [0, 1, 1, 1, 1, 2, 2, 3],
    [1, 2, 2, 2, 2, 3, 3, 4],
    [0, 0, 1, 1, 1, 2, 2, 3],
    [1, 1, 2, 2, 2, 3, 3, 4],
    [0, 1, 2, 2, 2, 3, 3, 4],
    [1, 2, 3, 3, 3, 4, 4, 5],
    [0, 0, 0, 1, 1, 2, 2, 3],
    [1, 1, 1, 2, 2, 3, 3, 4],
    [0, 1, 1, 2, 2, 3, 3, 4],
    [1, 2, 2, 3, 3, 4, 4, 5],
    [0, 0, 1, 2, 2, 3, 3, 4],
    [1, 1, 2, 3, 3, 4, 4, 5],
    [0, 1, 2, 3, 3, 4, 4, 5],
    [1, 2, 3, 4, 4, 5, 5, 6],
    [0, 0, 0, 0, 1, 2, 2, 3],
    [1, 1, 1, 1, 2, 3, 3, 4],
    [0, 1, 1, 1, 2, 3, 3, 4],
    [1, 2, 2, 2, 3, 4, 4, 5],
    [0, 0, 1, 1, 2, 3, 3, 4],
    [1, 1, 2, 2, 3, 4, 4, 5],
    [0, 1, 2, 2, 3, 4, 4, 5],
    [1, 2, 3, 3, 4, 5, 5, 6],
    [0, 0, 0, 1, 2, 3, 3, 4],
    [1, 1, 1, 2, 3, 4, 4, 5],
    [0, 1, 1, 2, 3, 4, 4, 5],
    [1, 2, 2, 3, 4, 5, 5, 6],
    [0, 0, 1, 2, 3, 4, 4, 5],
    [1, 1, 2, 3, 4, 5, 5, 6],
    [0, 1, 2, 3, 4, 5, 5, 6],
    [1, 2, 3, 4, 5, 6, 6, 7],
    [0, 0, 0, 0, 0, 0, 1, 2],
    [1, 1, 1, 1, 1, 1, 2, 3],
    [0, 1, 1, 1, 1, 1, 2, 3],
    [1, 2, 2, 2, 2, 2, 3, 4],
    [0, 0, 1, 1, 1, 1, 2, 3],
    [1, 1, 2, 2, 2, 2, 3, 4],
    [0, 1, 2, 2, 2, 2, 3, 4],
    [1, 2, 3, 3, 3, 3, 4, 5],
    [0, 0, 0, 1, 1, 1, 2, 3],
    [1, 1, 1, 2, 2, 2, 3, 4],
    [0, 1, 1, 2, 2, 2, 3, 4],
    [1, 2, 2, 3, 3, 3, 4, 5],
    [0, 0, 1, 2, 2, 2, 3, 4],
    [1, 1, 2, 3, 3, 3, 4, 5],
    [0, 1, 2, 3, 3, 3, 4, 5],
    [1, 2, 3, 4, 4, 4, 5, 6],
    [0, 0, 0, 0, 1, 1, 2, 3],
    [1, 1, 1, 1, 2, 2, 3, 4],
    [0, 1, 1, 1, 2, 2, 3, 4],
    [1, 2, 2, 2, 3, 3, 4, 5],
    [0, 0, 1, 1, 2, 2, 3, 4],
    [1, 1, 2, 2, 3, 3, 4, 5],
    [0, 1, 2, 2, 3, 3, 4, 5],
    [1, 2, 3, 3, 4, 4, 5, 6],
    [0, 0, 0, 1, 2, 2, 3, 4],
    [1, 1, 1, 2, 3, 3, 4, 5],
    [0, 1, 1, 2, 3, 3, 4, 5],
    [1, 2, 2, 3, 4, 4, 5, 6],
    [0, 0, 1, 2, 3, 3, 4, 5],
    [1, 1, 2, 3, 4, 4, 5, 6],
    [0, 1, 2, 3, 4, 4, 5, 6],
    [1, 2, 3, 4, 5, 5, 6, 7],
    [0, 0, 0, 0, 0, 1, 2, 3],
    [1, 1, 1, 1, 1, 2, 3, 4],
    [0, 1, 1, 1, 1, 2, 3, 4],
    [1, 2, 2, 2, 2, 3, 4, 5],
    [0, 0, 1, 1, 1, 2, 3, 4],
    [1, 1, 2, 2, 2, 3, 4, 5],
    [0, 1, 2, 2, 2, 3, 4, 5],
    [1, 2, 3, 3, 3, 4, 5, 6],
    [0, 0, 0, 1, 1, 2, 3, 4],
    [1, 1, 1, 2, 2, 3, 4, 5],
    [0, 1, 1, 2, 2, 3, 4, 5],
    [1, 2, 2, 3, 3, 4, 5, 6],
    [0, 0, 1, 2, 2, 3, 4, 5],
    [1, 1, 2, 3, 3, 4, 5, 6],
    [0, 1, 2, 3, 3, 4, 5, 6],
    [1, 2, 3, 4, 4, 5, 6, 7],
    [0, 0, 0, 0, 1, 2, 3, 4],
    [1, 1, 1, 1, 2, 3, 4, 5],
    [0, 1, 1, 1, 2, 3, 4, 5],
    [1, 2, 2, 2, 3, 4, 5, 6],
    [0, 0, 1, 1, 2, 3, 4, 5],
    [1, 1, 2, 2, 3, 4, 5, 6],
    [0, 1, 2, 2, 3, 4, 5, 6],
    [1, 2, 3, 3, 4, 5, 6, 7],
    [0, 0, 0, 1, 2, 3, 4, 5],
    [1, 1, 1, 2, 3, 4, 5, 6],
    [0, 1, 1, 2, 3, 4, 5, 6],
    [1, 2, 2, 3, 4, 5, 6, 7],
    [0, 0, 1, 2, 3, 4, 5, 6],
    [1, 1, 2, 3, 4, 5, 6, 7],
    [0, 1, 2, 3, 4, 5, 6, 7],
    [1, 2, 3, 4, 5, 6, 7, 8],
];

pub const INDENT_SWIZZLE_TABLE: [[u8; 8]; 256] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0],
    [2, 2, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 0, 0],
    [0, 2, 2, 0, 0, 0, 0, 0],
    [3, 3, 3, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0],
    [1, 0, 0, 1, 0, 0, 0, 0],
    [0, 1, 0, 1, 0, 0, 0, 0],
    [2, 2, 0, 1, 0, 0, 0, 0],
    [0, 0, 2, 2, 0, 0, 0, 0],
    [1, 0, 2, 2, 0, 0, 0, 0],
    [0, 3, 3, 3, 0, 0, 0, 0],
    [4, 4, 4, 4, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0],
    [1, 0, 0, 0, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [2, 2, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 1, 0, 0, 0],
    [1, 0, 1, 0, 1, 0, 0, 0],
    [0, 2, 2, 0, 1, 0, 0, 0],
    [3, 3, 3, 0, 1, 0, 0, 0],
    [0, 0, 0, 2, 2, 0, 0, 0],
    [1, 0, 0, 2, 2, 0, 0, 0],
    [0, 1, 0, 2, 2, 0, 0, 0],
    [2, 2, 0, 2, 2, 0, 0, 0],
    [0, 0, 3, 3, 3, 0, 0, 0],
    [1, 0, 3, 3, 3, 0, 0, 0],
    [0, 4, 4, 4, 4, 0, 0, 0],
    [5, 5, 5, 5, 5, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0],
    [1, 0, 0, 0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0, 1, 0, 0],
    [2, 2, 0, 0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0],
    [1, 0, 1, 0, 0, 1, 0, 0],
    [0, 2, 2, 0, 0, 1, 0, 0],
    [3, 3, 3, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 1, 0, 0],
    [1, 0, 0, 1, 0, 1, 0, 0],
    [0, 1, 0, 1, 0, 1, 0, 0],
    [2, 2, 0, 1, 0, 1, 0, 0],
    [0, 0, 2, 2, 0, 1, 0, 0],
    [1, 0, 2, 2, 0, 1, 0, 0],
    [0, 3, 3, 3, 0, 1, 0, 0],
    [4, 4, 4, 4, 0, 1, 0, 0],
    [0, 0, 0, 0, 2, 2, 0, 0],
    [1, 0, 0, 0, 2, 2, 0, 0],
    [0, 1, 0, 0, 2, 2, 0, 0],
    [2, 2, 0, 0, 2, 2, 0, 0],
    [0, 0, 1, 0, 2, 2, 0, 0],
    [1, 0, 1, 0, 2, 2, 0, 0],
    [0, 2, 2, 0, 2, 2, 0, 0],
    [3, 3, 3, 0, 2, 2, 0, 0],
    [0, 0, 0, 3, 3, 3, 0, 0],
    [1, 0, 0, 3, 3, 3, 0, 0],
    [0, 1, 0, 3, 3, 3, 0, 0],
    [2, 2, 0, 3, 3, 3, 0, 0],
    [0, 0, 4, 4, 4, 4, 0, 0],
    [1, 0, 4, 4, 4, 4, 0, 0],
    [0, 5, 5, 5, 5, 5, 0, 0],
    [6, 6, 6, 6, 6, 6, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 1, 0],
    [2, 2, 0, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 0, 1, 0],
    [1, 0, 1, 0, 0, 0, 1, 0],
    [0, 2, 2, 0, 0, 0, 1, 0],
    [3, 3, 3, 0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0, 0, 1, 0],
    [1, 0, 0, 1, 0, 0, 1, 0],
    [0, 1, 0, 1, 0, 0, 1, 0],
    [2, 2, 0, 1, 0, 0, 1, 0],
    [0, 0, 2, 2, 0, 0, 1, 0],
    [1, 0, 2, 2, 0, 0, 1, 0],
    [0, 3, 3, 3, 0, 0, 1, 0],
    [4, 4, 4, 4, 0, 0, 1, 0],
    [0, 0, 0, 0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1, 0, 1, 0],
    [0, 1, 0, 0, 1, 0, 1, 0],
    [2, 2, 0, 0, 1, 0, 1, 0],
    [0, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 0],
    [0, 2, 2, 0, 1, 0, 1, 0],
    [3, 3, 3, 0, 1, 0, 1, 0],
    [0, 0, 0, 2, 2, 0, 1, 0],
    [1, 0, 0, 2, 2, 0, 1, 0],
    [0, 1, 0, 2, 2, 0, 1, 0],
    [2, 2, 0, 2, 2, 0, 1, 0],
    [0, 0, 3, 3, 3, 0, 1, 0],
    [1, 0, 3, 3, 3, 0, 1, 0],
    [0, 4, 4, 4, 4, 0, 1, 0],
    [5, 5, 5, 5, 5, 0, 1, 0],
    [0, 0, 0, 0, 0, 2, 2, 0],
    [1, 0, 0, 0, 0, 2, 2, 0],
    [0, 1, 0, 0, 0, 2, 2, 0],
    [2, 2, 0, 0, 0, 2, 2, 0],
    [0, 0, 1, 0, 0, 2, 2, 0],
    [1, 0, 1, 0, 0, 2, 2, 0],
    [0, 2, 2, 0, 0, 2, 2, 0],
    [3, 3, 3, 0, 0, 2, 2, 0],
    [0, 0, 0, 1, 0, 2, 2, 0],
    [1, 0, 0, 1, 0, 2, 2, 0],
    [0, 1, 0, 1, 0, 2, 2, 0],
    [2, 2, 0, 1, 0, 2, 2, 0],
    [0, 0, 2, 2, 0, 2, 2, 0],
    [1, 0, 2, 2, 0, 2, 2, 0],
    [0, 3, 3, 3, 0, 2, 2, 0],
    [4, 4, 4, 4, 0, 2, 2, 0],
    [0, 0, 0, 0, 3, 3, 3, 0],
    [1, 0, 0, 0, 3, 3, 3, 0],
    [0, 1, 0, 0, 3, 3, 3, 0],
    [2, 2, 0, 0, 3, 3, 3, 0],
    [0, 0, 1, 0, 3, 3, 3, 0],
    [1, 0, 1, 0, 3, 3, 3, 0],
    [0, 2, 2, 0, 3, 3, 3, 0],
    [3, 3, 3, 0, 3, 3, 3, 0],
    [0, 0, 0, 4, 4, 4, 4, 0],
    [1, 0, 0, 4, 4, 4, 4, 0],
    [0, 1, 0, 4, 4, 4, 4, 0],
    [2, 2, 0, 4, 4, 4, 4, 0],
    [0, 0, 5, 5, 5, 5, 5, 0],
    [1, 0, 5, 5, 5, 5, 5, 0],
    [0, 6, 6, 6, 6, 6, 6, 0],
    [7, 7, 7, 7, 7, 7, 7, 0],
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, 0, 0, 1],
    [2, 2, 0, 0, 0, 0, 0, 1],
    [0, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [0, 2, 2, 0, 0, 0, 0, 1],
    [3, 3, 3, 0, 0, 0, 0, 1],
    [0, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0, 0, 0, 1],
    [2, 2, 0, 1, 0, 0, 0, 1],
    [0, 0, 2, 2, 0, 0, 0, 1],
    [1, 0, 2, 2, 0, 0, 0, 1],
    [0, 3, 3, 3, 0, 0, 0, 1],
    [4, 4, 4, 4, 0, 0, 0, 1],
    [0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1, 0, 0, 1],
    [2, 2, 0, 0, 1, 0, 0, 1],
    [0, 0, 1, 0, 1, 0, 0, 1],
    [1, 0, 1, 0, 1, 0, 0, 1],
    [0, 2, 2, 0, 1, 0, 0, 1],
    [3, 3, 3, 0, 1, 0, 0, 1],
    [0, 0, 0, 2, 2, 0, 0, 1],
    [1, 0, 0, 2, 2, 0, 0, 1],
    [0, 1, 0, 2, 2, 0, 0, 1],
    [2, 2, 0, 2, 2, 0, 0, 1],
    [0, 0, 3, 3, 3, 0, 0, 1],
    [1, 0, 3, 3, 3, 0, 0, 1],
    [0, 4, 4, 4, 4, 0, 0, 1],
    [5, 5, 5, 5, 5, 0, 0, 1],
    [0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1],
    [0, 1, 0, 0, 0, 1, 0, 1],
    [2, 2, 0, 0, 0, 1, 0, 1],
    [0, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [0, 2, 2, 0, 0, 1, 0, 1],
    [3, 3, 3, 0, 0, 1, 0, 1],
    [0, 0, 0, 1, 0, 1, 0, 1],
    [1, 0, 0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [2, 2, 0, 1, 0, 1, 0, 1],
    [0, 0, 2, 2, 0, 1, 0, 1],
    [1, 0, 2, 2, 0, 1, 0, 1],
    [0, 3, 3, 3, 0, 1, 0, 1],
    [4, 4, 4, 4, 0, 1, 0, 1],
    [0, 0, 0, 0, 2, 2, 0, 1],
    [1, 0, 0, 0, 2, 2, 0, 1],
    [0, 1, 0, 0, 2, 2, 0, 1],
    [2, 2, 0, 0, 2, 2, 0, 1],
    [0, 0, 1, 0, 2, 2, 0, 1],
    [1, 0, 1, 0, 2, 2, 0, 1],
    [0, 2, 2, 0, 2, 2, 0, 1],
    [3, 3, 3, 0, 2, 2, 0, 1],
    [0, 0, 0, 3, 3, 3, 0, 1],
    [1, 0, 0, 3, 3, 3, 0, 1],
    [0, 1, 0, 3, 3, 3, 0, 1],
    [2, 2, 0, 3, 3, 3, 0, 1],
    [0, 0, 4, 4, 4, 4, 0, 1],
    [1, 0, 4, 4, 4, 4, 0, 1],
    [0, 5, 5, 5, 5, 5, 0, 1],
    [6, 6, 6, 6, 6, 6, 0, 1],
    [0, 0, 0, 0, 0, 0, 2, 2],
    [1, 0, 0, 0, 0, 0, 2, 2],
    [0, 1, 0, 0, 0, 0, 2, 2],
    [2, 2, 0, 0, 0, 0, 2, 2],
    [0, 0, 1, 0, 0, 0, 2, 2],
    [1, 0, 1, 0, 0, 0, 2, 2],
    [0, 2, 2, 0, 0, 0, 2, 2],
    [3, 3, 3, 0, 0, 0, 2, 2],
    [0, 0, 0, 1, 0, 0, 2, 2],
    [1, 0, 0, 1, 0, 0, 2, 2],
    [0, 1, 0, 1, 0, 0, 2, 2],
    [2, 2, 0, 1, 0, 0, 2, 2],
    [0, 0, 2, 2, 0, 0, 2, 2],
    [1, 0, 2, 2, 0, 0, 2, 2],
    [0, 3, 3, 3, 0, 0, 2, 2],
    [4, 4, 4, 4, 0, 0, 2, 2],
    [0, 0, 0, 0, 1, 0, 2, 2],
    [1, 0, 0, 0, 1, 0, 2, 2],
    [0, 1, 0, 0, 1, 0, 2, 2],
    [2, 2, 0, 0, 1, 0, 2, 2],
    [0, 0, 1, 0, 1, 0, 2, 2],
    [1, 0, 1, 0, 1, 0, 2, 2],
    [0, 2, 2, 0, 1, 0, 2, 2],
    [3, 3, 3, 0, 1, 0, 2, 2],
    [0, 0, 0, 2, 2, 0, 2, 2],
    [1, 0, 0, 2, 2, 0, 2, 2],
    [0, 1, 0, 2, 2, 0, 2, 2],
    [2, 2, 0, 2, 2, 0, 2, 2],
    [0, 0, 3, 3, 3, 0, 2, 2],
    [1, 0, 3, 3, 3, 0, 2, 2],
    [0, 4, 4, 4, 4, 0, 2, 2],
    [5, 5, 5, 5, 5, 0, 2, 2],
    [0, 0, 0, 0, 0, 3, 3, 3],
    [1, 0, 0, 0, 0, 3, 3, 3],
    [0, 1, 0, 0, 0, 3, 3, 3],
    [2, 2, 0, 0, 0, 3, 3, 3],
    [0, 0, 1, 0, 0, 3, 3, 3],
    [1, 0, 1, 0, 0, 3, 3, 3],
    [0, 2, 2, 0, 0, 3, 3, 3],
    [3, 3, 3, 0, 0, 3, 3, 3],
    [0, 0, 0, 1, 0, 3, 3, 3],
    [1, 0, 0, 1, 0, 3, 3, 3],
    [0, 1, 0, 1, 0, 3, 3, 3],
    [2, 2, 0, 1, 0, 3, 3, 3],
    [0, 0, 2, 2, 0, 3, 3, 3],
    [1, 0, 2, 2, 0, 3, 3, 3],
    [0, 3, 3, 3, 0, 3, 3, 3],
    [4, 4, 4, 4, 0, 3, 3, 3],
    [0, 0, 0, 0, 4, 4, 4, 4],
    [1, 0, 0, 0, 4, 4, 4, 4],
    [0, 1, 0, 0, 4, 4, 4, 4],
    [2, 2, 0, 0, 4, 4, 4, 4],
    [0, 0, 1, 0, 4, 4, 4, 4],
    [1, 0, 1, 0, 4, 4, 4, 4],
    [0, 2, 2, 0, 4, 4, 4, 4],
    [3, 3, 3, 0, 4, 4, 4, 4],
    [0, 0, 0, 5, 5, 5, 5, 5],
    [1, 0, 0, 5, 5, 5, 5, 5],
    [0, 1, 0, 5, 5, 5, 5, 5],
    [2, 2, 0, 5, 5, 5, 5, 5],
    [0, 0, 6, 6, 6, 6, 6, 6],
    [1, 0, 6, 6, 6, 6, 6, 6],
    [0, 7, 7, 7, 7, 7, 7, 7],
    [8, 8, 8, 8, 8, 8, 8, 8],
];

pub const U8_INDEX_TABLE: [[u8; 8]; 256] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 2, 2, 2, 2, 2, 2],
    [0, 1, 2, 2, 2, 2, 2, 2],
    [0, 0, 0, 3, 3, 3, 3, 3],
    [0, 1, 1, 3, 3, 3, 3, 3],
    [0, 0, 2, 3, 3, 3, 3, 3],
    [0, 1, 2, 3, 3, 3, 3, 3],
    [0, 0, 0, 0, 4, 4, 4, 4],
    [0, 1, 1, 1, 4, 4, 4, 4],
    [0, 0, 2, 2, 4, 4, 4, 4],
    [0, 1, 2, 2, 4, 4, 4, 4],
    [0, 0, 0, 3, 4, 4, 4, 4],
    [0, 1, 1, 3, 4, 4, 4, 4],
    [0, 0, 2, 3, 4, 4, 4, 4],
    [0, 1, 2, 3, 4, 4, 4, 4],
    [0, 0, 0, 0, 0, 5, 5, 5],
    [0, 1, 1, 1, 1, 5, 5, 5],
    [0, 0, 2, 2, 2, 5, 5, 5],
    [0, 1, 2, 2, 2, 5, 5, 5],
    [0, 0, 0, 3, 3, 5, 5, 5],
    [0, 1, 1, 3, 3, 5, 5, 5],
    [0, 0, 2, 3, 3, 5, 5, 5],
    [0, 1, 2, 3, 3, 5, 5, 5],
    [0, 0, 0, 0, 4, 5, 5, 5],
    [0, 1, 1, 1, 4, 5, 5, 5],
    [0, 0, 2, 2, 4, 5, 5, 5],
    [0, 1, 2, 2, 4, 5, 5, 5],
    [0, 0, 0, 3, 4, 5, 5, 5],
    [0, 1, 1, 3, 4, 5, 5, 5],
    [0, 0, 2, 3, 4, 5, 5, 5],
    [0, 1, 2, 3, 4, 5, 5, 5],
    [0, 0, 0, 0, 0, 0, 6, 6],
    [0, 1, 1, 1, 1, 1, 6, 6],
    [0, 0, 2, 2, 2, 2, 6, 6],
    [0, 1, 2, 2, 2, 2, 6, 6],
    [0, 0, 0, 3, 3, 3, 6, 6],
    [0, 1, 1, 3, 3, 3, 6, 6],
    [0, 0, 2, 3, 3, 3, 6, 6],
    [0, 1, 2, 3, 3, 3, 6, 6],
    [0, 0, 0, 0, 4, 4, 6, 6],
    [0, 1, 1, 1, 4, 4, 6, 6],
    [0, 0, 2, 2, 4, 4, 6, 6],
    [0, 1, 2, 2, 4, 4, 6, 6],
    [0, 0, 0, 3, 4, 4, 6, 6],
    [0, 1, 1, 3, 4, 4, 6, 6],
    [0, 0, 2, 3, 4, 4, 6, 6],
    [0, 1, 2, 3, 4, 4, 6, 6],
    [0, 0, 0, 0, 0, 5, 6, 6],
    [0, 1, 1, 1, 1, 5, 6, 6],
    [0, 0, 2, 2, 2, 5, 6, 6],
    [0, 1, 2, 2, 2, 5, 6, 6],
    [0, 0, 0, 3, 3, 5, 6, 6],
    [0, 1, 1, 3, 3, 5, 6, 6],
    [0, 0, 2, 3, 3, 5, 6, 6],
    [0, 1, 2, 3, 3, 5, 6, 6],
    [0, 0, 0, 0, 4, 5, 6, 6],
    [0, 1, 1, 1, 4, 5, 6, 6],
    [0, 0, 2, 2, 4, 5, 6, 6],
    [0, 1, 2, 2, 4, 5, 6, 6],
    [0, 0, 0, 3, 4, 5, 6, 6],
    [0, 1, 1, 3, 4, 5, 6, 6],
    [0, 0, 2, 3, 4, 5, 6, 6],
    [0, 1, 2, 3, 4, 5, 6, 6],
    [0, 0, 0, 0, 0, 0, 0, 7],
    [0, 1, 1, 1, 1, 1, 1, 7],
    [0, 0, 2, 2, 2, 2, 2, 7],
    [0, 1, 2, 2, 2, 2, 2, 7],
    [0, 0, 0, 3, 3, 3, 3, 7],
    [0, 1, 1, 3, 3, 3, 3, 7],
    [0, 0, 2, 3, 3, 3, 3, 7],
    [0, 1, 2, 3, 3, 3, 3, 7],
    [0, 0, 0, 0, 4, 4, 4, 7],
    [0, 1, 1, 1, 4, 4, 4, 7],
    [0, 0, 2, 2, 4, 4, 4, 7],
    [0, 1, 2, 2, 4, 4, 4, 7],
    [0, 0, 0, 3, 4, 4, 4, 7],
    [0, 1, 1, 3, 4, 4, 4, 7],
    [0, 0, 2, 3, 4, 4, 4, 7],
    [0, 1, 2, 3, 4, 4, 4, 7],
    [0, 0, 0, 0, 0, 5, 5, 7],
    [0, 1, 1, 1, 1, 5, 5, 7],
    [0, 0, 2, 2, 2, 5, 5, 7],
    [0, 1, 2, 2, 2, 5, 5, 7],
    [0, 0, 0, 3, 3, 5, 5, 7],
    [0, 1, 1, 3, 3, 5, 5, 7],
    [0, 0, 2, 3, 3, 5, 5, 7],
    [0, 1, 2, 3, 3, 5, 5, 7],
    [0, 0, 0, 0, 4, 5, 5, 7],
    [0, 1, 1, 1, 4, 5, 5, 7],
    [0, 0, 2, 2, 4, 5, 5, 7],
    [0, 1, 2, 2, 4, 5, 5, 7],
    [0, 0, 0, 3, 4, 5, 5, 7],
    [0, 1, 1, 3, 4, 5, 5, 7],
    [0, 0, 2, 3, 4, 5, 5, 7],
    [0, 1, 2, 3, 4, 5, 5, 7],
    [0, 0, 0, 0, 0, 0, 6, 7],
    [0, 1, 1, 1, 1, 1, 6, 7],
    [0, 0, 2, 2, 2, 2, 6, 7],
    [0, 1, 2, 2, 2, 2, 6, 7],
    [0, 0, 0, 3, 3, 3, 6, 7],
    [0, 1, 1, 3, 3, 3, 6, 7],
    [0, 0, 2, 3, 3, 3, 6, 7],
    [0, 1, 2, 3, 3, 3, 6, 7],
    [0, 0, 0, 0, 4, 4, 6, 7],
    [0, 1, 1, 1, 4, 4, 6, 7],
    [0, 0, 2, 2, 4, 4, 6, 7],
    [0, 1, 2, 2, 4, 4, 6, 7],
    [0, 0, 0, 3, 4, 4, 6, 7],
    [0, 1, 1, 3, 4, 4, 6, 7],
    [0, 0, 2, 3, 4, 4, 6, 7],
    [0, 1, 2, 3, 4, 4, 6, 7],
    [0, 0, 0, 0, 0, 5, 6, 7],
    [0, 1, 1, 1, 1, 5, 6, 7],
    [0, 0, 2, 2, 2, 5, 6, 7],
    [0, 1, 2, 2, 2, 5, 6, 7],
    [0, 0, 0, 3, 3, 5, 6, 7],
    [0, 1, 1, 3, 3, 5, 6, 7],
    [0, 0, 2, 3, 3, 5, 6, 7],
    [0, 1, 2, 3, 3, 5, 6, 7],
    [0, 0, 0, 0, 4, 5, 6, 7],
    [0, 1, 1, 1, 4, 5, 6, 7],
    [0, 0, 2, 2, 4, 5, 6, 7],
    [0, 1, 2, 2, 4, 5, 6, 7],
    [0, 0, 0, 3, 4, 5, 6, 7],
    [0, 1, 1, 3, 4, 5, 6, 7],
    [0, 0, 2, 3, 4, 5, 6, 7],
    [0, 1, 2, 3, 4, 5, 6, 7],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 2, 2, 2, 2, 2, 2],
    [0, 1, 2, 2, 2, 2, 2, 2],
    [0, 0, 0, 3, 3, 3, 3, 3],
    [0, 1, 1, 3, 3, 3, 3, 3],
    [0, 0, 2, 3, 3, 3, 3, 3],
    [0, 1, 2, 3, 3, 3, 3, 3],
    [0, 0, 0, 0, 4, 4, 4, 4],
    [0, 1, 1, 1, 4, 4, 4, 4],
    [0, 0, 2, 2, 4, 4, 4, 4],
    [0, 1, 2, 2, 4, 4, 4, 4],
    [0, 0, 0, 3, 4, 4, 4, 4],
    [0, 1, 1, 3, 4, 4, 4, 4],
    [0, 0, 2, 3, 4, 4, 4, 4],
    [0, 1, 2, 3, 4, 4, 4, 4],
    [0, 0, 0, 0, 0, 5, 5, 5],
    [0, 1, 1, 1, 1, 5, 5, 5],
    [0, 0, 2, 2, 2, 5, 5, 5],
    [0, 1, 2, 2, 2, 5, 5, 5],
    [0, 0, 0, 3, 3, 5, 5, 5],
    [0, 1, 1, 3, 3, 5, 5, 5],
    [0, 0, 2, 3, 3, 5, 5, 5],
    [0, 1, 2, 3, 3, 5, 5, 5],
    [0, 0, 0, 0, 4, 5, 5, 5],
    [0, 1, 1, 1, 4, 5, 5, 5],
    [0, 0, 2, 2, 4, 5, 5, 5],
    [0, 1, 2, 2, 4, 5, 5, 5],
    [0, 0, 0, 3, 4, 5, 5, 5],
    [0, 1, 1, 3, 4, 5, 5, 5],
    [0, 0, 2, 3, 4, 5, 5, 5],
    [0, 1, 2, 3, 4, 5, 5, 5],
    [0, 0, 0, 0, 0, 0, 6, 6],
    [0, 1, 1, 1, 1, 1, 6, 6],
    [0, 0, 2, 2, 2, 2, 6, 6],
    [0, 1, 2, 2, 2, 2, 6, 6],
    [0, 0, 0, 3, 3, 3, 6, 6],
    [0, 1, 1, 3, 3, 3, 6, 6],
    [0, 0, 2, 3, 3, 3, 6, 6],
    [0, 1, 2, 3, 3, 3, 6, 6],
    [0, 0, 0, 0, 4, 4, 6, 6],
    [0, 1, 1, 1, 4, 4, 6, 6],
    [0, 0, 2, 2, 4, 4, 6, 6],
    [0, 1, 2, 2, 4, 4, 6, 6],
    [0, 0, 0, 3, 4, 4, 6, 6],
    [0, 1, 1, 3, 4, 4, 6, 6],
    [0, 0, 2, 3, 4, 4, 6, 6],
    [0, 1, 2, 3, 4, 4, 6, 6],
    [0, 0, 0, 0, 0, 5, 6, 6],
    [0, 1, 1, 1, 1, 5, 6, 6],
    [0, 0, 2, 2, 2, 5, 6, 6],
    [0, 1, 2, 2, 2, 5, 6, 6],
    [0, 0, 0, 3, 3, 5, 6, 6],
    [0, 1, 1, 3, 3, 5, 6, 6],
    [0, 0, 2, 3, 3, 5, 6, 6],
    [0, 1, 2, 3, 3, 5, 6, 6],
    [0, 0, 0, 0, 4, 5, 6, 6],
    [0, 1, 1, 1, 4, 5, 6, 6],
    [0, 0, 2, 2, 4, 5, 6, 6],
    [0, 1, 2, 2, 4, 5, 6, 6],
    [0, 0, 0, 3, 4, 5, 6, 6],
    [0, 1, 1, 3, 4, 5, 6, 6],
    [0, 0, 2, 3, 4, 5, 6, 6],
    [0, 1, 2, 3, 4, 5, 6, 6],
    [0, 0, 0, 0, 0, 0, 0, 7],
    [0, 1, 1, 1, 1, 1, 1, 7],
    [0, 0, 2, 2, 2, 2, 2, 7],
    [0, 1, 2, 2, 2, 2, 2, 7],
    [0, 0, 0, 3, 3, 3, 3, 7],
    [0, 1, 1, 3, 3, 3, 3, 7],
    [0, 0, 2, 3, 3, 3, 3, 7],
    [0, 1, 2, 3, 3, 3, 3, 7],
    [0, 0, 0, 0, 4, 4, 4, 7],
    [0, 1, 1, 1, 4, 4, 4, 7],
    [0, 0, 2, 2, 4, 4, 4, 7],
    [0, 1, 2, 2, 4, 4, 4, 7],
    [0, 0, 0, 3, 4, 4, 4, 7],
    [0, 1, 1, 3, 4, 4, 4, 7],
    [0, 0, 2, 3, 4, 4, 4, 7],
    [0, 1, 2, 3, 4, 4, 4, 7],
    [0, 0, 0, 0, 0, 5, 5, 7],
    [0, 1, 1, 1, 1, 5, 5, 7],
    [0, 0, 2, 2, 2, 5, 5, 7],
    [0, 1, 2, 2, 2, 5, 5, 7],
    [0, 0, 0, 3, 3, 5, 5, 7],
    [0, 1, 1, 3, 3, 5, 5, 7],
    [0, 0, 2, 3, 3, 5, 5, 7],
    [0, 1, 2, 3, 3, 5, 5, 7],
    [0, 0, 0, 0, 4, 5, 5, 7],
    [0, 1, 1, 1, 4, 5, 5, 7],
    [0, 0, 2, 2, 4, 5, 5, 7],
    [0, 1, 2, 2, 4, 5, 5, 7],
    [0, 0, 0, 3, 4, 5, 5, 7],
    [0, 1, 1, 3, 4, 5, 5, 7],
    [0, 0, 2, 3, 4, 5, 5, 7],
    [0, 1, 2, 3, 4, 5, 5, 7],
    [0, 0, 0, 0, 0, 0, 6, 7],
    [0, 1, 1, 1, 1, 1, 6, 7],
    [0, 0, 2, 2, 2, 2, 6, 7],
    [0, 1, 2, 2, 2, 2, 6, 7],
    [0, 0, 0, 3, 3, 3, 6, 7],
    [0, 1, 1, 3, 3, 3, 6, 7],
    [0, 0, 2, 3, 3, 3, 6, 7],
    [0, 1, 2, 3, 3, 3, 6, 7],
    [0, 0, 0, 0, 4, 4, 6, 7],
    [0, 1, 1, 1, 4, 4, 6, 7],
    [0, 0, 2, 2, 4, 4, 6, 7],
    [0, 1, 2, 2, 4, 4, 6, 7],
    [0, 0, 0, 3, 4, 4, 6, 7],
    [0, 1, 1, 3, 4, 4, 6, 7],
    [0, 0, 2, 3, 4, 4, 6, 7],
    [0, 1, 2, 3, 4, 4, 6, 7],
    [0, 0, 0, 0, 0, 5, 6, 7],
    [0, 1, 1, 1, 1, 5, 6, 7],
    [0, 0, 2, 2, 2, 5, 6, 7],
    [0, 1, 2, 2, 2, 5, 6, 7],
    [0, 0, 0, 3, 3, 5, 6, 7],
    [0, 1, 1, 3, 3, 5, 6, 7],
    [0, 0, 2, 3, 3, 5, 6, 7],
    [0, 1, 2, 3, 3, 5, 6, 7],
    [0, 0, 0, 0, 4, 5, 6, 7],
    [0, 1, 1, 1, 4, 5, 6, 7],
    [0, 0, 2, 2, 4, 5, 6, 7],
    [0, 1, 2, 2, 4, 5, 6, 7],
    [0, 0, 0, 3, 4, 5, 6, 7],
    [0, 1, 1, 3, 4, 5, 6, 7],
    [0, 0, 2, 3, 4, 5, 6, 7],
    [0, 1, 2, 3, 4, 5, 6, 7],
];
