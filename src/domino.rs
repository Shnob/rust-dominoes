use nannou::prelude::*;
use parry2d::shape::ConvexPolygon;
use parry2d::math::Point;

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

    pub fn knock(&mut self, dir: Vec2) {
        match self.state {
            DominoState::Up => {
                self.state = DominoState::Fall;
                if self.dir.dot(dir) < 0.0 {self.dir *= -1.0;};
            },
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
        let a_points: Vec<Point<f32>> = vec![
            Point::new(self.width/2.0 * self.dir.x + self.pos.x, self.height/2.0 * self.dir.y + self.pos.y),
            Point::new(-self.width/2.0 * self.dir.x + self.pos.x, self.height/2.0 * self.dir.y + self.pos.y),
            Point::new(self.width/2.0 * self.dir.x + self.pos.x, -self.height/2.0 * self.dir.y + self.pos.y),
            Point::new(-self.width/2.0 * self.dir.x + self.pos.x, -self.height/2.0 * self.dir.y + self.pos.y),
        ];
        let a = ConvexPolygon::from_convex_hull(a_points.as_slice()).unwrap();

        let b_points: Vec<Point<f32>> = vec![
            Point::new(other.width/2.0 * other.dir.x + other.pos.x, other.height/2.0 * other.dir.y + other.pos.y),
            Point::new(-other.width/2.0 * other.dir.x + other.pos.x, other.height/2.0 * other.dir.y + other.pos.y),
            Point::new(other.width/2.0 * other.dir.x + other.pos.x, -other.height/2.0 * other.dir.y + other.pos.y),
            Point::new(-other.width/2.0 * other.dir.x + other.pos.x, -other.height/2.0 * other.dir.y + other.pos.y),
        ];
        let b = ConvexPolygon::from_convex_hull(b_points.as_slice()).unwrap();

        let q = parry2d::query::contact::contact_support_map_support_map(
            &parry2d::math::Isometry::new(parry2d::na::Vector2::new(0.0, 0.0), 0.0),
            &a,
            &b,
            0.0
        );

        q.is_some()
    }

    pub fn dir(&self) -> Vec2 {
        self.dir
    }

    pub fn show(&self, draw: &Draw) {
        let rad = self.dir.angle() - PI / 2.0;
        match self.state {
            DominoState::Up | DominoState::Fall => {
                draw.rect().xy(self.pos).w_h(self.width, self.depth).rotate(rad).no_fill().stroke(WHITE).stroke_weight(0.8);
            }
            DominoState::Down => {
                draw.rect().xy(self.pos).w_h(self.width, self.height).rotate(rad).no_fill().stroke(WHITE).stroke_weight(0.8);
            }
        }
    }
}
