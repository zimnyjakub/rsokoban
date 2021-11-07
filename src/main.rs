use bevy::ecs::event::Events;
use bevy::math::*;
use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::util::clamp;

mod util;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

struct Materials {
    sokoban_atlas: Handle<TextureAtlas>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("sokoban_tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 13, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(Materials {
        sokoban_atlas: texture_atlas_handle.clone(),
    });

    let window = windows.get_primary().unwrap();

    let grid = Grid {
        bounds: IVec2::new(ARENA_WIDTH as i32, ARENA_HEIGHT as i32),
        size: 64,
        base_world_pos: Vec3::new(
            50.0 - window.width() / 2.0,
            50.0 - window.height() / 2.0,
            0.0,
        ),
    };
    let mut walls: Vec<IVec2> = Vec::with_capacity(std::cmp::max(grid.bounds.x as usize, grid.bounds.y as usize));

    for x in 0..grid.bounds.x {
        walls.push(IVec2::new(x, 0));
        walls.push(IVec2::new(x, grid.bounds.y - 1));
    }
    for y in 1..grid.bounds.y {
        walls.push(IVec2::new(0, y));
        walls.push(IVec2::new(grid.bounds.x - 1, y));
    }

    commands.insert_resource(grid);
    //todo: extract this to file load and support different levels
    commands.insert_resource(Level {
        wall_locations: walls,
        pushable_locations: vec![IVec2::new(2, 2), IVec2::new(3, 3), IVec2::new(3, 4)],
        goal_locations: vec![IVec2::new(1, 4)],
    })
}

fn window_resize(mut events: EventReader<WindowResized>, mut commands: Commands) {
    for event in events.iter() {
        commands.insert_resource(Grid {
            bounds: IVec2::new(ARENA_WIDTH as i32, ARENA_HEIGHT as i32),
            size: 64,
            base_world_pos: Vec3::new(
                50.0 - event.width / 2.0,
                50.0 - event.height / 2.0,
                0.0,
            ),
        });
    }
}

#[derive(Component, Debug)]
struct Player;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct Pushable;

#[derive(Component, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct IntendedPosition {
    x: i32,
    y: i32,
}

struct PlayerMovedEvent;

struct Grid {
    bounds: IVec2,

    size: i32,
    base_world_pos: Vec3,
}

struct Level {
    wall_locations: Vec<IVec2>,
    pushable_locations: Vec<IVec2>,
    goal_locations: Vec<IVec2>,
}

fn movement_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player: Query<(Entity, &Position), With<Player>>,
) {
    for (entity, pos) in player.iter() {
        if keyboard_input.just_pressed(KeyCode::Left) {
            commands.entity(entity).insert(IntendedPosition { x: pos.x - 1, y: pos.y });
        } else if keyboard_input.just_pressed(KeyCode::Up) {
            commands.entity(entity).insert(IntendedPosition { x: pos.x, y: pos.y + 1 });
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            commands.entity(entity).insert(IntendedPosition { x: pos.x + 1, y: pos.y });
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            commands.entity(entity).insert(IntendedPosition { x: pos.x, y: pos.y - 1 });
        }
    }
}

fn move_pushables(
    mut commands: Commands,
    mut wants_to_move: Query<(Entity, &IntendedPosition, &mut Position, &Pushable), Without<Obstacle>>,
    obstacles: Query<&Position, Or<(With<Obstacle>, With<Pushable>)>>,
    mut move_reader: EventReader<PlayerMovedEvent>,
) {
    if move_reader.iter().next().is_some() {
        for (ent, int_pos, mut pos, _) in wants_to_move.iter_mut() {
            if !obstacles.iter().any(|wall| int_pos.x == wall.x && int_pos.y == wall.y) {
                pos.x = int_pos.x;
                pos.y = int_pos.y;
            } else {
                println!("pushable: wall collided, not moving");
            }

            commands.entity(ent).remove::<IntendedPosition>();
        }
    }
}

fn move_player(
    mut commands: Commands,
    mut wants_to_move: Query<(Entity, &IntendedPosition, &mut Position, &Player), Without<Obstacle>>,
    obstacles: Query<&Position, Or<(With<Obstacle>, With<Pushable>)>>,
    mut move_writer: EventWriter<PlayerMovedEvent>,
) {
    for (ent, int_pos, mut pos, _) in wants_to_move.iter_mut() {
        if !obstacles.iter().any(|wall| int_pos.x == wall.x && int_pos.y == wall.y) {
            pos.x = int_pos.x;
            pos.y = int_pos.y;
        } else {
            //todo bug here
            println!("PLAYER wall collided, not moving");
        }

        move_writer.send(PlayerMovedEvent);
        commands.entity(ent).remove::<IntendedPosition>();
    }
}

fn check_pushable(
    mut commands: Commands,
    wants_to_move: Query<(&IntendedPosition, &Position), (Without<Pushable>, With<Player>)>,
    pushables: Query<(Entity, &Position), With<Pushable>>,
) {
    for (int_pos, pos) in wants_to_move.iter() {
        let pushable = pushables.iter().find(|(entity, pushable)| int_pos.x == pushable.x && int_pos.y == pushable.y);
        if let Some((ent, pushable)) = pushable {
            commands.entity(ent).insert(IntendedPosition {
                x: pushable.x + (pushable.x - pos.x),
                y: pushable.y + (pushable.y - pos.y),
            });
        }
    }
}


fn init_player(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: materials.sokoban_atlas.clone(),
            sprite: TextureAtlasSprite::new(52),
            ..Default::default()
        })
        .insert(Player)
        .insert(Position { x: 1, y: 1 });
}

fn init_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>,
) {
    for x in 0..grid.bounds.x {
        for y in 0..grid.bounds.y {
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgba(0.7, 0.7, 0.7, 0.3).into()),
                sprite: Sprite::new(Vec2::new((grid.size - 2) as f32, (grid.size - 2) as f32)),
                transform: Transform::from_translation(
                    grid.base_world_pos
                        + Vec3::new((x * grid.size) as f32, (y * grid.size) as f32, 0.),
                ),
                ..Default::default()
            });
        }
    }
}

fn snap_position_to_grid(mut q: Query<(&mut Transform, &Position)>, grid: Res<Grid>) {
    for (mut transform, position) in q.iter_mut() {
        transform.translation = grid.base_world_pos
            + Vec3::new(
            (position.x * grid.size) as f32,
            (position.y * grid.size) as f32,
            0.,
        );
    }
}

fn init_level(
    mut commands: Commands,
    materials: Res<Materials>,
    level: Res<Level>,
) {
    for wall in &level.wall_locations {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: materials.sokoban_atlas.clone(),
            sprite: TextureAtlasSprite::new(97),
            ..Default::default()
        })
            .insert(Obstacle)
            .insert(Position { x: wall.x, y: wall.y });
    }
    for pushable in &level.pushable_locations {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: materials.sokoban_atlas.clone(),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
            .insert(Pushable)
            .insert(Position { x: pushable.x, y: pushable.y });
    }
    for goal in &level.goal_locations {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: materials.sokoban_atlas.clone(),
            sprite: TextureAtlasSprite::new(3 * 24 + 2),
            ..Default::default()
        })
            .insert(Goal)
            .insert(Position { x: goal.x, y: goal.y });
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SokobanStages {
    Input,
    MovementPushable,
    MovementPlayer,
    PushableDetection,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "r_sokoban".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_event::<PlayerMovedEvent>()
        .add_startup_system(setup)
        .add_startup_stage("init_player", SystemStage::single(init_player))
        .add_startup_stage("init_grid", SystemStage::single(init_grid))
        .add_startup_stage("init_level", SystemStage::single(init_level))
        .add_startup_system(setup)
        .add_system(movement_input.label(SokobanStages::Input))
        .add_system(check_pushable.label(SokobanStages::PushableDetection))
        .add_system(move_pushables.label(SokobanStages::MovementPushable))
        .add_system(move_player.label(SokobanStages::MovementPlayer))
        .add_system_to_stage(CoreStage::PostUpdate, snap_position_to_grid)
        .add_system(window_resize)
        .add_plugins(DefaultPlugins)
        .run()
}
