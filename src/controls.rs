use bevy::prelude::*;

#[derive(Resource)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

fn user_input(input: Res<Input<KeyCode>>, mut last_pressed: ResMut<Direction>) {
    if input.pressed(KeyCode::Up) {
        *last_pressed = Direction::Up;
    } else if input.pressed(KeyCode::Down) {
        *last_pressed = Direction::Down
    } else if input.pressed(KeyCode::Right) {
        *last_pressed = Direction::Right
    } else if input.pressed(KeyCode::Left) {
        *last_pressed = Direction::Left
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>().add_system(user_input);
    }
}
