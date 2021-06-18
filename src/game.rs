use crate::input;
use crate::texture;
use crate::camera;
use crate::r_backend;
use crate::tetromino;
use crate::game_options;
use crate::tetris_board;

use winit::event::*;

pub struct Game {
    inputs: input::Inputs,
    tet: tetromino::Tetromino,
    board: tetris_board::TetrisBoard,
    left: bool,
    right: bool,
    ticks: u32,
    down_tick: u32,
    clear_tick: u32,
    next_pieces: [u8; 6],
}

impl Game {
    pub fn new() -> Game {
        let inputs = input::Inputs::new();
        let tet = tetromino::Tetromino::new_piece(rand::random::<u8>() % 7);
        let board = tetris_board::TetrisBoard::new();
        let left = false;
        let right = false;
        let ticks = 0;
        let down_tick = 60;
        let clear_tick = 20;
        let next_pieces = [
            rand::random::<u8>() % 7,
            rand::random::<u8>() % 7,
            rand::random::<u8>() % 7,
            rand::random::<u8>() % 7,
            rand::random::<u8>() % 7,
            rand::random::<u8>() % 7,
        ];
        Game { inputs, tet, board, left, right, ticks, down_tick, clear_tick, next_pieces }
    }

    pub fn init(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, r: &mut r_backend::Renderer) {
        r.load_sprite_array(device, queue, "./res/pieces/default/*.png", "default_pieces".to_string());
        //r.load_sprite(device, queue, std::path::Path::new("./res/piece.png"), "red".to_string());
        //r.load_sprite(device, queue, std::path::Path::new("./res/grey_piece.png"), "grey".to_string());
    }

    pub fn input(&mut self, event: &WindowEvent) {

        self.inputs.input(event);

        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let pressed = *state == ElementState::Pressed;
                if *keycode == VirtualKeyCode::Space && pressed {
                    while !self.tet.force_down(&mut self.board) {
                        //Push all the way down
                    }
                    self.update_next_pieces();
                    if !self.board.check_piece_fits(&self.tet) {
                        self.board = tetris_board::TetrisBoard::new();
                    }
                    self.clear_tick = self.ticks + 20;
                }
                if *keycode == VirtualKeyCode::Down && pressed {
                    if self.tet.force_down(&mut self.board) {
                        self.update_next_pieces();
                        if !self.board.check_piece_fits(&self.tet) {
                            self.board = tetris_board::TetrisBoard::new();
                        }
                        self.clear_tick = self.ticks + 20;
                    }
                }
                if *keycode == VirtualKeyCode::Left && pressed {
                    self.tet.move_piece(1, 0, &self.board);
                }
                if *keycode == VirtualKeyCode::Right && pressed {
                    self.tet.move_piece(-1, 0, &self.board);
                }
            }
            _ => (),
        }
    }

    pub fn process_inputs(&mut self, w_di: [u32; 2]) {

        if self.inputs.keys[VirtualKeyCode::Z as usize] && !self.right {
            self.tet.rotate_piece(-1, &self.board);
            self.right = true;
        }
        if !self.inputs.keys[VirtualKeyCode::Z as usize] {
            self.right = false;
        }
        if self.inputs.keys[VirtualKeyCode::X as usize] && !self.left {
            self.tet.rotate_piece(1, &self.board);
            self.left = true;
        }
        if !self.inputs.keys[VirtualKeyCode::X as usize] {
            self.left = false;
        }
    }

    pub fn update(&mut self) {

        self.ticks += 1;
        if self.ticks % self.down_tick == 0 {
            if self.tet.force_down(&mut self.board) {
                self.update_next_pieces();
                if !self.board.check_piece_fits(&self.tet) {
                    self.board = tetris_board::TetrisBoard::new();
                }
                self.clear_tick = self.ticks + 20;
            }
        }

        if self.board.did_zero {
            if self.ticks == self.clear_tick {
                self.board.clear_lines();
                self.board.did_zero = false;
            }
        }
    }

    fn update_next_pieces(&mut self) {
        self.tet = tetromino::Tetromino::new_piece(self.next_pieces[0]);
        for i in 0..5 {
            self.next_pieces[i] = self.next_pieces[i + 1]; 
        } 
        self.next_pieces[5] =  rand::random::<u8>() % 7;
    }

    pub fn render(&mut self, r: &mut r_backend::Renderer, camera: &mut camera::Camera) {

        self.board.render(r);
        self.tet.render(r, &self.board);
        //println!("{:?}", camera.position);
        let speed = 0.03;
        if self.inputs.keys[VirtualKeyCode::T as usize] {
            camera.position.y += 1.0 * speed;
        }
        if self.inputs.keys[VirtualKeyCode::G as usize] {
            camera.position.y -= 1.0 * speed;
        }

        if self.inputs.keys[VirtualKeyCode::H as usize] {
            camera.position.x += 1.0 * speed;
        }
        if self.inputs.keys[VirtualKeyCode::F as usize] {
            camera.position.x -= 1.0 * speed;
        }

        for i in 0..6 {
            let next_up = tetromino::Tetromino::new_piece(self.next_pieces[i]);
            next_up.render_force_position(r, -5.0, 20.0 - (i as f32 * 3.0));
        }
    }
}