use bevy::prelude::*;

static GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;
static STEP_SIZE: f32 = 1.0/160.0;

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct Acceleration(Vec3);

#[derive(Component)]
pub struct Mass(f32);


//I may make a dedicated planet spawning function later, maybe a function that works with console commands so it can be done runtime.

pub fn planet_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let sphere = Sphere::new(1.0);

    let sphere_mesh = meshes.add(sphere.mesh().uv(32, 18));

    commands.spawn((
        Mesh3d(meshes.add(sphere.clone())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("#D97706").unwrap().into(),
            metallic: 1.0,
            perceptual_roughness: 0.0,
            ..StandardMaterial::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Movable,
        Velocity(Vec3::X),
        Acceleration(Vec3::ZERO),
        Mass(1.0e11),
    ));

    commands.spawn((
        Mesh3d(meshes.add(sphere.clone())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("#ffd891").unwrap().into(),
            metallic: 1.0,
            perceptual_roughness: 0.0,
            ..StandardMaterial::default()
        })),
    Transform::from_xyz(8.0, 0.0, 0.0),
    //Visibility::default(),
    Movable,
    Velocity(Vec3::from([-1.0, 1.0, 1.0])),
    Acceleration(Vec3::ZERO),
    Mass(1.0e11),
    ));
}

//Just doing basic Euler's Method here as a proof of
//concept, so I'm not concerned too much with accuracy
//as I will be replacing this with a symplectic integrator.

/*
I collect all the data from every planet entity, store it in a seperate vector, run operations on that vector in order to implement the numerical method,
and imprint the results back onto the entities. This is necessary because the structuring of the queries can't be duplicated and rust's borrow checker prevents
a mutable and immutable borrow from occuring at the same time. Queries don't support indexing either which is why this structure is necessary.
*/

struct PlanetData
{
    //entity: Entity, - not includig this because I don't need to worry about the order of entities in the query changing up in the same frame.
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    mass: f32,
}

pub fn planet_update(mut query: Query<(&mut Transform, &mut Velocity, &mut Acceleration, &Mass), With<Movable>>) {

    let mut planet_states: Vec<PlanetData> = query.iter().map(|(p, v, a, m)| PlanetData{position: p.translation, velocity: v.0, acceleration: a.0, mass: m.0 }).collect();

    planet_states = calculate_forces(planet_states); // might cause borrowing issues will fix momentarily.

    for (i, (mut transform, mut velocity, mut acceleration, _)) in query.iter_mut().enumerate() {
        *velocity = Velocity(planet_states[i].velocity);
        *acceleration = Acceleration(planet_states[i].acceleration);
        transform.translation += velocity.0 * STEP_SIZE + 0.5 * acceleration.0 * STEP_SIZE.powi(2);
    }

}

fn calculate_forces(mut planet_states: Vec<PlanetData>) -> Vec<PlanetData>{
            for i in 0..planet_states.len()
            {
                for j in 0..planet_states.len()
                {
                    if i != j {

                        let distance = planet_states[j].position - planet_states[i].position;
                        let force = GRAVITATIONAL_CONSTANT * (planet_states[i].mass * planet_states[j].mass)/(distance.length_squared());
                        planet_states[i].acceleration = distance.normalize() * force/planet_states[i].mass;
                        planet_states[i].velocity = planet_states[i].velocity + STEP_SIZE * planet_states[i].acceleration;
                    }
                }
            }
            return planet_states;
    }

/*
The below code creates an ephemeris. This will eventually be modified
to use chebyshev polynomials instead of interpolation between discrete
points in time. I'm not yet sure if I'll be using a predetermined quantity
of steps for the calculations, if I do I'll add it to the "universe_bus",
current name for the entity that indexes the ephemeris for the display and holds
the number of bodies being simulated. need to look into chrono crate.
*/

// Consider pre-allocation for speed, may be worth it. Can easily get a maximum
// size estimate by using bottom limit for adaptive step.
struct EphemerisEntry {
    t: f64,
    states: [PlanetData; 9], // must be changed whenever more bodies are added.
}

fn create_discrete_ephemeris(mut init_planet_states: Vec<PlanetData>) -> () {
    
}

    // For play-back capabilities and trajectory optimization if
    // I reach that point I'll have to implement an ephemeris.
    // I should also potentially compare my ephemerides to JPL's
    // to illustrate differences in accuracies.
    // I will use JPL's method of translating state data into 
    // a chebyshev series for every planet, I will decide on
    // the contiguous time divisions after tinkering more with
    // wisdom.