use bevy::prelude::*;
use bevy::math::prelude::*;

static GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;
static STEP_SIZE: f32 = 1.0/160.0;


#[derive(Component)]
pub struct Mass(f32);

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct Acceleration(Vec3);

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Focused; //This component exists purely to comply
//with borrowing rules enforced by bevy and rust. I can't safely
//access a query from two points at the same time unless I use
//filters that help me break up the query into disjoint subsets.

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

pub fn planet_update(mut parameters: ParamSet<(
    mut mutable_query:
    Query<(&mut Transform,
    Entity,
    &mut Velocity,
    &mut Acceleration,
    &Mass),
    With<Movable>>,
    Query<(&Transform,
    Entity,
    &Velocity, // should probably not be including these velocities.
    &Acceleration,
    &Mass),
    With<Movable>,
    >)
    ) {

        for (this_transform, 
            this_entity, 
            this_velocity, 
            this_acceleration
            this_mass
        ) in parameters.p0().iter_mut()
        {
            for (other_transform,
                other_entity,
                other_velocity,
                other_acceleration,
                other_mass
            ) in parameters.p1().iter()
            {
                let distance = other_transform.translation - this_transform.translation;
                let force = GRAVITATIONAL_CONSTANT * (this_mass.0 * other_mass.0)/(distance.length_squared());
                *this_acceleration = Acceleration(distance.normalize() * force/ this_mass.0);
                //I know multiplying and dividing by mass is redundant, just keeping it for clarity.
                *this_velocity = Velocity(this_velocity.0 + STEP_SIZE * this_acceleration.0);
            }
        }

    //after the nested loop velocity update I do the 
    //position update:
    for (mut transform, _entity, velocity, acceleration, _mass) in mutable_query.iter_mut()
    {
        transform.translation += velocity.0*STEP_SIZE + 0.5 * acceleration.0 * STEP_SIZE.powi(2);
    }
}

fn calculate_forces(
    ) {
        for (other_transform,
            other_entity,
            _other_velocity,
            _other_acceleration,
            other_mass) in immutable_query.iter()
                {
                    let distance = other_transform.translation - this_transform.translation;
                    let force = GRAVITATIONAL_CONSTANT * (this_mass.0 * other_mass.0)/(distance.length_squared());
                    *this_acceleration = Acceleration(distance.normalize() * force/ this_mass.0);
                    //I know multiplying and dividing by mass is redundant, just keeping it for clarity.
                    *this_velocity = Velocity(this_velocity.0 + STEP_SIZE * this_acceleration.0);
                }
    }