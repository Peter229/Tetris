use crate::r_backend;
use crate::tetromino;
use crate::game_options;

use std::iter::*;
use std::collections::BTreeSet;

pub struct TetrisBoard {
    board: [u8; game_options::BOARD_WIDTH * game_options::BOARD_HEIGHT],
    pub did_zero: bool,
    empty_lines: BTreeSet<usize>,
}

impl TetrisBoard {
    pub fn new() -> TetrisBoard {
        let mut board: [u8; game_options::BOARD_WIDTH * game_options::BOARD_HEIGHT] = [0; game_options::BOARD_WIDTH * game_options::BOARD_HEIGHT];
        for i in 0..game_options::BOARD_WIDTH {
            board[0 * game_options::BOARD_WIDTH + i] = 8;
            board[(game_options::BOARD_HEIGHT - 1) * game_options::BOARD_WIDTH + i] = 8;
        }
        for i in 0..game_options::BOARD_HEIGHT {
            board[i * game_options::BOARD_WIDTH + 0] = 8;
            board[i * game_options::BOARD_WIDTH + (game_options::BOARD_WIDTH - 1)] = 8;
        }
        let did_zero = false;
        let empty_lines: BTreeSet<usize> = BTreeSet::new();
        TetrisBoard { board, did_zero, empty_lines }
    }

    pub fn render(&self, r: &mut r_backend::Renderer) {

        for x in 0..game_options::BOARD_WIDTH {
            for y in 0..game_options::BOARD_HEIGHT {
                if self.board[y * game_options::BOARD_WIDTH + x] > 0 {
                    r.render_sprite_array([x as f32 * -game_options::SCALE, y as f32 * game_options::SCALE], [1.0 * game_options::SCALE, 1.0 * game_options::SCALE], -1.0, "default_pieces".to_string(), self.board[y * game_options::BOARD_WIDTH + x] as i32 - 1);
                }
            }
        }
    }

    pub fn check_piece_fits(&self, piece: &tetromino::Tetromino) -> bool {

        for x in 0..piece.rotation_constant {
            for y in 0..piece.rotation_constant {
                let board_x = (piece.x - x) as usize;
                let board_y = (piece.y - y) as usize;
                let piece_index = y as usize * piece.rotation_constant as usize + x as usize;
                let board_index = board_y * game_options::BOARD_WIDTH as usize + board_x;
                if board_index >= game_options::BOARD_WIDTH as usize * game_options::BOARD_HEIGHT as usize && piece.piece_data[piece_index] > 0 {
                    return false;
                }
                if piece.piece_data[piece_index] > 0 && self.board[board_index] > 0 {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn add_piece_to_board(&mut self, piece: &tetromino::Tetromino) {

        for x in 0..piece.rotation_constant {
            for y in 0..piece.rotation_constant {
                let board_x = (piece.x - x) as usize;
                let board_y = (piece.y - y) as usize;
                if piece.piece_data[y as usize * piece.rotation_constant as usize + x as usize] > 0 {

                    self.board[board_y * game_options::BOARD_WIDTH as usize + board_x] = piece.piece_data[y as usize * piece.rotation_constant as usize + x as usize];
                }
            }
        }
    }

    pub fn check_line(&mut self, rotation_constant: u8, piece_y: u8) {

        for y in 0..rotation_constant {
            let mut should_zero = true;
            for x in 1..(game_options::BOARD_WIDTH - 1) {
            
                let board_y = (piece_y - y) as usize;
                
                if piece_y < y || board_y == 0 || board_y == game_options::BOARD_HEIGHT - 1 {
                    continue;
                }

                let board_index = board_y * game_options::BOARD_WIDTH as usize + x;

                if self.board[board_index] == 0 {
                    should_zero = false;
                    break;
                }
            }

            if should_zero {
                self.did_zero = true;
                for x in 1..(game_options::BOARD_WIDTH - 1) {

                    let board_y = (piece_y - y) as usize;

                    if piece_y < y || board_y == 0 || board_y == game_options::BOARD_HEIGHT - 1 {
                        continue;
                    }

                    let board_index = board_y * game_options::BOARD_WIDTH as usize + x;
                    self.empty_lines.insert(board_y);
                    self.board[board_index] = 9;
                }
            }
        }
    }

    pub fn clear_lines(&mut self) {

        if self.did_zero {
            for i in self.empty_lines.iter().rev() {
                for x in 1..(game_options::BOARD_WIDTH - 1) {
                    for y in *i..(game_options::BOARD_HEIGHT - 2) {
                        let board_index = y * game_options::BOARD_WIDTH as usize + x;
                        let board_index_above = (y + 1) * game_options::BOARD_WIDTH as usize + x;
                        self.board[board_index] = self.board[board_index_above];
                    }
                }
            }
            self.empty_lines.clear();
        }
    }
}