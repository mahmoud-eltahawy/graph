use bevy::{color::palettes::css::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (graph, xyz))
        .run();
}

#[derive(Component)]
struct Camera;

#[derive(Resource)]
struct WindowWidth(f32);

fn setup(mut commands: Commands, windows: Query<&mut Window>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10., -100., 10.).looking_at(Vec3::ZERO, Vec3::Z),
        Camera,
    ));
    let width = windows
        .iter()
        .map(|x| x.resolution.width())
        .reduce(|w1, w2| w1.max(w2))
        .unwrap();
    commands.insert_resource(WindowWidth(width));
}

fn points(width: f32, fun: fn(f32) -> f32) -> Vec<Vec3> {
    let range = (width / 25.).floor() as i32;
    (-range..=range)
        .flat_map(|x| {
            let x = x as f32;
            (1..width.ceil() as i32)
                .map(|extra| x + (extra as f32 / width))
                .collect::<Vec<_>>()
        })
        .map(|x| Vec3::new(x, 0., fun(x)))
        .collect()
}

fn test_fn(x: f32) -> f32 {
    x * x + x.sin() * x.sin() - x.cos() * x.cos()
}

fn graph(mut gizmos: Gizmos, width: Res<WindowWidth>) {
    let width = width.0;
    gizmos.linestrip(points(width, test_fn), PURPLE);
}
fn xyz(mut gizmos: Gizmos, width: Res<WindowWidth>) {
    let width = width.0;
    gizmos.line(Vec3::X * (-width), Vec3::X * width, WHITE);
    gizmos.line(Vec3::Y * (-width), Vec3::Y * width, WHITE);
    gizmos.line(Vec3::Z * (-width), Vec3::Z * width, WHITE);
}
