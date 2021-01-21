use bevy::{
    input::{keyboard::KeyCode, Input},
    asset::AssetServerSettings,
    prelude::*
};
use std::time::Duration;
use rand::Rng;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Static,
}

#[derive(Debug)]
enum PappleType {
    // wesh wesh, canne à…
    Peach,
    Pear, // …du
    Grapes, // Pain grappe
    YesItIsTrue, // AKA "pomme de pin"
}

struct Snake(f32, Direction, Vec<(u32, u32, Entity)>);
struct Papple(PappleType, u32, u32);

struct SnakeTimer(Timer);
struct Score(u32);
struct Tail((u32, u32));


const SIZE: u32 = 10;
const TILE_SIZE: u32 = 50;

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let body = asset_server.load("serpent.png");
    let center = (TILE_SIZE * SIZE) / 2;
    
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(center as f32, center as f32, 0.),
                ..Default::default()
            },
            ..Default::default()
        });
    commands.spawn(CameraUiBundle::default());
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new((SIZE / 2) as f32, 10., 0.),
                ..Default::default()
            },
            text: Text {
                value: "0 point".to_string(),
                font: asset_server.load("Blazed.ttf"),
                style: TextStyle {
                    font_size: 50.0,
                    color: Color::RED,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    commands
        .spawn(SpriteBundle {
            material: materials.add(body.into()),
            transform: Transform {
                translation: Vec3::new(250., 250., 0.),
                ..Default::default()
            },
            ..Default::default()
        });
    
    let curr_ent = commands.current_entity().unwrap();
    commands
        .with(Snake(TILE_SIZE as f32, Direction::Static, vec![
            (SIZE / 2, SIZE / 2, curr_ent)
        ]));
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let grass1 = asset_server.load("herbe1.png");
            let grass2 = asset_server.load("herbe2.png");
            commands
                .spawn(SpriteBundle {
                    material: materials.add((if (x + y) % 2 == 0 { grass1 } else { grass2 }).into()),
                    transform: Transform {
                        translation: Vec3::new((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                });
        }
    }
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

fn mov(mut trans: Query<(&mut Snake, &mut Transform)>, timer: Res<SnakeTimer>) {
    if timer.0.finished() {
        for (mut snake, mut tra) in trans.iter_mut() {
            match snake.1 {
                Direction::Up => {
                    tra.translation.y += snake.0;       // tail = snake.2.last, snake.2.last déplacé en premier
                    match snake.2.iter_mut().nth(0) {
                       Some(c) => c.1 += 1,
                       None => panic!("Head loss"),
                    }
                                      
                },
                Direction::Down => {
                    tra.translation.y -= snake.0;
                    match snake.2.iter_mut().nth(0) {
                        Some(c) => c.1 -= 1,
                        None => panic!("Head loss"),
                    }  
                },
                Direction::Right => {
                    tra.translation.x += snake.0;
                    match snake.2.iter_mut().nth(0) {
                        Some(c) => c.0 += 1,
                        None => panic!("Head loss"),
                    }
                },
                Direction::Left => {
                    tra.translation.x -= snake.0;
                    match snake.2.iter_mut().nth(0) {
                        Some(c) => c.0 -= 1,
                        None => panic!("Head loss"),
                    }
                },
                _ => {},
            }
        }
    }
}

fn collide(mut trans: Query<(&mut Snake, &Transform)>) {
    for (mut snake, tra) in trans.iter_mut() {
        if tra.translation.x < 0 as f32 || tra.translation.x > ((SIZE-1) * TILE_SIZE) as f32 || tra.translation.y < 0 as f32 || tra.translation.y > ((SIZE-1) * TILE_SIZE) as f32 {
            snake.1 = Direction::Static;
        }
    }
}

/* TOuDOuM

- CRONCHE
- bouger tout le snake et update tail (mov)
- redimensionner les fruits
- faire pivoter les fruits

*/

fn collide_papple(
    commands: &mut Commands,
    mut score: ResMut<Score>,
    mut snake: Query<&mut Snake>,
    papples: Query<(Entity, &Papple)>,
    tail: Res<Tail>,
    mut timer: ResMut<SnakeTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for mut snake in snake.iter_mut() {
        for (entity, papple) in papples.iter() {
            let head = snake.2.iter().nth(0).expect("on a perdu la tete");
            if papple.1 == head.0 && papple.2 == head.1 {// le if de la colision
                let body = asset_server.load("serpent.png");
                score.0 += 1;
                commands.despawn(entity);
                commands
                    .spawn(SpriteBundle {
                        material: materials.add(body.into()),
                        transform: Transform {
                            translation: Vec3::new(tail.0.0 as f32, tail.0.1 as f32, 0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                snake.2.push((tail.0.0, tail.0.1, commands.current_entity().unwrap()));
                let current_duration = timer.0.duration();
                timer.0.set_duration(current_duration * 0.95);
            }
        }
    }
}

fn spawn_body(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    tail: Res<Tail>,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            material: material,
            transform: Transform {
                translation: Vec3::new(tail.0.0 as f32, tail.0.1 as f32, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .current_entity()
        .unwrap()
}

fn spawn_papple(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    timer: Res<SnakeTimer>
) {
    if !timer.0.finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    let proba = rng.gen::<u32>() % 50;
    if proba < 4 {
        let (papple_type, filename) = match proba {
            0 => (PappleType::Peach, "peche.png"),
            1 => (PappleType::Pear, "poire.png"),
            2 => (PappleType::Grapes, "raisin.png"),
            3 => (PappleType::YesItIsTrue, "uicvrai.png"),
            _ => unreachable!("proba < 4, mais proba >= 4"),
        };
        let x = rng.gen::<u32>() % SIZE;
        let y = rng.gen::<u32>() % SIZE;
        let texture = asset_server.load(filename);
        commands
            .spawn(SpriteBundle {
                material: materials.add(texture.into()),
                transform: Transform {
                    translation: Vec3::new((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Papple(papple_type, x, y));
    }
}

fn snake_timer(time: Res<Time>, mut snake_timer: ResMut<SnakeTimer>) {
    snake_timer.0.tick(time.delta_seconds());
}

fn show_score(mut query: Query<&mut Text>, score: Res<Score>) {
    for mut text in query.iter_mut() {
        text.value = format!("{} points", score.0);
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

    #[cfg(target_arch = "wasm32")]
    let asset_folder = "/".to_string();

    #[cfg(not(target_arch = "wasm32"))]
    let asset_folder = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/").to_string();

    app.add_resource(AssetServerSettings {
        asset_folder,
    });
    app.add_resource(SnakeTimer(Timer::new(
        Duration::from_millis(250. as u64),
        true,
    )));
    app.add_resource(Score(0));
    app.add_resource(Tail(((TILE_SIZE*SIZE)/2-TILE_SIZE,(TILE_SIZE*SIZE)/2)));
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
    app.add_system(collide.system());
    app.add_system(snake_timer.system());
    app.add_system(spawn_papple.system());
    app.add_system(collide_papple.system());
    app.add_system(show_score.system());

    // on lance !
    app.run();
}
