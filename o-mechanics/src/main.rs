mod planet_logic;
use planet_logic::*;
use bevy::prelude::*;
use bevy::math::prelude::*;

//Going to use the Wisdom-Holman integrator for the N-body dynamics
//Currently will also use it for the craft, but I might implement
//a hybrid method if local accuracy proves to be important.
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Startup, planet_setup)
    .add_systems(Update, planet_update)
    .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0)
        .looking_at(Vec3::ZERO, Vec3::Y),..default()});

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}