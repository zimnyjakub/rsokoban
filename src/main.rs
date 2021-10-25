use bevy::prelude::*;

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
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("sokoban_tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 13, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(Materials {
        sokoban_atlas: texture_atlas_handle.clone()
    })
}

#[derive(Component)]
struct Player;

fn movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Transform)>,
) {
    if let Some((_player, mut transform)) = players.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        } else if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        } else if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        } else if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    materials: Res<Materials>,
) {
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: materials.sokoban_atlas.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: TextureAtlasSprite::new(52),
        ..Default::default()
    })
        .insert(Player);
}

fn init_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    //todo extract 64 as some kind of a constant
    //64 is the default size of a tile / cell
    for x in 0..ARENA_WIDTH {
        for y in 0..ARENA_HEIGHT {
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgba(0.7, 0.7, 0.7, 0.3).into()),
                sprite: Sprite::new(Vec2::new(62.0, 62.0)),
                transform: Transform::from_translation(Vec3::new(
                    ((x * 64) as f32 - window.width() /2.0)+50.0,
                    ((y * 64) as f32 - window.height()/2.0)+50.0,
                    0.0)),
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
        .add_startup_stage("game_start", SystemStage::single(spawn_player))
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup)
                .with_system(init_grid)
        )
        .add_plugins(DefaultPlugins)
        .run()
}
