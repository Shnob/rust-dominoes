pub enum DominoState {
    Up,
    Falling,
    Down
}

pub struct Domino {
    pos: (f32, f32),
    dir: (f32, f32),
    width: f32,
    depth: f32,
    height: f32,
    state: DominoState,
}
