mod planet_logic;
mod user_interface;
use bevy_fps_counter::{FpsCounter, FpsCounterPlugin};
use planet_logic::*;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

//Going to use the Wisdom-Holman integrator for the N-body dynamics
//Currently will also use it for the craft, but I might implement
//a hybrid method if local accuracy proves to be important.
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(FpsCounterPlugin)
    .add_systems(Startup, setup)
    .add_systems(Startup, planet_setup)
    .add_systems(Update, planet_update)
    .run();
}

fn setup(
    mut commands: Commands, mut counter_state: ResMut<FpsCounter>
) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        PanOrbitCamera::default(),
    )); 

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            range: 10.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        Visibility::default(),
    ));

    counter_state.enable();

}