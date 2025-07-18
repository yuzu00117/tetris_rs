use macroquad::prelude::*;

pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub rotate: bool,
    pub hard_drop: bool,
}

impl InputState {
    /// 毎フレーム呼び出して最新のキー状態を得る
    pub fn poll() -> Self {
        Self {
            left: is_key_down(KeyCode::Left),
            right: is_key_down(KeyCode::Right),
            down: is_key_down(KeyCode::Down),
            rotate: is_key_pressed(KeyCode::Up),
            hard_drop: is_key_pressed(KeyCode::Space),
        }
    }
}
