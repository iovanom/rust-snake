use bevy::prelude::Resource;

use crate::board::Position;
use std::collections::VecDeque;

#[derive(Debug, Resource)]
pub struct Snake {
    pub segments: VecDeque<Position>,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            segments: VecDeque::from([
                Position { x: 4, y: 4 },
                Position { x: 3, y: 4 },
                Position { x: 2, y: 4 },
            ]),
        }
    }
}
