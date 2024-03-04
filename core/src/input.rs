use super::*;

#[derive(Clone, Debug)]
pub struct MouseState {
    pub position: LogicalPosition<f32>,
    pub buttons: MouseButtons,
}

#[derive(Clone, Debug)]
pub struct MouseInput {
    pub button: MouseButton,
    pub button_state: ButtonState,
    pub mouse_state: MouseState,
}

#[derive(Clone, Debug)]
pub struct CursorMoved {
    pub mouse_state: MouseState,
}

#[derive(Clone, Debug)]
pub struct MouseWheel {
    pub axis: MouseWheelAxis,
    pub distance: i32,
    pub mouse_state: MouseState,
}

#[derive(Clone, Debug)]
pub struct KeyInput {
    pub vkey: VirtualKey,
    pub key_state: KeyState,
}

#[derive(Clone, Debug)]
pub struct Clause {
    pub range: std::ops::Range<usize>,
    pub targeted: bool,
}

#[derive(Clone, Debug)]
pub struct Composition {
    pub chars: Vec<char>,
    pub clauses: Vec<Clause>,
    pub cursor_position: usize,
}

#[derive(Clone, Debug)]
pub enum Input {
    MouseInput(MouseInput),
    CursorMoved(CursorMoved),
    KeyInput(KeyInput),
    CharInput(char),
    ImeBeginComposition,
    ImeUpdateComposition(Composition),
    ImeEndComposition(Option<String>),
}
