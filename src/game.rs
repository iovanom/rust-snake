use bevy::prelude::*;

use crate::{
    board::{spawn_board, Position, SpawnSnakeSegment},
    controls,
    food::{Food, FoodPlugin, NewFoodEvent},
    snake::Snake,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.52, 0.73, 0.17)))
            .insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .init_resource::<Snake>()
            .add_startup_system(spawn_board)
            .add_plugin(FoodPlugin)
            .add_system(game_update);
    }
}

#[derive(Resource)]
struct GameTimer(Timer);

fn game_update(
    commands: Commands,
    time: Res<Time>,
    positions: Query<(Entity, &Position)>,
    mut timer: ResMut<GameTimer>,
    snake: ResMut<Snake>,
    input: Res<controls::Direction>,
    query_food: Query<(Entity, &Position), With<Food>>,
    food_event: EventWriter<NewFoodEvent>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        tick(commands, positions, snake, input, query_food, food_event);
    }
}

fn tick(
    mut commands: Commands,
    positions: Query<(Entity, &Position)>,
    mut snake: ResMut<Snake>,
    input: Res<controls::Direction>,
    query_food: Query<(Entity, &Position), With<Food>>,
    mut food_event: EventWriter<NewFoodEvent>,
) {
    let mut next_position = snake.segments[0].clone();
    match *input {
        controls::Direction::Up => {
            next_position.y += 1;
        }
        controls::Direction::Down => {
            next_position.y -= 1;
        }
        controls::Direction::Right => {
            next_position.x += 1;
        }
        controls::Direction::Left => {
            next_position.x -= 1;
        }
    }
    snake.segments.push_front(next_position);
    commands.add(SpawnSnakeSegment {
        position: next_position,
    });

    // spawn new food if current one was eaten
    let is_food = query_food.iter().find(|(_, pos)| &&next_position == pos);
    match is_food {
        Some((entity, _)) => {
            commands.entity(entity).despawn_recursive();
            food_event.send(NewFoodEvent);
        }
        None => {
            let old_tail = snake.segments.pop_back().unwrap();
            if let Some((entity, _)) = positions.iter().find(|(_, pos)| pos == &&old_tail) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
