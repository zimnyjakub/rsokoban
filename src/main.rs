use bevy::prelude::*;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            title: "r_sokoban".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugin(DefaultPlugins)
        .run()
}