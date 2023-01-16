use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
pub enum OkiAction {
    #[default]
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    Up,
    Down,
    Left,
    Right,
}
