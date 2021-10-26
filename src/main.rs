use bevy::math::*;
use bevy::prelude::*;

use crate::util::clamp;

mod util;

const ARENA_WIDTH: u32 = 4;
const ARENA_HEIGHT: u32 = 4;

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

    commands.insert_resource(Grid {
        bounds: IVec2::new(ARENA_WIDTH as i32, ARENA_HEIGHT as i32),
        size: 64,
        base_world_pos: Vec3::new(
            50.0 - window.width() / 2.0,
            50.0 - window.height() / 2.0,
            0.0,
        ),
    });
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

struct Grid {
    bounds: IVec2,

    size: i32,
    base_world_pos: Vec3,
}

fn movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut Position, With<Player>>,
    grid: Res<Grid>,
) {
    let mut position = players.single_mut();

    if keyboard_input.just_pressed(KeyCode::Left) {
        position.x = clamp(position.x - 1, 0, grid.bounds.x as i32 - 1);
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        position.y = clamp(position.y + 1, 0, grid.bounds.y as i32 - 1);
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        position.x = clamp(position.x + 1, 0, grid.bounds.x as i32 - 1);
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        position.y = clamp(position.y - 1, 0, grid.bounds.y as i32 - 1);
    }

}

fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: materials.sokoban_atlas.clone(),
            sprite: TextureAtlasSprite::new(52),
            ..Default::default()
        })
        .insert(Player)
        .insert(Position { x: 0, y: 0 });
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

fn snap_player_to_grid(mut players: Query<(&mut Transform, &Position)>, grid: Res<Grid>) {
    let (mut transform, position) = players.single_mut();

    transform.translation = grid.base_world_pos
        + Vec3::new(
            (position.x * grid.size) as f32,
            (position.y * grid.size) as f32,
            0.,
        )
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "r_sokoban".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_startup_stage("player_spawn", SystemStage::single(spawn_player))
        .add_startup_stage("grid_init", SystemStage::single(init_grid))
        .add_startup_system(setup)
        .add_system(movement_input)
        .add_system_to_stage(CoreStage::PostUpdate, snap_player_to_grid)
        .add_plugins(DefaultPlugins)
        .run()
}
