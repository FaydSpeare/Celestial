use crate::types::*;
use crate::motion::*;
use bitintr::*;

/* Const Arrays */


pub const KING_MOVES: [u64; 64] = [0x302, 0x705, 0xe0a, 0x1c14, 0x3828, 0x7050, 0xe0a0, 0xc040,
                                   0x30203, 0x70507, 0xe0a0e, 0x1c141c, 0x382838, 0x705070, 0xe0a0e0, 0xc040c0,
                                   0x3020300, 0x7050700, 0xe0a0e00, 0x1c141c00, 0x38283800, 0x70507000, 0xe0a0e000, 0xc040c000,
                                   0x302030000, 0x705070000, 0xe0a0e0000, 0x1c141c0000, 0x3828380000, 0x7050700000, 0xe0a0e00000, 0xc040c00000, 
                                   0x30203000000, 0x70507000000, 0xe0a0e000000, 0x1c141c000000, 0x382838000000, 0x705070000000, 0xe0a0e0000000, 0xc040c0000000, 
                                   0x3020300000000, 0x7050700000000, 0xe0a0e00000000, 0x1c141c00000000, 0x38283800000000, 0x70507000000000, 0xe0a0e000000000, 0xc040c000000000, 
                                   0x302030000000000, 0x705070000000000, 0xe0a0e0000000000, 0x1c141c0000000000, 0x3828380000000000, 0x7050700000000000, 0xe0a0e00000000000, 0xc040c00000000000, 
                                   0x203000000000000, 0x507000000000000, 0xa0e000000000000, 0x141c000000000000, 0x2838000000000000, 0x5070000000000000, 0xa0e0000000000000, 0x40c0000000000000];

pub const KN_MOVES: [u64; 64] = [0x20400, 0x50800, 0xa1100, 0x142200, 0x284400, 0x508800, 0xa01000, 0x402000, 
                                0x2040004, 0x5080008, 0xa110011, 0x14220022, 0x28440044, 0x50880088, 0xa0100010, 0x40200020, 
                                0x204000402, 0x508000805, 0xa1100110a, 0x1422002214, 0x2844004428, 0x5088008850, 0xa0100010a0, 0x4020002040, 
                                0x20400040200, 0x50800080500, 0xa1100110a00, 0x142200221400, 0x284400442800, 0x508800885000, 0xa0100010a000, 0x402000204000, 
                                0x2040004020000, 0x5080008050000, 0xa1100110a0000, 0x14220022140000, 0x28440044280000, 0x50880088500000, 0xa0100010a00000, 0x40200020400000, 
                                0x204000402000000, 0x508000805000000, 0xa1100110a000000, 0x1422002214000000, 0x2844004428000000, 0x5088008850000000, 0xa0100010a0000000, 0x4020002040000000,
                                0x400040200000000, 0x800080500000000, 0x1100110a00000000, 0x2200221400000000, 0x4400442800000000, 0x8800885000000000, 0x100010a000000000, 0x2000204000000000, 
                                0x4020000000000, 0x8050000000000, 0x110a0000000000, 0x22140000000000, 0x44280000000000, 0x88500000000000, 0x10a00000000000, 0x20400000000000];

pub const W_PAWN_CAP: [u64; 64] = [0x200, 0x500, 0xa00, 0x1400, 0x2800, 0x5000, 0xa000, 0x4000,
                                   0x20000, 0x50000, 0xa0000, 0x140000, 0x280000, 0x500000, 0xa00000, 0x400000,
                                   0x2000000, 0x5000000, 0xa000000, 0x14000000, 0x28000000, 0x50000000, 0xa0000000, 0x40000000,
                                   0x200000000, 0x500000000, 0xa00000000, 0x1400000000, 0x2800000000, 0x5000000000, 0xa000000000, 0x4000000000, 
                                   0x20000000000, 0x50000000000, 0xa0000000000, 0x140000000000, 0x280000000000, 0x500000000000, 0xa00000000000, 0x400000000000,
                                   0x2000000000000, 0x5000000000000, 0xa000000000000, 0x14000000000000, 0x28000000000000, 0x50000000000000, 0xa0000000000000, 0x40000000000000, 
                                   0x200000000000000, 0x500000000000000, 0xa00000000000000, 0x1400000000000000, 0x2800000000000000, 0x5000000000000000, 0xa000000000000000, 0x4000000000000000, 
                                   0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

pub const B_PAWN_CAP: [u64; 64] = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 
                                   0x2, 0x5, 0xa, 0x14, 0x28, 0x50, 0xa0, 0x40, 
                                   0x200, 0x500, 0xa00, 0x1400, 0x2800, 0x5000, 0xa000, 0x4000,
                                   0x20000, 0x50000, 0xa0000, 0x140000, 0x280000, 0x500000, 0xa00000, 0x400000,
                                   0x2000000, 0x5000000, 0xa000000, 0x14000000, 0x28000000, 0x50000000, 0xa0000000, 0x40000000, 
                                   0x200000000, 0x500000000, 0xa00000000, 0x1400000000, 0x2800000000, 0x5000000000, 0xa000000000, 0x4000000000,
                                   0x20000000000, 0x50000000000, 0xa0000000000, 0x140000000000, 0x280000000000, 0x500000000000, 0xa00000000000, 0x400000000000, 
                                   0x2000000000000, 0x5000000000000, 0xa000000000000, 0x14000000000000, 0x28000000000000, 0x50000000000000, 0xa0000000000000, 0x40000000000000];

pub const FILE_MASK: [u64; 8] = [0x101010101010101, 0x202020202020202, 0x404040404040404, 0x808080808080808,
                                 0x1010101010101010, 0x2020202020202020, 0x4040404040404040, 0x8080808080808080];

pub const RANK_MASK: [u64; 8] = [0xFF, 0xFF00, 0xFF0000, 0xFF000000, 0xFF00000000, 0xFF0000000000, 0xFF000000000000, 0xFF00000000000000];

pub const DIAG_MASK: [u64; 15] = [0x1, 0x102, 0x10204, 0x1020408, 0x102040810, 0x10204081020, 0x1020408102040,
                    0x102040810204080, 0x204081020408000, 0x408102040800000, 0x810204080000000,
                    0x1020408000000000, 0x2040800000000000, 0x4080000000000000, 0x8000000000000000];

pub const ANTI_DIAG_MASK: [u64; 15] = [0x80, 0x8040, 0x804020, 0x80402010, 0x8040201008, 0x804020100804, 0x80402010080402,
                    0x8040201008040201, 0x4020100804020100, 0x2010080402010000, 0x1008040201000000,
                    0x804020100000000, 0x402010000000000, 0x201000000000000, 0x100000000000000];

/* SLIDING ATTACKS */

pub fn sliding_attacks(sq: usize, occ: u64) -> u64 {
    flat_sliding_attacks(sq, occ) | diag_sliding_attacks(sq, occ)
}

pub fn flat_sliding_attacks(sq: usize, occ: u64) -> u64 {
    horizontal_sliding_attacks(sq, occ) | vertical_sliding_attacks(sq, occ)
}

pub fn horizontal_sliding_attacks(sq: usize, occ: u64) -> u64 {
    let foc: u64 = (occ | FILE_MASK[0] | FILE_MASK[7]) & RANK_MASK[sq/8];
    let right: u64 = (foc - 2*SET_MASK[sq]);
    let left: u64 = (foc.rbit() - 2*SET_MASK[sq].rbit()).rbit();
    (right ^ left) & RANK_MASK[sq/8]
}

pub fn vertical_sliding_attacks(sq: usize, occ: u64) -> u64 {
    let foc: u64 = (occ | RANK_MASK[0] | RANK_MASK[7]) & FILE_MASK[sq%8];
    let right: u64 = (foc - 2*SET_MASK[sq]);
    let left: u64 = (foc.rbit() - 2*SET_MASK[sq].rbit()).rbit();
    (right ^ left) & FILE_MASK[sq%8]
}

pub fn diag_sliding_attacks(sq: usize, occ: u64) -> u64 {
    diagonal_sliding_attacks(sq, occ) | anti_diagonal_sliding_attacks(sq, occ)
}

pub fn diagonal_sliding_attacks(sq: usize, occ: u64) -> u64 {
    let foc: u64 = (occ | RANK_MASK[0] | RANK_MASK[7] | FILE_MASK[0] | FILE_MASK[7]) & DIAG_MASK[(sq/8)+(sq%8)];
    let right: u64 = (foc - 2*SET_MASK[sq]);
    let left: u64 = (foc.rbit() - 2*SET_MASK[sq].rbit()).rbit();
    (right ^ left) & DIAG_MASK[(sq/8)+(sq%8)]
}

pub fn anti_diagonal_sliding_attacks(sq: usize, occ: u64) -> u64 {
    let foc: u64 = (occ | RANK_MASK[0] | RANK_MASK[7] | FILE_MASK[0] | FILE_MASK[7]) & ANTI_DIAG_MASK[(sq/8)+7-(sq%8)];
    let right: u64 = (foc - 2*SET_MASK[sq]);
    let left: u64 = (foc.rbit() - 2*SET_MASK[sq].rbit()).rbit();
    (right ^ left) & ANTI_DIAG_MASK[(sq/8)+7-(sq%8)]
}

/* MOTION LIST ADDERS */

// PAWNS - Promotion Flag, EnPassnt Flag

pub fn add_pawn_motion(motion_list: &mut Vec<Motion>, from: i32, to: i32){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: 0
    })
}

pub fn add_pawn_promotion(motion_list: &mut Vec<Motion>, from: i32, to: i32, promotee: Promotee){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, promotee as u16, Flag::PROMOTION as u16),
        score: 0
    })
}

pub fn add_pawn_enpassant_motion(motion_list: &mut Vec<Motion>, from: i32, to: i32){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::ENPASSANT as u16),
        score: 0
    })
}

// KNIGHTS

pub fn add_knight_motion(motion_list: &mut Vec<Motion>, from: i32, to: i32){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: 0
    })
}

// BISHOP 

pub fn add_bishop_motion(motion_list: &mut Vec<Motion>, from: i32, to: i32){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: 0
    })
}

// ROOK 

pub fn add_rook_motion(motion_list: &mut Vec<Motion>, from: i32, to: i32){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: 0
    })
}

// QUEEN 

pub fn add_queen_motion(motion_list: &mut Vec<Motion>, from: i32, to: i32){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: 0
    })
}

// KING - Castling Flag

pub fn add_king_motion(motion_list: &mut Vec<Motion>, from: i32, to: i32){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: 0
    })
}

pub fn add_king_castling_motion(motion_list: &mut Vec<Motion>, from: i32, to: i32){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::CASTLING as u16),
        score: 0
    })
}

/* MOVE GENERATION */

pub gen_white_pawn_moves(motion_list: )


