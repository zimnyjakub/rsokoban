use bevy::math::*;
use bevy::prelude::*;

use crate::util::clamp;

mod util;

const ARENA_WIDTH: u32 = 4;
const ARENA_HEIGHT: u32 = 4;

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

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
        sokoban_atlas: texture_atlas_handle.clone()
    });

    let window = windows.get_primary().unwrap();

    commands.insert_resource(Grid {
        bounds: UVec2::new(4, 4),
        grid_size: 64,
        base_world_pos: Vec3::new(
            50.0 - window.width() / 2.0,
            50.0 - window.height() / 2.0,
            0.0),
    });
}

#[derive(Component)]
struct Player {
    pos: UVec2,
}

struct Grid {
    bounds: UVec2,

    grid_size: u32,
    base_world_pos: Vec3,
}

fn movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut Player>,
) {
    if let Some(mut player) = players.iter_mut().next() {
        if keyboard_input.just_pressed(KeyCode::Left) {
            player.pos.x = clamp(player.pos.x - 1, 0, ARENA_WIDTH);
        } else if keyboard_input.just_pressed(KeyCode::Up) {
            player.pos.y = clamp(player.pos.y + 1, 0, ARENA_HEIGHT);
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            player.pos.x = clamp(player.pos.x + 1, 0, ARENA_WIDTH);
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            player.pos.y = clamp(player.pos.y - 1, 0, ARENA_HEIGHT);
        }

        println!("current pos x {} y {}", player.pos.x, player.pos.y)
    }
}

fn spawn_player(
    mut commands: Commands,
    materials: Res<Materials>,
) {
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: materials.sokoban_atlas.clone(),
        sprite: TextureAtlasSprite::new(52),
        ..Default::default()
    })
        .insert(Player { pos: UVec2::new(0, 0) });
}

fn init_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>,
) {
    //todo extract 64 as some kind of a constant
    //64 is the default size of a tile / cell
    for x in 0..grid.bounds.x {
        for y in 0..grid.bounds.y {
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgba(0.7, 0.7, 0.7, 0.3).into()),
                sprite: Sprite::new(Vec2::new((grid.grid_size - 2) as f32, (grid.grid_size - 2) as f32)),
                transform: Transform::from_translation(
                    grid.base_world_pos +
                        Vec3::new((x * grid.grid_size) as f32, (y * grid.grid_size) as f32, 0.)
                ),
                ..Default::default()
            }).insert(Size::square(0.9));
        }
    }
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
        .add_system(movement_input)
        .add_startup_stage("player_spawn", SystemStage::single(spawn_player))
        .add_startup_stage("grid_init", SystemStage::single(init_grid))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .run()
}
