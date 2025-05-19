use bevy::prelude::*;
use bevy::math::prelude::*;

static GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
static STEP_SIZE: f64 = 1.0/160.0;


#[derive(Component)]
pub struct Mass(f64);

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct Movable;

pub fn planet_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let sphere = Sphere::new(1.0);

    let sphere_mesh = meshes.add(sphere.mesh().uv(32, 18));

    commands.spawn((
        PbrBundle {
        mesh: sphere_mesh.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.9),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()},
    Movable,
    Velocity(Vec3::ZERO),
    ));

    commands.spawn((
        PbrBundle {
        mesh: sphere_mesh,
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.3, 0.9),
            ..default()
        }),
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        ..default()},
    Movable,
    Velocity(Vec3::ZERO)
    ));
}

//Just doing basic Euler's Method here as a proof of
//concept, so I'm not concerned too much with accuracy
//as I will be replacing this with a symplectic integrator.

pub fn planet_update(mut query:
    Query<&mut Transform,
    Entity,
    Velocity,
    Mass,
    With<Movable>>) {
    for (mut transform, this_entity, this_velocity, this_mass) in query.iter_mut() {
        transform.translation.x += 0.1;
        transform.translation.y += 0.1;
    }
}