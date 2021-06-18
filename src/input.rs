use winit::event::*;

pub struct Inputs {
    pub keys: [bool; 1024],
    pub mouse_pos: [f32; 2],
    pub mouse_btns: [bool; 128],
}

impl Inputs {

    pub fn new() -> Inputs {
        let keys: [bool; 1024] = [false; 1024];
        let mouse_pos: [f32; 2] = [0.0, 0.0];
        let mouse_btns: [bool; 128] = [false; 128];
        Inputs { keys, mouse_pos, mouse_btns }
    }

    pub fn input(&mut self, event: &WindowEvent) {

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
                if *keycode as usize >= 1024 {
                    println!("Key out of range {:?}", *keycode);
                }
                self.keys[*keycode as usize] = pressed;
            }
            WindowEvent::CursorMoved  { position, .. } => {
                self.mouse_pos[0] = position.x as f32;
                self.mouse_pos[1] = position.y as f32;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = *state == ElementState::Pressed;
                let btn = match *button {  
                    MouseButton::Left => {
                        0_usize
                    },
                    MouseButton::Right => {
                        1_usize
                    },
                    MouseButton::Other(r) => {
                        r as usize
                    },
                    _ => {
                        127_usize
                    },
                };
                self.mouse_btns[btn as usize] = pressed;
            }
            _ => {
                //not keyboard input
            },
        }
    }
}