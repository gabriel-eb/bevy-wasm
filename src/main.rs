use bevy::prelude::*;
use rand::random;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_BODY_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const FOOD_COLOR: Color = Color::rgb(0.3, 0.8, 0.5);
const SET_ARENA_WIDTH: u32 = 10;
const SET_ARENA_HEIGHT: u32 = 10;
const ARENA_WIDTH: f32 = SET_ARENA_WIDTH as f32;
const ARENA_HEIGHT: f32 = SET_ARENA_HEIGHT as f32;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.09, 0.09, 0.09)))
        .insert_resource(Time::<Fixed>::from_seconds(0.4))
        .insert_resource(SnakeSegments::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SssssSnake!".into(),
                name: Some("sssssnakebevy.app".into()),
                resolution: (500., 500.).into(),
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(FixedUpdate, (food_spawner, snake_movement))
        .add_systems(Update, snake_movement_input.before(snake_movement))
        .add_systems(PostUpdate, (position_translation, size_scailing))
        .run();
}

#[derive(Component)]
struct MyCameraMaker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMaker,
    ));
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: SNAKE_HEAD_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(10.0, 10.0, 10.0),
                        ..default()
                    },
                    ..default()
                },
                Position { x: 3, y: 3 },
                Size { side: 0.8 },
                SnakeHead {
                    direction: Direction::Up,
                },
                SnakeSegment,
            ))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut snake: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = snake.iter_mut().next() {
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => head_pos.x -= 1,
            Direction::Right => head_pos.x += 1,
            Direction::Up => head_pos.y += 1,
            Direction::Down => head_pos.y -= 1,
        };

        let body_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        body_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    };
}

fn snake_movement_input(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut SnakeHead>) {
    if let Some(mut snake) = query.iter_mut().next() {
        let dir: Direction =
            if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
                Direction::Up
            } else if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
                Direction::Down
            } else if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
                Direction::Right
            } else if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
                Direction::Left
            } else {
                snake.direction
            };
        if dir != snake.direction.opposite() {
            snake.direction = dir;
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    side: f32,
}

fn size_scailing(windows: Query<&Window>, mut query: Query<(&Size, &mut Transform)>) {
    let win = windows.single();
    for (sprite_size, mut transform) in &mut query {
        transform.scale = Vec3::new(
            sprite_size.side / ARENA_WIDTH as f32 * win.width(),
            sprite_size.side / ARENA_HEIGHT as f32 * win.height(),
            1.0,
        );
    }
}

fn position_translation(windows: Query<&Window>, mut query: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.single();
    for (pos, mut transform) in &mut query {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT),
            0.0,
        );
    }
}

#[derive(Component)]
struct Food;

fn food_spawner(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        },
        Position {
            x: (random::<f32>() * ARENA_WIDTH) as i32,
            y: (random::<f32>() * ARENA_HEIGHT) as i32,
        },
        Size { side: 0.5 },
        Food,
    ));
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Resource, Default)]
struct SnakeSegments(pub Vec<Entity>);

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_BODY_COLOR,
                    ..default()
                },
                ..default()
            },
            position,
            Size { side: 0.65 },
            SnakeSegment,
        ))
        .id()
}
