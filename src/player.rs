use rusty_sword_arena::game::{Color, PlayerState};
use rusty_sword_arena::gfx::{Shape, Window};

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub body: Shape,
    pub sword: Shape,
}

impl Player {
    pub fn new(state: PlayerState, window: &Window) -> Self {
        let body = Shape::new_circle(
            window,
            state.radius,
            state.pos,
            state.direction,
            state.color,
        );
        let sword = Shape::new_ring(
            window,
            state.weapon.radius,
            state.pos,
            state.direction,
            Color::new(1.0, 0.0, 0.0),
        );
        Self {
            state,
            body,
            sword,
        }
    }
    pub fn update_state(&mut self, state: PlayerState) {
        self.body.pos = state.pos;
        self.body.direction = state.direction;
        self.sword.pos = state.pos;
        self.sword.direction = state.direction;
        self.state = state;
    }
}
