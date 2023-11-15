use bevy::prelude::*;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl From<Direction> for KeyCode {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => KeyCode::W,
            Direction::Down => KeyCode::S,
            Direction::Left => KeyCode::A,
            Direction::Right => KeyCode::D,
        }
    }
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment {
    lifetime: usize,
}

#[derive(Component)]
struct Apple;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (create_board, spawn_player, spawn_initial_apple))
        .add_systems(Update, move_player)
        .run()
}

const BOARD_SIZE: usize = 40;
const PIXEL_SIZE: usize = 25;

const SNAKE_INITIAL_SIZE: usize = 1;

fn create_board(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands) {
    let snake_sprite: Sprite = Sprite {
        color: Color::MIDNIGHT_BLUE,
        ..default()
    };

    let snake_transform: Transform =
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(PIXEL_SIZE as f32));

    let snake_head: SnakeHead = SnakeHead {
        direction: Direction::Up,
    };


    commands.spawn((
        SpriteBundle {
            sprite: snake_sprite,
            transform: snake_transform,
            ..default()
        },
        snake_head,
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query_player: Query<&mut SnakeHead>,
    mut query_position: Query<&mut Transform, With<SnakeHead>>,
) {
    let mut snake_head = query_player.iter_mut().next().unwrap();
    let mut player_position = query_position.iter_mut().next().unwrap();

    if keys.pressed(Direction::Up.into()) && snake_head.direction != Direction::Down {
        snake_head.direction = Direction::Up;
    } else if keys.pressed(Direction::Down.into()) && snake_head.direction != Direction::Up {
        snake_head.direction = Direction::Down;
    } else if keys.pressed(Direction::Left.into()) && snake_head.direction != Direction::Right {
        snake_head.direction = Direction::Left;
    } else if keys.pressed(Direction::Right.into()) && snake_head.direction != Direction::Left {
        snake_head.direction = Direction::Right;
    }

    match snake_head.direction {
        Direction::Up => player_position.translation.y += 0.2 * time.delta().as_millis() as f32,
        Direction::Down => player_position.translation.y -= 0.2 * time.delta().as_millis() as f32,
        Direction::Left => player_position.translation.x -= 0.2 * time.delta().as_millis() as f32,
        Direction::Right => player_position.translation.x += 0.2 * time.delta().as_millis() as f32,
    };
}

fn spawn_initial_apple(mut commands: Commands) {}
