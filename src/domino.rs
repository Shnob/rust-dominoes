use nannou::prelude::*;

pub enum DominoState {
    Up,
    Fall,
    Down,
}

pub struct Domino {
    pos: Vec2,
    dir: Vec2,
    width: f32,
    depth: f32,
    height: f32,
    state: DominoState,
}

impl Domino {
    pub fn new(pos: Vec2, dir: Vec2, shape: (f32, f32, f32)) -> Domino {
        let dir = dir.normalize();
        Domino {pos, dir, width: shape.0, depth: shape.1, height: shape.2, state: DominoState::Up}
    }

    pub fn knock(&mut self) {
        match self.state {
            DominoState::Up => self.state = DominoState::Fall,
            _ => (),
        }
    }

    pub fn falling(&self) -> bool {
        match self.state {
            DominoState::Fall => true,
            _ => false,
        }
    }

    pub fn update(&mut self) {
        match self.state {
            DominoState::Fall => {
                self.state = DominoState::Down;
                self.pos += self.dir * (self.height/2.0);
            },
            _ => (),
        }
    }

    pub fn coll(&self, other: &Domino) -> bool {
        true
    }

    pub fn show(&self, draw: &Draw) {
        let rad = self.dir.angle() - PI / 2.0;
        match self.state {
            DominoState::Up | DominoState::Fall => {
                draw.rect().xy(self.pos).w_h(self.width, self.depth).rotate(rad).no_fill().stroke(WHITE).stroke_weight(1.0);
            }
            DominoState::Down => {
                draw.rect().xy(self.pos).w_h(self.width, self.height).rotate(rad).no_fill().stroke(WHITE).stroke_weight(1.0);
            }
        }
    }
}
