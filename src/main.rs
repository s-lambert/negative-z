use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 2000.0,
            ..default()
        },
        ..default()
    });
}

fn show_rectangle(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.7, 0.7, 0.7),
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            translation: Vec3::new(-10.0, -10.0, -10.0),
            ..default()
        },
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Negative Z Values".to_string(),
                width: 500.0,
                height: 500.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(show_rectangle)
        .run();
}
