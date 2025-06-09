use std::collections::HashSet;

use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct Input {
    keys_pressed: HashSet<PhysicalKey>,
}

impl Input {
    pub(in crate::engine) fn handle_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key,
                        state,
                        ..
                    },
                ..
            } => {
                if state == &ElementState::Pressed {
                    self.keys_pressed.insert(physical_key.to_owned());
                } else {
                    self.keys_pressed.remove(physical_key);
                }

                true
            }
            _ => false,
        }
    }

    pub fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.keys_pressed.contains(&PhysicalKey::Code(keycode))
    }
}
