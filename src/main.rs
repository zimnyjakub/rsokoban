use bevy::prelude::*;

const ARENA_WIDTH: u32 = 20;
const ARENA_HEIGHT: u32 = 20;

struct Materials {
    player_material: Handle<ColorMaterial>,
    floor_material: Handle<ColorMaterial>,
    wall_material: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        floor_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        wall_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
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

fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Player);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "r_sokoban".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(movement_input)
        .add_startup_stage("game_start", SystemStage::single(spawn_player))
        .add_plugins(DefaultPlugins)
        .run()
}
