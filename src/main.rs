use bevy::prelude::*;
//use bevy::input::keyboard::KeyboardInput;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(add_people)
        .add_startup_system(load_mesh)
        .add_startup_system(add_planets)
        .add_system(animate_light_direction)
        //.add_system(orbit_tick)
        .run();
}

fn add_planets(mut commands: Commands) {
    commands.spawn_bundle( PlanetBundle {
        name: Name("IAmAPlanet".to_string()),
        trans: Transform::from_xyz(0.0,0.0,0.0),
        pop: Population(10),
        _p: Planet,
    });

    commands.spawn_bundle( PlanetBundle {
        name: Name("IAmAMoon".to_string()),
        trans: Transform::from_xyz(10.0,10.0,0.0),
        pop: Population(10),
        _p: Planet,
    });
}
#[allow(unused)]
fn load_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("ship.gltf#Scene0"));
}


fn load_mesh(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("ship.glb#Scene0"));
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(40.0, 0.0, 40.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

fn make_refugee_ship(mut c: Commands) {

}

fn flee_from_planet() {

}

fn game_tick() {

}

fn refugee_move(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<Refugee>>,
    ) {
    let delta: f32 = time.delta_seconds();
    for (mut transform, speed) in query.iter_mut() {
        transform.translation += speed.0 * delta;
        //println!("moved by {}", speed.0 * delta);
    }
}
        
#[derive(Component)]
struct Refugee;

#[derive(Bundle)]
struct AsteroidBundle {
    name: Name,
    mass: Mass,
    trans: Transform,
    speed: Speed,
    _p: Asteroid,
}

#[derive(Component)]
struct Asteroid;

#[derive(Bundle)]
struct PlanetBundle {
    name: Name,
    trans: Transform,
    pop: Population,
    _p: Planet,
}

#[derive(Component)]
struct Population(u32);

#[derive(Bundle)]
struct ShipBundle {
    name: Name,
    mass: Mass,
    trans: Transform,
}

#[derive(Component)]
struct Boxes {

}

#[derive(Component)]
struct Speed(Vec3);

#[derive(Component)]
struct Person;

#[derive(Component)]
#[derive(Default)]
struct Planet;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Mass(f32);
