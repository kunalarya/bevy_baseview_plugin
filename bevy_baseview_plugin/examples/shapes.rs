//! Shows how to render simple primitive shapes with a single color.

use bevy::app::App;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_baseview_plugin::{attach_to, AppProxy, DefaultBaseviewPlugins, ParentWin};

fn main() {
    bevy_baseview_standalone_helper::run_app(create_app);
}

fn create_app(parent_win: ParentWin, width: f64, height: f64) -> AppProxy {
    // Initailize the app.
    let mut app = App::new();
    let window_open_options = baseview::WindowOpenOptions {
        title: "Shapes example".to_string(),
        size: baseview::Size::new(width, height),
        scale: baseview::WindowScalePolicy::SystemScaleFactor,
    };
    let proxy = attach_to(&mut app, &window_open_options, parent_win);
    app.add_plugins(DefaultBaseviewPlugins)
        .add_plugin(bevy::log::LogPlugin::default())
        .add_startup_system(setup)
        .run();
    proxy
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    // Circle
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}
