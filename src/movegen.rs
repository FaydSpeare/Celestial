use crate::types::*;
use crate::motion::*;
use crate::position::*;
use crate::attack::*;
use bitintr::*;

/* Const Arrays */ //position fen r1b1k2r/ppppnppp/2n2q2/2b5/3NP3/2P1B3/PP3PPP/RN1QKB1R w KQkq - 0 1

pub const MVV_LVA: [[i32; 13]; 13] = [
                                    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605, 0], 
                                    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604, 0], 
                                    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603, 0],
                                    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602, 0],
                                    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601, 0],
                                    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600, 0], 
                                    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605, 0], 
                                    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604, 0], 
                                    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603, 0],
                                    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602, 0],
                                    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601, 0],
                                    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600, 0], 
                                    [  5,   4,   3,   2,   1,   0,   5,   4,   3,   2,   1,   0, 0]

                                    ];



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

#[inline]
pub fn sliding_attacks(sq: usize, occ: u64) -> u64 {
    flat_sliding_attacks(sq, occ) | diag_sliding_attacks(sq, occ)
}

#[inline]
pub fn flat_sliding_attacks(sq: usize, occ: u64) -> u64 {
    horizontal_sliding_attacks(sq, occ) | vertical_sliding_attacks(sq, occ)
}

#[inline]
pub fn horizontal_sliding_attacks(sq: usize, occ: u64) -> u64 {
    ((occ.wrapping_sub(SET_MASK[sq].wrapping_mul(2))) ^ (occ.rbit().wrapping_sub(SET_MASK[sq].rbit().wrapping_mul(2))).rbit()) & RANK_MASK[sq/8]
}

#[inline]
pub fn vertical_sliding_attacks(sq: usize, occ: u64) -> u64 {
    let occ = occ & FILE_MASK[sq%8];
    ((occ.wrapping_sub(SET_MASK[sq].wrapping_mul(2))) ^ (occ.rbit().wrapping_sub(SET_MASK[sq].rbit().wrapping_mul(2))).rbit()) & FILE_MASK[sq%8]
}

#[inline]
pub fn diag_sliding_attacks(sq: usize, occ: u64) -> u64 {
    diagonal_sliding_attacks(sq, occ) | anti_diagonal_sliding_attacks(sq, occ)
}

#[inline]
pub fn diagonal_sliding_attacks(sq: usize, occ: u64) -> u64 {
    let index = (sq/8)+(sq%8);
    let foc: u64 = occ & DIAG_MASK[index];
    ((foc.wrapping_sub(SET_MASK[sq].wrapping_mul(2))) ^ (foc.rbit().wrapping_sub(SET_MASK[sq].rbit().wrapping_mul(2))).rbit()) & DIAG_MASK[index]
}

#[inline]
pub fn anti_diagonal_sliding_attacks(sq: usize, occ: u64) -> u64 {
    let index = (sq/8)+7-(sq%8);
    let foc: u64 = occ & ANTI_DIAG_MASK[index];
    ((foc.wrapping_sub(SET_MASK[sq].wrapping_mul(2))) ^ (foc.rbit().wrapping_sub(SET_MASK[sq].rbit().wrapping_mul(2))).rbit()) & ANTI_DIAG_MASK[index]
}

/* MOTION LIST ADDERS */

pub fn add_quiet_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    let m = MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16);
    let s = if pos.search_killers[0][pos.search_ply as usize] == m {
        900000
    } else if pos.search_killers[1][pos.search_ply as usize] == m {
        800000
    } else {
        pos.search_history[pos.board[from] as usize][to]
    };
    
    motion_list.push(Motion {
        motion: m,
        score: s,
        capture: false
    });
}

pub fn add_capture_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: MVV_LVA[pos.board[from] as usize][pos.board[to] as usize] + 1000000,
        capture: true
    });
}

// PAWNS - Promotion Flag, EnPassnt Flag

pub fn add_pawn_promotion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    let cap: bool = pos.board[to] != Piece::EMPTY as i32;
    let m = MOVE_INT!(from as u16, to as u16, Promotee::QUEEN as u16, Flag::PROMOTION as u16);
    let sco = if cap {
        MVV_LVA[pos.board[from] as usize][pos.board[to] as usize] + 1000000
    } else {
        if pos.search_killers[0][pos.search_ply as usize] == m {
            900000
        } else if pos.search_killers[1][pos.search_ply as usize] == m {
            800000
        } else {
            0
        }
    };
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, Promotee::QUEEN as u16, Flag::PROMOTION as u16),
        score: sco,
        capture: cap
    });
    let m = MOVE_INT!(from as u16, to as u16, Promotee::ROOK as u16, Flag::PROMOTION as u16);
    let sco = if cap {
        MVV_LVA[pos.board[from] as usize][pos.board[to] as usize] + 1000000
    } else {
        if pos.search_killers[0][pos.search_ply as usize] == m {
            900000
        } else if pos.search_killers[1][pos.search_ply as usize] == m {
            800000
        } else {
            0
        }
    };
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, Promotee::ROOK as u16, Flag::PROMOTION as u16),
        score: sco,
        capture: cap
    });
    let m = MOVE_INT!(from as u16, to as u16, Promotee::BISHOP as u16, Flag::PROMOTION as u16);
    let sco = if cap {
        MVV_LVA[pos.board[from] as usize][pos.board[to] as usize] + 1000000
    } else {
        if pos.search_killers[0][pos.search_ply as usize] == m {
            900000
        } else if pos.search_killers[1][pos.search_ply as usize] == m {
            800000
        } else {
            0
        }
    };
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, Promotee::BISHOP as u16, Flag::PROMOTION as u16),
        score: sco,
        capture: cap
    });
    let m = MOVE_INT!(from as u16, to as u16, Promotee::KNIGHT as u16, Flag::PROMOTION as u16);
    let sco = if cap {
        MVV_LVA[pos.board[from] as usize][pos.board[to] as usize] + 1000000
    } else {
        if pos.search_killers[0][pos.search_ply as usize] == m {
            900000
        } else if pos.search_killers[1][pos.search_ply as usize] == m {
            800000
        } else {
            0
        }
    };
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, Promotee::KNIGHT as u16, Flag::PROMOTION as u16),
        score: sco,
        capture: cap
    });
}

pub fn add_pawn_enpassant_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::ENPASSANT as u16),
        score: MVV_LVA[pos.board[from] as usize][pos.board[from] as usize] + 1000000, // from->from as pawn will capture pawn but move to empty square
        capture: true
    });
}

// KNIGHTS
/*
pub fn add_knight_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: MVV_LVA[pos.board[from] as usize][pos.board[to] as usize]
    });
}


// BISHOP 

pub fn add_bishop_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: MVV_LVA[pos.board[from] as usize][pos.board[to] as usize]
    });
}

// ROOK 

pub fn add_rook_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: MVV_LVA[pos.board[from] as usize][pos.board[to] as usize]
    });
}

// QUEEN 

pub fn add_queen_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: MVV_LVA[pos.board[from] as usize][pos.board[to] as usize]
    });
}


// KING - Castling Flag

pub fn add_king_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    motion_list.push(Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: MVV_LVA[pos.board[from] as usize][pos.board[to] as usize]
    });
}
*/

pub fn add_king_castling_motion(pos: &Position, motion_list: &mut Vec<Motion>, from: usize, to: usize){
    let m = MOVE_INT!(from as u16, to as u16, 0, Flag::CASTLING as u16);
    let sco = if pos.search_killers[0][pos.search_ply as usize] == m {
            900000
    } else if pos.search_killers[1][pos.search_ply as usize] == m {
            800000
    } else {
            0
    };

    motion_list.push(Motion {
        motion: m,
        score: sco,
        capture: false
    });
}

/* MOVE GENERATION */

//# PAWNS #//

pub fn gen_white_pawn_moves(motion_list: &mut Vec<Motion>, position: &Position) {

    for i in 0..(position.piece_num[Piece::W_PAWN as usize]) {
        let current = position.piece_list[Piece::W_PAWN as usize][i as usize] as usize;

        if current / 8 == Rank::RANK_2 as usize {

            // PAWN STARTS
            if (SET_MASK[current+8] | SET_MASK[current+16]) & position.colour_bb[Colour::BOTH as usize] == 0 {
                add_quiet_motion(position, motion_list, current, current + 16);
            }
        } 

        // PAWN FORWARD
        if SET_MASK[current+8] & position.colour_bb[Colour::BOTH as usize] == 0 {
            
            // CHECK PROMOTION
            if current / 8 == 6 {
                add_pawn_promotion(position, motion_list, current, current + 8);
            } else {
                add_quiet_motion(position, motion_list, current, current + 8);
            }
        }

        // PAWN CAPTURE RIGHT
        if current % 8 < 7 {
            if SET_MASK[current+9] & position.colour_bb[Colour::BLACK as usize] != 0 {

                // CHECK PROMOTION
                if current / 8 == 6 {
                    add_pawn_promotion(position, motion_list, current, current + 9);
                } else {
                    add_capture_motion(position, motion_list, current, current + 9);
                }
                
            }
        }
        
        // PAWN CAPTURE LEFT
        if current % 8 > 0 {
            if SET_MASK[current+7] & position.colour_bb[Colour::BLACK as usize] != 0 {
                
                // CHECK PROMOTION
                if current / 8 == 6 {
                    add_pawn_promotion(position, motion_list, current, current + 7);
                } else {
                    add_capture_motion(position, motion_list, current, current + 7);
                }
            }
        }

        // EP CAPTURE 
        if position.ep != Square::NO_SQ as i32 {

            //println!("ep{}",position.ep);
            if current % 8 < 7 {
                if current + 9 == position.ep as usize {
                    //println!("eps1");
                    add_pawn_enpassant_motion(position, motion_list, current, current + 9)
                }
            }
            
            if current % 8 > 0 {
                if current + 7 == position.ep as usize {
                    //println!("eps2");
                    add_pawn_enpassant_motion(position, motion_list, current, current + 7)
                }
            }
        }
    }

}

pub fn gen_black_pawn_moves(motion_list: &mut Vec<Motion>, position: &Position) {

    for i in 0..(position.piece_num[Piece::B_PAWN as usize]) {
        let current = position.piece_list[Piece::B_PAWN as usize][i as usize] as usize;

        if current / 8 == Rank::RANK_7 as usize {

            // PAWN STARTS
            if (SET_MASK[current-8] | SET_MASK[current-16]) & position.colour_bb[Colour::BOTH as usize] == 0 {
                add_quiet_motion(position, motion_list, current, current - 16);
            }
        } 

        // PAWN FORWARD
        if SET_MASK[current-8] & position.colour_bb[Colour::BOTH as usize] == 0 {
            
            // CHECK PROMOTION
            if current / 8 == 1 {
                add_pawn_promotion(position, motion_list, current, current - 8);
            } else {
                add_quiet_motion(position, motion_list, current, current - 8);
            }
        }

        // PAWN CAPTURE RIGHT
        if current % 8 < 7 {
            if SET_MASK[current-7] & position.colour_bb[Colour::WHITE as usize] != 0 {

                // CHECK PROMOTION
                if current / 8 == 1 {
                    add_pawn_promotion(position, motion_list, current, current - 7);
                } else {
                    add_capture_motion(position, motion_list, current, current - 7);
                }
                
            }
        }
        
        // PAWN CAPTURE LEFT
        if current % 8 > 0 {
            if SET_MASK[current-9] & position.colour_bb[Colour::WHITE as usize] != 0 {
                
                // CHECK PROMOTION
                if current / 8 == 1 {
                    add_pawn_promotion(position, motion_list, current, current - 9);
                } else {
                    add_capture_motion(position, motion_list, current, current - 9);
                }
            }
        }

        // EP CAPTURE 
        if position.ep != Square::NO_SQ as i32 {

            if current % 8 > 0 {
                if current - 9 == position.ep as usize {
                    //println!("eps");
                    add_pawn_enpassant_motion(position, motion_list, current, current - 9)
                }
            }
            
            if current % 8 < 7 {
                if current - 7 == position.ep as usize {
                   // println!("eps");
                    add_pawn_enpassant_motion(position, motion_list, current, current - 7)
                }
            }
            
        }
    }

}

//# KNIGHTS #//

pub fn gen_white_knight_moves(motion_list: &mut Vec<Motion>, position: &Position){

    for i in 0..(position.piece_num[Piece::W_KNIGHT as usize]) {
        let current = position.piece_list[Piece::W_KNIGHT as usize][i as usize] as usize;

        let mut kn_moves = KN_MOVES[current] & !position.colour_bb[Colour::WHITE as usize];

        while kn_moves != 0 {
            let next = LSB!(kn_moves) as usize;

            if position.board[next] == Piece::EMPTY as i32 {
                add_quiet_motion(position, motion_list, current, next);
            } else {
                add_capture_motion(position, motion_list, current, next);
            }
            

            kn_moves ^= 1 << next;
        }
    }
}

pub fn gen_black_knight_moves(motion_list: &mut Vec<Motion>, position: &Position){

    for i in 0..(position.piece_num[Piece::B_KNIGHT as usize]) {
        let current = position.piece_list[Piece::B_KNIGHT as usize][i as usize] as usize;

        let mut kn_moves = KN_MOVES[current] & !position.colour_bb[Colour::BLACK as usize];

        while kn_moves != 0 {
            let next = LSB!(kn_moves) as usize;

            if position.board[next] == Piece::EMPTY as i32 {
                add_quiet_motion(position, motion_list, current, next);
            } else {
                add_capture_motion(position, motion_list, current, next);
            }

            kn_moves ^= 1 << next;
        }
    }
}

//# BISHOPS #//

pub fn gen_white_bishop_moves(motion_list: &mut Vec<Motion>, position: &Position){

    for i in 0..(position.piece_num[Piece::W_BISHOP as usize]) {
        let current = position.piece_list[Piece::W_BISHOP as usize][i as usize] as usize;

        let mut bishop_moves = diag_sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::WHITE as usize];

        while bishop_moves != 0 {
            let next = LSB!(bishop_moves) as usize;

            if position.board[next] == Piece::EMPTY as i32 {
                add_quiet_motion(position, motion_list, current, next);
            } else {
                add_capture_motion(position, motion_list, current, next);
            }

            bishop_moves ^= 1 << next;
        }
    }
}

pub fn gen_black_bishop_moves(motion_list: &mut Vec<Motion>, position: &Position){

    for i in 0..(position.piece_num[Piece::B_BISHOP as usize]) {
        let current = position.piece_list[Piece::B_BISHOP as usize][i as usize] as usize;

        let mut bishop_moves = diag_sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::BLACK as usize];

        while bishop_moves != 0 {
            let next = LSB!(bishop_moves) as usize;

            if position.board[next] == Piece::EMPTY as i32 {
                add_quiet_motion(position, motion_list, current, next);
            } else {
                add_capture_motion(position, motion_list, current, next);
            }

            bishop_moves ^= 1 << next;
        }
    }
}

//# ROOKS #// 

pub fn gen_white_rook_moves(motion_list: &mut Vec<Motion>, position: &Position){

    for i in 0..(position.piece_num[Piece::W_ROOK as usize]) {
        let current = position.piece_list[Piece::W_ROOK as usize][i as usize] as usize;

        let mut rook_moves = flat_sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::WHITE as usize];

        while rook_moves != 0 {
            let next = LSB!(rook_moves) as usize;

            if position.board[next] == Piece::EMPTY as i32 {
                add_quiet_motion(position, motion_list, current, next);
            } else {
                add_capture_motion(position, motion_list, current, next);
            }

            rook_moves ^= 1 << next;
        }
    }
}

pub fn gen_black_rook_moves(motion_list: &mut Vec<Motion>, position: &Position){

    for i in 0..(position.piece_num[Piece::B_ROOK as usize]) {
        let current = position.piece_list[Piece::B_ROOK as usize][i as usize] as usize;

        let mut rook_moves = flat_sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::BLACK as usize];

        while rook_moves != 0 {
            let next = LSB!(rook_moves) as usize;

            if position.board[next] == Piece::EMPTY as i32 {
                add_quiet_motion(position, motion_list, current, next);
            } else {
                add_capture_motion(position, motion_list, current, next);
            }

            rook_moves ^= 1 << next;
        }
    }
}

//# QUEENS #//
pub fn gen_white_queen_moves(motion_list: &mut Vec<Motion>, position: &Position){

    for i in 0..(position.piece_num[Piece::W_QUEEN as usize]) {
        let current = position.piece_list[Piece::W_QUEEN as usize][i as usize] as usize;

        let mut queen_moves = sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::WHITE as usize];

        while queen_moves != 0 {
            let next = LSB!(queen_moves) as usize;

            if position.board[next] == Piece::EMPTY as i32 {
                add_quiet_motion(position, motion_list, current, next);
            } else {
                add_capture_motion(position, motion_list, current, next);
            }

            queen_moves ^= 1 << next;
        }
    }
}

pub fn gen_black_queen_moves(motion_list: &mut Vec<Motion>, position: &Position){

    for i in 0..(position.piece_num[Piece::B_QUEEN as usize]) {
        let current = position.piece_list[Piece::B_QUEEN as usize][i as usize] as usize;

        let mut queen_moves = sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::BLACK as usize];

        while queen_moves != 0 {
            let next = LSB!(queen_moves) as usize;

            if position.board[next] == Piece::EMPTY as i32 {
                add_quiet_motion(position, motion_list, current, next);
            } else {
                add_capture_motion(position, motion_list, current, next);
            }

            queen_moves ^= 1 << next;
        }
    }
}

//# KING #//

pub fn gen_white_king_moves(motion_list: &mut Vec<Motion>, position: &Position){
    let current = position.king_sq[Colour::WHITE as usize] as usize;
    let mut king_moves = KING_MOVES[current] & !position.colour_bb[Colour::WHITE as usize];

    while king_moves != 0 {
        let next = LSB!(king_moves) as usize;

        if position.board[next] == Piece::EMPTY as i32 {
            add_quiet_motion(position, motion_list, current, next);
        } else {
            add_capture_motion(position, motion_list, current, next);
        }

        king_moves ^= 1 << next;
    }

    if position.castling_rights & Castling::W_OO as u8 == Castling::W_OO as u8 {
        if position.board[5] == Piece::EMPTY as i32 && position.board[6] == Piece::EMPTY as i32 {
            if !is_attacked_by(position, 4, false) && !is_attacked_by(position, 5, false) {
                add_king_castling_motion(position, motion_list, 4, 6);
            }
        }
    }

    if position.castling_rights & Castling::W_OOO as u8 == Castling::W_OOO as u8 {
        if position.board[3] == Piece::EMPTY as i32 && position.board[2] == Piece::EMPTY as i32 && position.board[1] == Piece::EMPTY as i32 {
            if !is_attacked_by(position, 4, false) && !is_attacked_by(position, 3, false) {
                add_king_castling_motion(position, motion_list, 4, 2);
            }
        }
    }
}

pub fn gen_black_king_moves(motion_list: &mut Vec<Motion>, position: &Position){
    let current = position.king_sq[Colour::BLACK as usize] as usize;
    let mut king_moves = KING_MOVES[current] & !position.colour_bb[Colour::BLACK as usize];

    while king_moves != 0 {
        let next = LSB!(king_moves) as usize;

        if position.board[next] == Piece::EMPTY as i32 {
            add_quiet_motion(position, motion_list, current, next);
        } else {
            add_capture_motion(position, motion_list, current, next);
        }

        king_moves ^= 1 << next;
    }

    if position.castling_rights & Castling::B_OO as u8 == Castling::B_OO as u8 {
        if position.board[61] == Piece::EMPTY as i32 && position.board[62] == Piece::EMPTY as i32 {
            if !is_attacked_by(position, 60, true) && !is_attacked_by(position, 61, true) {
                add_king_castling_motion(position, motion_list, 60, 62);
            }
        }
    }

    if position.castling_rights & Castling::B_OOO as u8 == Castling::B_OOO as u8 {
        if position.board[59] == Piece::EMPTY as i32 && position.board[58] == Piece::EMPTY as i32 && position.board[57] == Piece::EMPTY as i32 {
            if !is_attacked_by(position, 60, true) && !is_attacked_by(position, 59, true) {
                add_king_castling_motion(position, motion_list, 60, 58);
            }
        }
    }
}



// # COLLECTIVE # //

pub fn gen_white_moves(motion_list: &mut Vec<Motion>, position: &Position){
    //println!("pawn");
    gen_white_pawn_moves(motion_list, position);
    //println!("knight");
    gen_white_knight_moves(motion_list, position);
    //println!("bishop");
    gen_white_bishop_moves(motion_list, position);
    //println!("rook");
    gen_white_rook_moves(motion_list, position);
    //println!("queen");
    gen_white_queen_moves(motion_list, position);
    //println!("king");
    gen_white_king_moves(motion_list, position);
    //println!("done");

}

pub fn gen_black_moves(motion_list: &mut Vec<Motion>, position: &Position){
    
    gen_black_pawn_moves(motion_list, position);
    gen_black_knight_moves(motion_list, position);
    gen_black_bishop_moves(motion_list, position);
    gen_black_rook_moves(motion_list, position);
    gen_black_queen_moves(motion_list, position);
    gen_black_king_moves(motion_list, position);

}

pub fn gen_moves(motion_list: &mut Vec<Motion>, position: &Position){

    if position.side_to_move {
        gen_white_moves(motion_list, position);
    } else {
        gen_black_moves(motion_list, position);
    }

}

pub fn gen_legal_moves(motion_list: &mut Vec<Motion>, position: &mut Position){

    let mut pseudos: Vec<Motion> = vec![];

    if position.side_to_move {
        gen_white_moves(&mut pseudos, position);
    
    } else {
        gen_black_moves(&mut pseudos, position);
    }

    for i in pseudos {
        if position.do_motion(&i) {
            position.undo_motion();
            motion_list.push(i);
        }
    }

}

pub fn gen_white_captures(motion_list: &mut Vec<Motion>, position: &Position) {

    for i in 0..(position.piece_num[Piece::W_PAWN as usize]) {
        let current = position.piece_list[Piece::W_PAWN as usize][i as usize] as usize;

        // PAWN CAPTURE RIGHT
        if current % 8 < 7 {
            if SET_MASK[current+9] & position.colour_bb[Colour::BLACK as usize] != 0 {

                // CHECK PROMOTION
                if current / 8 == 6 {
                    add_pawn_promotion(position, motion_list, current, current + 9);
                } else {
                    add_capture_motion(position, motion_list, current, current + 9);
                }
                
            }
        }
        
        // PAWN CAPTURE LEFT
        if current % 8 > 0 {
            if SET_MASK[current+7] & position.colour_bb[Colour::BLACK as usize] != 0 {
                
                // CHECK PROMOTION
                if current / 8 == 6 {
                    add_pawn_promotion(position, motion_list, current, current + 7);
                } else {
                    add_capture_motion(position, motion_list, current, current + 7);
                }
            }
        }

        // EP CAPTURE 
        if position.ep != Square::NO_SQ as i32 {

            //println!("ep{}",position.ep);
            if current % 8 < 7 {
                if current + 9 == position.ep as usize {
                    //println!("eps1");
                    add_pawn_enpassant_motion(position, motion_list, current, current + 9)
                }
            }
            
            if current % 8 > 0 {
                if current + 7 == position.ep as usize {
                    //println!("eps2");
                    add_pawn_enpassant_motion(position, motion_list, current, current + 7)
                }
            }
        }
    }

    for i in 0..(position.piece_num[Piece::W_KNIGHT as usize]) {
        let current = position.piece_list[Piece::W_KNIGHT as usize][i as usize] as usize;

        let mut kn_moves = KN_MOVES[current] & !position.colour_bb[Colour::WHITE as usize] & position.colour_bb[Colour::BLACK as usize];

        while kn_moves != 0 {
            let next = LSB!(kn_moves) as usize;

            add_capture_motion(position, motion_list, current, next);

            kn_moves ^= 1 << next;
        }
    }

    for i in 0..(position.piece_num[Piece::W_BISHOP as usize]) {
        let current = position.piece_list[Piece::W_BISHOP as usize][i as usize] as usize;

        let mut bishop_moves = diag_sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::WHITE as usize] & position.colour_bb[Colour::BLACK as usize];

        while bishop_moves != 0 {
            let next = LSB!(bishop_moves) as usize;

            add_capture_motion(position, motion_list, current, next);

            bishop_moves ^= 1 << next;
        }
    }

    for i in 0..(position.piece_num[Piece::W_ROOK as usize]) {
        let current = position.piece_list[Piece::W_ROOK as usize][i as usize] as usize;

        let mut rook_moves = flat_sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::WHITE as usize] & position.colour_bb[Colour::BLACK as usize];

        while rook_moves != 0 {
            let next = LSB!(rook_moves) as usize;

            add_capture_motion(position, motion_list, current, next);

            rook_moves ^= 1 << next;
        }
    }

    for i in 0..(position.piece_num[Piece::W_QUEEN as usize]) {
        let current = position.piece_list[Piece::W_QUEEN as usize][i as usize] as usize;

        let mut queen_moves = sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::WHITE as usize] & position.colour_bb[Colour::BLACK as usize];

        while queen_moves != 0 {
            let next = LSB!(queen_moves) as usize;

            add_capture_motion(position, motion_list, current, next);

            queen_moves ^= 1 << next;
        }
    }

    let current = position.king_sq[Colour::WHITE as usize] as usize;
    let mut king_moves = KING_MOVES[current] & !position.colour_bb[Colour::WHITE as usize] & position.colour_bb[Colour::BLACK as usize];

    while king_moves != 0 {
        let next = LSB!(king_moves) as usize;

        add_capture_motion(position, motion_list, current, next);

        king_moves ^= 1 << next;
    }
}

pub fn gen_black_captures(motion_list: &mut Vec<Motion>, position: &Position) {

    for i in 0..(position.piece_num[Piece::B_PAWN as usize]) {
        let current = position.piece_list[Piece::B_PAWN as usize][i as usize] as usize;

        // PAWN CAPTURE RIGHT
        if current % 8 < 7 {
            if SET_MASK[current-7] & position.colour_bb[Colour::WHITE as usize] != 0 {

                // CHECK PROMOTION
                if current / 8 == 1 {
                    add_pawn_promotion(position, motion_list, current, current - 7);
                } else {
                    add_capture_motion(position, motion_list, current, current - 7);
                }
                
            }
        }
        
        // PAWN CAPTURE LEFT
        if current % 8 > 0 {
            if SET_MASK[current-9] & position.colour_bb[Colour::WHITE as usize] != 0 {
                
                // CHECK PROMOTION
                if current / 8 == 1 {
                    add_pawn_promotion(position, motion_list, current, current - 9);
                } else {
                    add_capture_motion(position, motion_list, current, current - 9);
                }
            }
        }

        // EP CAPTURE 
        if position.ep != Square::NO_SQ as i32 {

            if current % 8 > 0 {
                if current - 9 == position.ep as usize {
                    //println!("eps");
                    add_pawn_enpassant_motion(position, motion_list, current, current - 9)
                }
            }
            
            if current % 8 < 7 {
                if current - 7 == position.ep as usize {
                   // println!("eps");
                    add_pawn_enpassant_motion(position, motion_list, current, current - 7)
                }
            }
            
        }
    }

    for i in 0..(position.piece_num[Piece::B_KNIGHT as usize]) {
        let current = position.piece_list[Piece::B_KNIGHT as usize][i as usize] as usize;

        let mut kn_moves = KN_MOVES[current] & !position.colour_bb[Colour::BLACK as usize] & position.colour_bb[Colour::WHITE as usize];

        while kn_moves != 0 {
            let next = LSB!(kn_moves) as usize;

            add_capture_motion(position, motion_list, current, next);

            kn_moves ^= 1 << next;
        }
    }

    for i in 0..(position.piece_num[Piece::B_BISHOP as usize]) {
        let current = position.piece_list[Piece::B_BISHOP as usize][i as usize] as usize;

        let mut bishop_moves = diag_sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::BLACK as usize] & position.colour_bb[Colour::WHITE as usize];

        while bishop_moves != 0 {
            let next = LSB!(bishop_moves) as usize;

            add_capture_motion(position, motion_list, current, next);

            bishop_moves ^= 1 << next;
        }
    }

    for i in 0..(position.piece_num[Piece::B_ROOK as usize]) {
        let current = position.piece_list[Piece::B_ROOK as usize][i as usize] as usize;

        let mut rook_moves = flat_sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::BLACK as usize] & position.colour_bb[Colour::WHITE as usize];

        while rook_moves != 0 {
            let next = LSB!(rook_moves) as usize;

            add_capture_motion(position, motion_list, current, next);

            rook_moves ^= 1 << next;
        }
    }

    for i in 0..(position.piece_num[Piece::B_QUEEN as usize]) {
        let current = position.piece_list[Piece::B_QUEEN as usize][i as usize] as usize;

        let mut queen_moves = sliding_attacks(current, position.colour_bb[Colour::BOTH as usize]) & !position.colour_bb[Colour::BLACK as usize] & position.colour_bb[Colour::WHITE as usize];

        while queen_moves != 0 {
            let next = LSB!(queen_moves) as usize;

            add_capture_motion(position, motion_list, current, next);

            queen_moves ^= 1 << next;
        }
    }

    let current = position.king_sq[Colour::BLACK as usize] as usize;
    let mut king_moves = KING_MOVES[current] & !position.colour_bb[Colour::BLACK as usize] & position.colour_bb[Colour::WHITE as usize];

    while king_moves != 0 {
        let next = LSB!(king_moves) as usize;

        add_capture_motion(position, motion_list, current, next);

        king_moves ^= 1 << next;
    }

}

pub fn gen_captures(motion_list: &mut Vec<Motion>, position: &Position){

    if position.side_to_move {
        gen_white_captures(motion_list, position);
    } else {
        gen_black_captures(motion_list, position);
    }

}

        
