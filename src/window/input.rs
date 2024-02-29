use hashbrown::HashMap;
use winit::keyboard::PhysicalKey;
pub use winit::{
    event::MouseButton,
    keyboard::KeyCode,
};

use crate::types::{
    Delta,
    Position,
};

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PressType {
    Unknown,
    JustPressed,
    Pressed,
    JustReleased,
    Released,
}

#[derive(Debug)]
pub struct PressState {
    pub pt: PressType,
}

//

#[derive(Debug)]
pub struct Input {
    // keyboard
    keys_changed: bool,
    keys: HashMap<KeyCode, PressState>,

    // mouse
    pressed_buttons: HashMap<MouseButton, PressState>,

    mouse_position_changed: bool,
    mouse_position: Position<f32>,

    mouse_motion_changed: bool,
    mouse_motion: Delta<f64>,

    mouse_scroll_changed: bool,
    mouse_wheel: Delta<f32>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            // keyboard
            keys_changed: false,
            keys: HashMap::new(),

            // mouse
            pressed_buttons: HashMap::new(),

            mouse_position_changed: false,
            mouse_position: Position::default(),

            mouse_motion_changed: false,
            mouse_motion: Delta::new(0., 0.),

            mouse_scroll_changed: false,
            mouse_wheel: Delta::new(0., 0.),
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn process_key(&mut self, key: PhysicalKey, state: bool) {
        let key_code = match key {
            PhysicalKey::Code(code) => code,
            PhysicalKey::Unidentified(key) => {
                log::error!("Failed registering unknown key: {:?}", key); // TODO: handle this
                return;
            },
        };

        if let Some(key) = self.keys.get(&key_code) {
            self.keys.insert(key_code, match state {
                true => {
                    if key.pt == PressType::Pressed || key.pt == PressType::JustPressed {
                        PressState {
                            pt: PressType::Pressed,
                        }
                    } else {
                        PressState {
                            pt: PressType::JustPressed,
                        }
                    }
                },
                false => {
                    if key.pt == PressType::Released || key.pt == PressType::JustReleased {
                        PressState {
                            pt: PressType::Released,
                        }
                    } else {
                        PressState {
                            pt: PressType::JustReleased,
                        }
                    }
                },
            });
        } else {
            self.keys.insert(key_code, match state {
                true => PressState {
                    pt: PressType::JustPressed,
                },
                false => PressState {
                    pt: PressType::JustReleased,
                },
            });
        }

        self.keys_changed = true;
    }

    pub(crate) fn process_mouse_button(&mut self, button: MouseButton, state: bool) {
        self.pressed_buttons.insert(button, match state {
            true => PressState {
                pt: PressType::JustPressed,
            },
            false => PressState {
                pt: PressType::JustReleased,
            },
        });
    }

    pub(crate) fn set_mouse_position(&mut self, position: (f32, f32)) {
        self.mouse_position = position.into();
        self.mouse_position_changed = true;
    }

    pub(crate) fn process_mouse_wheel(&mut self, delta_x: f32, delta_y: f32) {
        self.mouse_wheel.x += delta_x;
        self.mouse_wheel.y += delta_y;
        self.mouse_scroll_changed = true;
    }

    pub(crate) fn process_mouse_motion(&mut self, delta: (f64, f64)) {
        self.mouse_motion.x += delta.0;
        self.mouse_motion.y += delta.1;
        self.mouse_motion_changed = true;
    }

    pub fn mouse_position(&self) -> Position<f32> {
        self.mouse_position
    }

    pub fn mouse_motion(&self) -> Delta<f64> {
        self.mouse_motion
    }

    pub fn mouse_scroll(&self) -> Delta<f32> {
        self.mouse_wheel
    }

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        match self
            .pressed_buttons
            .get(&button)
            .unwrap_or(&PressState {
                pt: PressType::Unknown,
            })
            .pt
        {
            PressType::Pressed | PressType::JustPressed => true,
            PressType::Released | PressType::JustReleased => false,
            PressType::Unknown => false,
        }
    }

    pub fn mouse_button_released(&self, button: MouseButton) -> bool {
        match self
            .pressed_buttons
            .get(&button)
            .unwrap_or(&PressState {
                pt: PressType::Unknown,
            })
            .pt
        {
            PressType::Pressed | PressType::JustPressed => false,
            PressType::Released | PressType::JustReleased => true,
            PressType::Unknown => true,
        }
    }

    pub fn mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        match self
            .pressed_buttons
            .get(&button)
            .unwrap_or(&PressState {
                pt: PressType::Unknown,
            })
            .pt
        {
            PressType::JustPressed => true,
            PressType::JustReleased => false,
            _ => false,
        }
    }

    pub fn mouse_button_just_released(&self, button: MouseButton) -> bool {
        match self
            .pressed_buttons
            .get(&button)
            .unwrap_or(&PressState {
                pt: PressType::Unknown,
            })
            .pt
        {
            PressType::JustPressed => false,
            PressType::JustReleased => true,
            _ => false,
        }
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        match self
            .keys
            .get(&key)
            .unwrap_or(&PressState {
                pt: PressType::Unknown,
            })
            .pt
        {
            PressType::Pressed | PressType::JustPressed => true,
            PressType::Released | PressType::JustReleased => false,
            PressType::Unknown => false,
        }
    }

    pub fn key_released(&self, key: KeyCode) -> bool {
        match self
            .keys
            .get(&key)
            .unwrap_or(&PressState {
                pt: PressType::Unknown,
            })
            .pt
        {
            PressType::Pressed | PressType::JustPressed => false,
            PressType::Released | PressType::JustReleased => true,
            PressType::Unknown => true,
        }
    }

    pub fn key_just_pressed(&self, key: KeyCode) -> bool {
        match self
            .keys
            .get(&key)
            .unwrap_or(&PressState {
                pt: PressType::Unknown,
            })
            .pt
        {
            PressType::JustPressed => true,
            PressType::JustReleased => false,
            _ => false,
        }
    }

    pub fn key_just_released(&self, key: KeyCode) -> bool {
        match self
            .keys
            .get(&key)
            .unwrap_or(&PressState {
                pt: PressType::Unknown,
            })
            .pt
        {
            PressType::JustPressed => false,
            PressType::JustReleased => true,
            _ => false,
        }
    }

    pub fn mouse_position_changed(&self) -> bool {
        self.mouse_position_changed
    }

    pub fn mouse_motion_changed(&self) -> bool {
        self.mouse_motion_changed
    }

    pub fn mouse_scroll_changed(&self) -> bool {
        self.mouse_scroll_changed
    }

    pub fn keys_changed(&self) -> bool {
        self.keys_changed
    }

    pub fn keys(&self) -> &HashMap<KeyCode, PressState> {
        &self.keys
    }

    pub fn consume_mouse(&mut self) {
        self.reset_vals();

        for pb in self.pressed_buttons.iter_mut() {
            pb.1.pt = PressType::Released;
        }
    }

    pub fn consume_keys(&mut self) {
        self.reset_vals();

        for key in self.keys.iter_mut() {
            key.1.pt = PressType::Released;
        }
    }

    fn reset_vals(&mut self) {
        self.mouse_motion = Delta::new(0., 0.);
        self.mouse_wheel = Delta::new(0., 0.);

        self.mouse_position_changed = false;
        self.mouse_scroll_changed = false;
        self.mouse_motion_changed = false;
        self.keys_changed = false;
    }

    pub(crate) fn reset(&mut self) {
        self.reset_vals();

        for pb in self.pressed_buttons.iter_mut() {
            match pb.1.pt {
                PressType::JustPressed => pb.1.pt = PressType::Pressed,
                PressType::JustReleased => pb.1.pt = PressType::Released,
                _ => (),
            }
        }

        for key in self.keys.iter_mut() {
            match key.1.pt {
                PressType::JustPressed => key.1.pt = PressType::Pressed,
                PressType::JustReleased => key.1.pt = PressType::Released,
                _ => (),
            }
        }
    }
}
