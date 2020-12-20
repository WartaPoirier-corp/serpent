use bevy::{
    input::{keyboard::KeyCode, Input},
    asset::AssetServerSettings,
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

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("image.png");
    commands
        .spawn((Papple,))
        .spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Snake(6., Direction::Static));
}

fn dir(keyboard_input: Res<Input<KeyCode>>, mut snake: Query<&mut Snake>,) {
    for mut snake in snake.iter_mut() {
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

fn mov(mut trans: Query<(&Snake, &mut Transform)>) {
    for (snake, mut tra) in trans.iter_mut() {
        match snake.1 {
            Direction::Up => tra.translation.y += snake.0,
            Direction::Down => tra.translation.y -= snake.0,
            Direction::Right => tra.translation.x += snake.0,
            Direction::Left => tra.translation.x -= snake.0,
            _ => {},
        }
    }   
}

fn main() {
    // met en place le logger, pour avoir les messages de debug de Bevy entre autres
    pretty_env_logger::init();

    // crée une nouvelle "App" Bevy
    let mut app = App::build();
    // configure le titre de la fenêtre
    app.add_resource(WindowDescriptor {
        title: "Serpent".to_string(),
        ..Default::default()}
    );
    app.add_resource(AssetServerSettings {
        asset_folder: "/".to_string(),
    });
    // Ajoute les plugins de base
    app.add_plugins(DefaultPlugins);

    // Pour la version web (= WASM) uniquement : on ajoute
    // le plugin WebGL2 qui permet de faire un rendu 3D dans
    // un site web
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // on a ajoute les différents systèmes
    app.add_startup_system(setup.system());
    app.add_system(mov.system());
    app.add_system(dir.system());

    // on lance !
    app.run();
}
