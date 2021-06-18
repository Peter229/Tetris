use crate::r_backend;
use crate::game_options;
use crate::tetris_board;

const CYAN_DATA: [u8; 25] = [
    0, 0, 0, 0, 0,
    0, 0, 0, 0, 0,
    0, 1, 1, 1, 1,
    0, 0, 0, 0, 0,
    0, 0, 0, 0, 0
];

const BLUE_DATA: [u8; 9] = [
    2, 0, 0,
    2, 2, 2,
    0, 0, 0
];

const ORANGE_DATA: [u8; 9] = [
    0, 0, 3,
    3, 3, 3,
    0, 0, 0
];

const YELLOW_DATA: [u8; 9] = [
    0, 4, 4,
    0, 4, 4,
    0, 0, 0
];

const GREEN_DATA: [u8; 9] = [
    0, 5, 5,
    5, 5, 0,
    0, 0, 0
];

const PURPLE_DATA: [u8; 9] = [
    0, 6, 0,
    6, 6, 6,
    0, 0, 0
];

const RED_DATA: [u8; 9] = [
    7, 7, 0,
    0, 7, 7,
    0, 0, 0
];

//https://tetris.wiki/Super_Rotation_System#Wall_Kicks invert X
const OTHER_OFFSETS: [i8; 40] = [
    0, 0,   0, 0,    0, 0,  0, 0,   0, 0,
    0, 0,   -1, 0,   -1, -1,  0, 2,   -1, 2,
    0, 0,   0, 0,   0, 0,   0, 0,   0, 0,
    0, 0,   1, 0,  1, -1,   0, 2,   1, 2,
];

const CYAN_OFFSETS: [i8; 40] = [
    0, 0,   1, 0,    -2, 0,  1, 0,   -2, 0,
    1, 0,  0, 0,    0, 0,  0, 1,    0, -2,
    1, 1,  -1, 1,   2, 1,  -1, 0,   2, 0,
    0, 1,   0, 1,   0, 1,   0, -1,  0, 2
];

const YELLOW_OFFSETS: [i8; 8] = [
    0, 0,
    0, -1,
    1, -1,
    1, 0
];

/*
0 1 2
3 4 5
6 7 8
*/

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub piece_data: Vec<u8>,
    static_piece_data: Vec<u8>,
    piece_offsets: Vec<i8>,
    pub rotation_constant: u8,
    rotation_90: u8,
    rotation_180: u8,
    rotation_270: u8,
    rotation: u8,
    pub x: u8,
    pub y: u8,
}

impl Tetromino {

    pub fn new_piece(piece: u8) -> Tetromino {
        let (x, y, piece_data, piece_offsets) = match piece {
            0 => (8, 22, CYAN_DATA.to_vec(), CYAN_OFFSETS.to_vec()),
            1 => (7, 20, BLUE_DATA.to_vec(), OTHER_OFFSETS.to_vec()),
            2 => (7, 20, ORANGE_DATA.to_vec(), OTHER_OFFSETS.to_vec()),
            3 => (7, 20, YELLOW_DATA.to_vec(), YELLOW_OFFSETS.to_vec()),
            4 => (7, 20, GREEN_DATA.to_vec(), OTHER_OFFSETS.to_vec()),
            5 => (7, 20, PURPLE_DATA.to_vec(), OTHER_OFFSETS.to_vec()),
            _ => (7, 20, RED_DATA.to_vec(), OTHER_OFFSETS.to_vec()),
        };
        let static_piece_data = piece_data.clone();
        let piece_data_len = piece_data.len() as i32;
        let rotation_constant = (piece_data_len as f32).sqrt() as u8;
        let rotation_90 = (rotation_constant - 1) as u8 * rotation_constant;
        let rotation_180 = (piece_data_len - 1) as u8;
        let rotation_270 = (rotation_constant - 1);
        let rotation = 0;
        Tetromino { piece_data, static_piece_data, piece_offsets, rotation_constant, rotation_90, rotation_180, rotation_270, rotation, x, y }
    }

    pub fn get_ghost(&mut self, board: &tetris_board::TetrisBoard) -> (u8, u8) {

        let orgi_x = self.x;
        let orgi_y = self.y;

        let mut ghost_y = self.y;
        let mut ghost_x = self.x;

        while board.check_piece_fits(&self) {
            ghost_y = self.y;
            ghost_x = self.x;

            self.x = (self.x as i8) as u8;
            self.y = ((self.y as i8) - 1) as u8;
        }

        self.x = orgi_x;
        self.y = orgi_y;

        (ghost_x, ghost_y)
    }

    pub fn force_down(&mut self, board: &mut tetris_board::TetrisBoard) -> bool {

        let old_y = self.y;
        let old_x = self.x;

        self.x = (self.x as i8) as u8;
        self.y = ((self.y as i8) - 1) as u8;

        if !board.check_piece_fits(&self) {
            self.x = old_x;
            self.y = old_y;
            self.add_piece_to_board(board);
            board.check_line(self.rotation_constant, self.y);
            return true;
        }

        return false;
    }

    pub fn move_piece(&mut self, dx: i8, dy: i8, board: &tetris_board::TetrisBoard) {

        let old_y = self.y;
        let old_x = self.x;

        self.x = ((self.x as i8) + dx) as u8;
        self.y = ((self.y as i8) + dy) as u8;

        if !board.check_piece_fits(&self) {
            self.x = old_x;
            self.y = old_y;
        }
    }

    pub fn rotate_piece(&mut self, rotate: i8, board: &tetris_board::TetrisBoard) {
        let mut temp_rotate = self.rotation as i8 + rotate;
        let start_data = self.piece_data.clone();
        if temp_rotate < 0 {
            temp_rotate = 3;
        }
        else if temp_rotate > 3 {
            temp_rotate = 0;
        }
        for i in 0..self.static_piece_data.len() {
            let mut i_x = (i as u8 % self.rotation_constant) as usize;
            let mut i_y = (i as u8 / self.rotation_constant) as usize;
            match temp_rotate {
                0 => {
                    self.piece_data[i] = self.static_piece_data[i_y * self.rotation_constant as usize + i_x];
                }
                1 => {
                    self.piece_data[i] = self.static_piece_data[self.rotation_90 as usize + i_y - (i_x * self.rotation_constant as usize)];
                }
                2 => {
                    self.piece_data[i] = self.static_piece_data[self.rotation_180 as usize - (i_y * self.rotation_constant as usize) - i_x];
                }
                3 => {
                    self.piece_data[i] = self.static_piece_data[self.rotation_270 as usize - i_y + (i_x * self.rotation_constant as usize)];
                }
                _ => (),
            }
        }

        let o_x = self.x;
        let o_y = self.y;
        let mut cant_fit = true;

        let mut num_to_check = 5;
        let mut offset: i8 = 10;
        if self.piece_offsets.len() < 30 {
            num_to_check = 1;
            offset = 2;
        }

        for i in 0..num_to_check {

            self.x = (self.x as i8 + (self.piece_offsets[(self.rotation * offset as u8) as usize + (i * 2)] - self.piece_offsets[(temp_rotate * offset) as usize + (i * 2)])) as u8;
            self.y = (self.y as i8 + (self.piece_offsets[(self.rotation * offset as u8) as usize + 1 + (i * 2)] - self.piece_offsets[(temp_rotate * offset) as usize + 1 + (i * 2)])) as u8;

            //println!("{} {}", self.piece_offsets[(self.rotation * 2) as usize] - self.piece_offsets[(temp_rotate * 2) as usize], self.piece_offsets[(self.rotation * 2) as usize + 1] - self.piece_offsets[(temp_rotate * 2) as usize + 1]);
            if board.check_piece_fits(&self) {
                cant_fit = false;
                self.rotation = temp_rotate as u8;
                break;
            }
            else {
                self.x = o_x;
                self.y = o_y;
            }
        }

        /*self.x = (self.x as i8 + (self.piece_offsets[(self.rotation * 2) as usize] - self.piece_offsets[(temp_rotate * 2) as usize])) as u8;
        self.y = (self.y as i8 + (self.piece_offsets[(self.rotation * 2) as usize + 1] - self.piece_offsets[(temp_rotate * 2) as usize + 1])) as u8;
        println!("{} {}", self.piece_offsets[(self.rotation * 2) as usize] - self.piece_offsets[(temp_rotate * 2) as usize], self.piece_offsets[(self.rotation * 2) as usize + 1] - self.piece_offsets[(temp_rotate * 2) as usize + 1]);
        if board.check_piece_fits(&self) {
            cant_fit = false;
            self.rotation = temp_rotate as u8;
        }*/

        if cant_fit {
            self.piece_data = start_data;
            self.x = o_x;
            self.y = o_y;
        }
    }

    pub fn add_piece_to_board(&self, board: &mut tetris_board::TetrisBoard) {
        board.add_piece_to_board(&self);
    }

    pub fn render(&mut self, r: &mut r_backend::Renderer, b: &tetris_board::TetrisBoard) {
        //println!("{} {}", self.x, self.y);
        for i in 0..self.piece_data.len() {
            if self.piece_data[i] > 0 {
                let mut i_x: f32 = (i as u8 % self.rotation_constant) as f32;
                let mut i_y: f32 = (i as u8 / self.rotation_constant) as f32;
                i_y = self.y as f32 - i_y;
                i_x = self.x as f32 - i_x;
                r.render_sprite_array([i_x * -game_options::SCALE, i_y * game_options::SCALE], [1.0 * game_options::SCALE, 1.0 * game_options::SCALE], -1.0, "default_pieces".to_string(), self.piece_data[i] as i32 - 1);
            }
        }

        let (ghost_x, ghost_y) = self.get_ghost(b);
        for i in 0..self.piece_data.len() {
            if self.piece_data[i] > 0 {
                let mut i_x: f32 = (i as u8 % self.rotation_constant) as f32;
                let mut i_y: f32 = (i as u8 / self.rotation_constant) as f32;
                i_y = ghost_y as f32 - i_y;
                i_x = ghost_x as f32 - i_x;
                r.render_sprite_transparent([i_x * -game_options::SCALE, i_y * game_options::SCALE], [1.0 * game_options::SCALE, 1.0 * game_options::SCALE], -1.0, "default_pieces".to_string(), self.piece_data[i] as i32 - 1, 0.3);
            }
        }
    }

    pub fn render_force_position(&self, r: &mut r_backend::Renderer, x: f32, y: f32) {
        //println!("{} {}", self.x, self.y);
        for i in 0..self.piece_data.len() {
            if self.piece_data[i] > 0 {
                let mut i_x: f32 = (i as u8 % self.rotation_constant) as f32;
                let mut i_y: f32 = (i as u8 / self.rotation_constant) as f32;
                i_y = y as f32 - i_y;
                i_x = x as f32 - i_x;
                r.render_sprite_array([i_x * -game_options::SCALE, i_y * game_options::SCALE], [1.0 * game_options::SCALE, 1.0 * game_options::SCALE], -1.0, "default_pieces".to_string(), self.piece_data[i] as i32 - 1);
            }
        }
    }
}