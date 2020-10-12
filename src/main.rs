use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*
};

#[derive(Clone, Copy)]
enum Direction{
    Up,
    Right,
    Down,
    Left,
    Static,
}

struct Snake(f32, Direction);
struct Papple;

fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("/home/mathis/Images/1504784576-risitasv2-1.png").unwrap();
    commands
        .spawn((Papple,))
        .spawn(Camera2dComponents::default());
    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Snake(6., Direction::Static));
}

fn dir(keyboard_input: Res<Input<KeyCode>>, audio_output: Res<AudioOutput>,
    asset_server: Res<AssetServer>, mut snake: Query<&mut Snake>,) {
    for mut snake in &mut snake.iter() {
        if keyboard_input.just_pressed(KeyCode::Up){
            snake.1 = Direction::Up;
        }
        if keyboard_input.just_pressed(KeyCode::Down){
            snake.1 = Direction::Down;
        }
        if keyboard_input.just_pressed(KeyCode::Left){
            snake.1 = Direction::Left;
        }
        if keyboard_input.just_pressed(KeyCode::Right){
            snake.1 = Direction::Right;
        }
    }
}

fn mov(mut trans: Query<(&Snake, &mut Translation)>) {
    for (snake, mut tra) in &mut trans.iter() {
        match snake.1 {
            Direction::Up => *tra.0.y_mut() += snake.0,
            Direction::Down => *tra.0.y_mut() -= snake.0,
            Direction::Right => *tra.0.x_mut() += snake.0,
            Direction::Left => *tra.0.x_mut() -= snake.0,
            _ => {},
        }
    }   
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
        title: "Snake".to_string(),
        ..Default::default()})
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(mov.system())
        .add_system(dir.system())
        .run();
}
