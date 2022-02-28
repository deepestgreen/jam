use bevy::prelude::*;
//use bevy::input::keyboard::KeyboardInput;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::gltf::Gltf;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    GameInit,
    Playing,
    Paused,
    PauseMenu,
}

//TODO:Is This necessary? Let's investigate when we have the game logic going.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum PauseState {
    Pausing,
    Paused,
    Unpausing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum CursorState {
    Selecting,
    Rotating,
    Outside,
}

enum ClickEvent {
    Select,
    Move,
}

enum CameraMoveEvent {
    Pan{ x: f32, z:f32 },
    Rotate{x: f32, y: f32},
    Zoom{inn: f32, out:f32},
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Playing)
        .add_state(CursorState::Selecting)
        .add_event::<ClickEvent>()
        .add_event::<CameraMoveEvent>()
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(add_people)
        .add_startup_system(load_ship)
        .add_startup_system(load_system.label("load"))
        .add_startup_system(load_camera)
        .add_startup_system(load_light)
        .add_startup_system(add_planets)
        .add_startup_system(start_init)
        .add_system_set(
            SystemSet::on_enter(AppState::GameInit)
                .with_system(spawn_system)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::PauseMenu)
                .with_system(open_pause_menu)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::PauseMenu)
                .with_system(close_pause_menu)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Paused)
                .with_system(pause_game)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Paused)
                .with_system(unpause_game)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(game_tick)
                //.with_system(animate_light_direction)
                .with_system(check_mouse_scroll)
        )
        .add_system_set(
            SystemSet::on_update(CursorState::Selecting)
                .with_system(absolute_mouse_move)
                .with_system(check_mouse_exit)
        )
        .add_system_set(
            SystemSet::on_update(CursorState::Rotating)
                .with_system(relative_mouse_move)
                //.with_system(check_mouse_exit)
        )
        .add_system_set(
            SystemSet::on_enter(CursorState::Rotating)
                .with_system(start_rotate)
        )
        .add_system_set(
            SystemSet::on_exit(CursorState::Rotating)
                .with_system(stop_rotate)
        )
        .add_system_set(
            SystemSet::on_update(CursorState::Outside)
                .with_system(check_mouse_enter)
        )
        .add_system(check_key_input)
        .add_system(check_mouse_buttons)
        .add_system(check_mouse_position)
        .add_system(move_camera)
        .run();

}

fn check_key_input(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut cam_evs: EventWriter<CameraMoveEvent>,
    ){

    if keys.just_pressed(KeyCode::Escape) {
        //Open the pause menu.
        match app_state.current() {
            AppState::Playing => 
                app_state.set(AppState::PauseMenu).unwrap(),
            AppState::PauseMenu => 
                app_state.set(AppState::Playing).unwrap(),
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        //Pause the game.
        match app_state.current() {
            AppState::Playing => 
                app_state.set(AppState::Paused).unwrap(),
            AppState::Paused => 
                app_state.set(AppState::Playing).unwrap(),
            _ => (),
        }
    }
    {
        let mut pan: (f32,f32) = (0.0,0.0);

        if keys.pressed(KeyCode::W) {
            //pan forward
            pan.1 -= 1.0;
        }
        if keys.pressed(KeyCode::S) {
            pan.1 += 1.0;
        }
        if keys.pressed(KeyCode::A) {
            pan.0 -= 1.0;
        }
        if keys.pressed(KeyCode::D) {
            pan.0 += 1.0;
        }

        if pan.0 != 0.0 || pan.1 != 0.0 {
            cam_evs.send(CameraMoveEvent::Pan{ x:pan.0, z:pan.1 });
        }
    }
}

use bevy::app::AppExit;

fn open_pause_menu(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

fn close_pause_menu() {

}

    //Pause the game.
fn pause_game() {
    //Stop the timer

}

fn unpause_game() {
    //Start the timer
}

fn check_mouse_buttons(
    buttons: Res<Input<MouseButton>>,
    mut cursor_state: ResMut<State<CursorState>>,
    mut ev: EventWriter<ClickEvent>
) {
    if buttons.just_pressed(MouseButton::Middle) {
        match cursor_state.current() {
            CursorState::Selecting => {
                cursor_state.set(CursorState::Rotating).unwrap();
                println!("Start Rotating");
            }
            _ => (),
        }
    } else if buttons.just_released(MouseButton::Middle) {
        match cursor_state.current() {
            CursorState::Rotating => {
                cursor_state.set(CursorState::Selecting).unwrap();
                println!("Stop Rotating");
            }
            _ => (),
        }
    }
    if buttons.just_pressed(MouseButton::Left) {
        ev.send(ClickEvent::Select);
    }
    if buttons.just_pressed(MouseButton::Right) {
        ev.send(ClickEvent::Move)
    }
        
}

use bevy::input::mouse::MouseWheel;
fn check_mouse_scroll(
    mut evs: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evs.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
}

//Do anything that needs to happen when you start rotating
fn start_rotate(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    //Hide the cursor

}

//Do anything that needs to happen when you stop rotating
fn stop_rotate(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(true);
    //Show the cursor

}

fn check_mouse_enter( 
    mut evs: EventReader<CursorEntered>,
    mut cursor_state: ResMut<State<CursorState>>,
) {
    for ev in evs.iter() {
        match cursor_state.current() {
            CursorState::Outside => {
                cursor_state.set(CursorState::Selecting).unwrap();
                println!("Entered!");
            }
            _ => {
                println!("Don't enter")
            }
        }
    }
}

fn check_mouse_exit( 
    mut evs: EventReader<CursorLeft>,
    mut cursor_state: ResMut<State<CursorState>>,
) {
    for ev in evs.iter() {
        cursor_state.set(CursorState::Outside).unwrap();
        println!("Exited!");
    }
}

fn absolute_mouse_move(
    mut evs: EventReader<CursorMoved>,
) {
    for ev in evs.iter() {
       /* println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.id
        );*/
    }
}



use bevy::input::mouse::MouseMotion;
fn relative_mouse_move(
    mut motion: EventReader<MouseMotion>,
    mut ev_out: EventWriter<CameraMoveEvent>,
) {
    for ev in motion.iter() {
        //println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        ev_out.send(CameraMoveEvent::Rotate{
            x: ev.delta.x, y: ev.delta.y
        });
    }
}


fn check_mouse_position(win: Res<Windows>) {
    let window = win.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        //Cursor in window.
    } else {
        //Cursor not in window.
    }
}

fn start_init(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::GameInit).unwrap();
    println!("STARTING INIT");
}

fn add_planets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mesh = asset_server.load("planet.glb#Scene0");
    let mesh2 = mesh.clone();

    commands.spawn_bundle((
        Transform::from_xyz(-5.0,1.0,0.0)
            .with_scale(Vec3::new(2.0,2.0,2.0)),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(mesh);
    });

    
    commands.spawn_bundle((
        Transform::from_xyz(5.0,1.0,0.0)
            .with_scale(Vec3::new(2.0,2.0,2.0)),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(mesh2);
    });
   

    /*commands.spawn_bundle( PlanetBundle {
        name: Name("IAmAPlanet".to_string()),
        trans: Transform::from_xyz(0.0,0.0,0.0),
        pop: Population(10),
        _p: Planet,
    });*/

}

fn load_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mesh = asset_server.load("ship.glb#Scene0");
    commands.spawn_bundle((
        Ship, Speed(Vec3::new(1.0,0.0,0.0)),
        Transform::from_xyz(0.0,0.0,0.0)
            .with_scale(Vec3::new(0.2,0.2,0.2))
            .with_rotation(Quat::from_euler(EulerRot::XYZ,0.0,1.571,0.0)),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(mesh);
    });
}

#[derive(Default)]
struct AssetPack(Handle<Gltf>);

fn load_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("points.gltf");
    commands.insert_resource(AssetPack(gltf));
    println!("LOADING LEVEL ASSET");
    //commands.spawn_scene(asset_server.load("points.gltf#Scene/Node0"));
}

fn spawn_system(mut c: Commands, ap: Res<AssetPack>, 
                assets_gltf: Res<Assets<Gltf>>,
                mut app_state: ResMut<State<AppState>>) {
    if let Some(gltf) = assets_gltf.get(&ap.0) {
        c.spawn_scene(gltf.scenes[0].clone());
        c.spawn_bundle((
            Transform::from_xyz(1.0, 2.0, 3.0),
            GlobalTransform::identity(),
        )).with_children(|parent| {
            parent.spawn_scene(gltf.named_scenes["geo0"].clone());
        });
    }
    println!("SPAWNING LEVEL");

    app_state.set(AppState::Playing).unwrap();
}


fn load_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 20.0, 20.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn move_camera(
    mut evs: EventReader<CameraMoveEvent>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut cam_trans = query.single_mut();

    //Go through rotate events and put them into effect for the camera.
    //TODO: Add up the rotation events to a single rotation
    let delta = 0.5;
    for ev in evs.iter() {
        match ev {
            CameraMoveEvent::Pan{x,z} => 
                cam_trans.translation += Vec3::new(x*delta,0.0, z*delta),
            CameraMoveEvent::Rotate{x, y} => 
                cam_trans.rotation *= Quat::from_axis_angle(Vec3::Y, x*0.017),

            CameraMoveEvent::Zoom{inn, out} => (),
        }
    }
}

fn load_light(mut commands: Commands) {
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

fn make_refugee_ship(mut c: Commands, trans: Transform) -> Entity{
    c.spawn()
        .insert_bundle(ShipBundle {
            trans: trans,
        })
        .insert(Refugee)
        .id()
}

//Triggered when a refugee wave comes from a planet. 
//Generates the refugee ship and alters the required state.
fn flee_from_planet() {
    
}


//Updates state that needs to be per-frame.
fn game_tick(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Speed), With<Ship>>,
) {
    let delta: f32 = time.delta_seconds();
    for (mut transform, mut speed) in query.iter_mut() {
        if transform.translation.to_array()[0] > 5.0 ||
            transform.translation.to_array()[0] < -5.0 {
       // if transform.translation.cmpgt(right_line).any() {
            println!("ABOUT FACE!");
            transform.rotation *= Quat::from_euler(EulerRot::XYZ,0.0,3.142,0.0);
            speed.0 *= -1.0;
        }
        transform.translation += speed.0 * delta;
        //println!("moved by {}", speed.0 * delta);
        let right_line: Vec3 = Vec3::new(5.0,-10.0,-10.0);
    }
}

//Updates refugee ship position per-frame.
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
    trans: Transform,
}

#[derive(Component)]
struct Ship;

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
