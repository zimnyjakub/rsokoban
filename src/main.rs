use bevy::prelude::*;

const ARENA_WIDTH: u32 = 20;
const ARENA_HEIGHT: u32 = 20;

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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 13,8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(Materials {
        sokoban_atlas: texture_atlas_handle.clone()
    })

}

#[derive(Component)]
struct Player;

fn movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Transform)>
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
    materials: Res<Materials>
) {
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: materials.sokoban_atlas.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: TextureAtlasSprite::new(1),
        ..Default::default()
    })
        .insert(Player)
        .insert(Size::square(0.9));

}


fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();

    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            // sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            // sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            sprite_size.width *20.,
            sprite_size.height *20.
        )
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
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(size_scaling)
        )
        .add_plugins(DefaultPlugins)
        .run()
}
